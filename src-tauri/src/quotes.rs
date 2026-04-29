use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Quote {
    pub id: i64,
    pub quote_number: Option<String>,
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub notes: Option<String>,
    pub subtotal_cents: i64,
    pub tax_rate_bps: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub template_id: Option<i64>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct QuoteItem {
    pub id: i64,
    pub quote_id: i64,
    pub product_id: Option<i64>,
    pub position: i64,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i64,
    pub unit_price_cents: i64,
    pub line_total_cents: i64,
    pub currency: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct QuoteWithItems {
    pub quote: Quote,
    pub items: Vec<QuoteItem>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct QuoteSummary {
    pub id: i64,
    pub quote_number: Option<String>,
    pub customer_name: Option<String>,
    pub total_cents: i64,
    pub status: String,
    pub created_at: String,
    pub currency: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuoteItemInput {
    pub product_id: Option<i64>,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i64,
    pub unit_price_cents: i64,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateQuoteInput {
    pub quote_number: Option<String>,
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub notes: Option<String>,
    pub tax_rate_bps: i64,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub template_id: Option<i64>,
    pub items: Vec<QuoteItemInput>,
}

fn calc_totals(items: &[QuoteItemInput], tax_rate_bps: i64) -> Result<(i64, i64, i64, Vec<i64>), String> {
    if tax_rate_bps < 0 {
        return Err("tax_rate_bps must be >= 0".to_string());
    }

    let mut line_totals: Vec<i64> = Vec::with_capacity(items.len());
    let mut subtotal: i128 = 0;

    for item in items {
        if item.quantity <= 0 {
            return Err("quantity must be > 0".to_string());
        }
        if item.unit_price_cents < 0 {
            return Err("unit_price_cents must be >= 0".to_string());
        }

        let line_total = (item.quantity as i128)
            .checked_mul(item.unit_price_cents as i128)
            .ok_or_else(|| "line total overflow".to_string())?;

        subtotal = subtotal
            .checked_add(line_total)
            .ok_or_else(|| "subtotal overflow".to_string())?;

        line_totals.push(
            i64::try_from(line_total).map_err(|_| "line total out of range".to_string())?,
        );
    }

    let subtotal_i64 = i64::try_from(subtotal).map_err(|_| "subtotal out of range".to_string())?;

    let tax = ((subtotal * (tax_rate_bps as i128)) + 5000) / 10000;
    let tax_i64 = i64::try_from(tax).map_err(|_| "tax out of range".to_string())?;

    let total = subtotal
        .checked_add(tax)
        .ok_or_else(|| "total overflow".to_string())?;
    let total_i64 = i64::try_from(total).map_err(|_| "total out of range".to_string())?;

    Ok((subtotal_i64, tax_i64, total_i64, line_totals))
}

async fn fetch_quote(pool: &SqlitePool, id: i64) -> Result<Quote, String> {
    sqlx::query_as::<_, Quote>(
        r#"
SELECT id,
       quote_number,
       customer_name,
       customer_email,
       notes,
       subtotal_cents,
       tax_rate_bps,
       tax_cents,
       total_cents,
       status,
       created_at,
       updated_at,
       template_id,
       currency
FROM quotes
WHERE id = ?1
"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}

async fn fetch_quote_items(pool: &SqlitePool, quote_id: i64) -> Result<Vec<QuoteItem>, String> {
    sqlx::query_as::<_, QuoteItem>(
        r#"
SELECT id,
       quote_id,
       product_id,
       position,
       sku,
       name,
       description,
       quantity,
       unit_price_cents,
       line_total_cents,
       currency,
       created_at,
       updated_at
FROM quote_items
WHERE quote_id = ?1
ORDER BY position ASC, id ASC
"#,
    )
    .bind(quote_id)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_quotes(db: State<'_, Db>, search: Option<String>) -> Result<Vec<QuoteSummary>, String> {
    let pool = &db.0;

    if let Some(search) = search {
        let pattern = format!("%{}%", search);
        sqlx::query_as::<_, QuoteSummary>(
            r#"
SELECT id,
       quote_number,
       customer_name,
       total_cents,
       status,
       created_at,
       currency
FROM quotes
WHERE quote_number LIKE ?1 OR customer_name LIKE ?1
ORDER BY created_at DESC
LIMIT 200
"#,
        )
        .bind(pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    } else {
        sqlx::query_as::<_, QuoteSummary>(
            r#"
SELECT id,
       quote_number,
       customer_name,
       total_cents,
       status,
       created_at,
       currency
FROM quotes
ORDER BY created_at DESC
LIMIT 200
"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_quote(db: State<'_, Db>, id: i64) -> Result<QuoteWithItems, String> {
    let pool = &db.0;

    let quote = fetch_quote(pool, id).await?;
    let items = fetch_quote_items(pool, id).await?;

    Ok(QuoteWithItems { quote, items })
}

#[tauri::command]
pub async fn create_quote(db: State<'_, Db>, input: CreateQuoteInput) -> Result<QuoteWithItems, String> {
    if input.items.is_empty() {
        return Err("quote must have at least 1 item".to_string());
    }

    let pool = &db.0;

    let currency = input.currency.unwrap_or_else(|| "MXN".to_string());
    let status = input.status.unwrap_or_else(|| "draft".to_string());

    let (subtotal_cents, tax_cents, total_cents, line_totals) =
        calc_totals(&input.items, input.tax_rate_bps)?;

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let res = sqlx::query(
        r#"
INSERT INTO quotes (
  quote_number,
  customer_name,
  customer_email,
  notes,
  subtotal_cents,
  tax_rate_bps,
  tax_cents,
  total_cents,
  status,
  created_at,
  updated_at,
  template_id,
  currency
)
VALUES (
  ?1, ?2, ?3, ?4,
  ?5, ?6, ?7, ?8,
  ?9,
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
  ?10,
  ?11
)
"#,
    )
    .bind(input.quote_number.clone())
    .bind(input.customer_name.clone())
    .bind(input.customer_email.clone())
    .bind(input.notes.clone())
    .bind(subtotal_cents)
    .bind(input.tax_rate_bps)
    .bind(tax_cents)
    .bind(total_cents)
    .bind(status)
    .bind(input.template_id)
    .bind(currency.clone())
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let quote_id = res.last_insert_rowid();

    let quote_number = input
        .quote_number
        .clone()
        .unwrap_or_else(|| format!("Q-{:06}", quote_id));

    sqlx::query(
        r#"
UPDATE quotes
SET quote_number = ?1,
    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
WHERE id = ?2
"#,
    )
    .bind(quote_number)
    .bind(quote_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for (idx, item) in input.items.iter().enumerate() {
        let line_total = line_totals[idx];
        let item_currency = item
            .currency
            .clone()
            .unwrap_or_else(|| currency.clone());

        sqlx::query(
            r#"
INSERT INTO quote_items (
  quote_id,
  product_id,
  position,
  sku,
  name,
  description,
  quantity,
  unit_price_cents,
  line_total_cents,
  currency,
  created_at,
  updated_at
)
VALUES (
  ?1, ?2, ?3, ?4, ?5, ?6,
  ?7, ?8, ?9, ?10,
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
)
"#,
        )
        .bind(quote_id)
        .bind(item.product_id)
        .bind(idx as i64)
        .bind(item.sku.clone())
        .bind(item.name.clone())
        .bind(item.description.clone())
        .bind(item.quantity)
        .bind(item.unit_price_cents)
        .bind(line_total)
        .bind(item_currency)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    let quote = fetch_quote(pool, quote_id).await?;
    let items = fetch_quote_items(pool, quote_id).await?;

    Ok(QuoteWithItems { quote, items })
}
