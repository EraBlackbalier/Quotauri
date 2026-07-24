use crate::db::Db;
use crate::templates::{render_html, RenderTemplateInput, Template, TemplateTranslation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, SqlitePool};
use tauri::State;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn format_money(cents: i64) -> String {
    let v = (cents as f64) / 100.0;
    format!("{:.2}", v)
}

async fn fetch_template_by_id(pool: &SqlitePool, id: i64) -> Result<Template, String> {
    sqlx::query_as::<_, Template>(
        r#"
SELECT id,
       name,
       logo_path,
       logo_data_url,
       accent_color,
       header_html,
       body_html,
       footer_html,
       created_at,
       updated_at
FROM templates
WHERE id = ?1
"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}

async fn fetch_latest_template(pool: &SqlitePool) -> Result<Template, String> {
    sqlx::query_as::<_, Template>(
        r#"
SELECT id,
       name,
       logo_path,
       logo_data_url,
       accent_color,
       header_html,
       body_html,
       footer_html,
       created_at,
       updated_at
FROM templates
ORDER BY updated_at DESC
LIMIT 1
"#,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}

async fn fetch_template_translation(
    pool: &SqlitePool,
    template_id: i64,
    lang_code: &str,
) -> Result<Option<TemplateTranslation>, String> {
    sqlx::query_as::<_, TemplateTranslation>(
        r#"
SELECT id,
       template_id,
       lang_code,
       name,
       header_html,
       body_html,
       footer_html,
       variables_json,
       created_at,
       updated_at
FROM template_translations
WHERE template_id = ?1
  AND lang_code = ?2
"#,
    )
    .bind(template_id)
    .bind(lang_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())
}

fn file_url_from_path(p: &std::path::Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/");
    let s = s.replace(' ', "%20");
    format!("file:///{}", s)
}

fn find_edge_exe() -> Option<std::path::PathBuf> {
    let candidates = [
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    ];

    for c in candidates {
        let p = std::path::PathBuf::from(c);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

async fn render_quote_html_internal(
    pool: &SqlitePool,
    quote_id: i64,
    lang_code: &str,
) -> Result<String, String> {
    let quote = fetch_quote(pool, quote_id).await?;
    let items = fetch_quote_items(pool, quote_id).await?;

    let template = if let Some(tid) = quote.template_id {
        fetch_template_by_id(pool, tid).await?
    } else {
        fetch_latest_template(pool).await?
    };

    let translation = fetch_template_translation(pool, template.id, lang_code).await?;

    let custom_vars: Value = if let Some(t) = &translation {
        if let Some(raw) = &t.variables_json {
            if raw.trim().is_empty() {
                json!({})
            } else {
                serde_json::from_str(raw).map_err(|e| format!("invalid variables_json: {e}"))?
            }
        } else {
            json!({})
        }
    } else {
        json!({})
    };

    let quote_number_display = quote
        .quote_number
        .clone()
        .unwrap_or_else(|| format!("Q-{:06}", quote.id));

    let item_vars: Vec<Value> = items
        .iter()
        .map(|it| {
            json!({
              "sku": it.sku,
              "name": it.name,
              "description": it.description,
              "quantity": it.quantity,
              "unit_price_cents": it.unit_price_cents,
              "unit_price": format_money(it.unit_price_cents),
              "line_total_cents": it.line_total_cents,
              "line_total": format_money(it.line_total_cents),
              "currency": it.currency,
            })
        })
        .collect();

    let mut vars = match custom_vars {
        Value::Object(m) => Value::Object(m),
        _ => json!({}),
    };

    if let Value::Object(map) = &mut vars {
        map.insert(
            "quote".to_string(),
            json!({
              "id": quote.id,
              "quote_number": quote.quote_number,
              "quote_number_display": quote_number_display,
              "customer_name": quote.customer_name,
              "customer_email": quote.customer_email,
              "notes": quote.notes,
              "status": quote.status,
              "created_at": quote.created_at,
              "updated_at": quote.updated_at,
              "subtotal_cents": quote.subtotal_cents,
              "tax_rate_bps": quote.tax_rate_bps,
              "tax_cents": quote.tax_cents,
              "total_cents": quote.total_cents,
              "subtotal": format_money(quote.subtotal_cents),
              "tax": format_money(quote.tax_cents),
              "total": format_money(quote.total_cents),
              "currency": quote.currency,
              "template_id": quote.template_id,
              "lang_code": lang_code,
            }),
        );
        map.insert("items".to_string(), Value::Array(item_vars));
    }

    let name = translation
        .as_ref()
        .and_then(|t| t.name.clone())
        .unwrap_or_else(|| template.name.clone());

    let header_html = translation
        .as_ref()
        .and_then(|t| t.header_html.clone())
        .or_else(|| template.header_html.clone());
    let body_html = translation
        .as_ref()
        .and_then(|t| t.body_html.clone())
        .or_else(|| template.body_html.clone());
    let footer_html = translation
        .as_ref()
        .and_then(|t| t.footer_html.clone())
        .or_else(|| template.footer_html.clone());

    let input = RenderTemplateInput {
        name: Some(name),
        logo_data_url: template.logo_data_url.clone(),
        accent_color: template.accent_color.clone(),
        header_html,
        body_html,
        footer_html,
        variables: vars,
    };

    render_html(input)
}

#[tauri::command]
pub async fn render_quote_html(
    db: State<'_, Db>,
    quote_id: i64,
    lang_code: Option<String>,
) -> Result<String, String> {
    let pool = &db.0;
    let lang = lang_code.unwrap_or_else(|| "es".to_string());
    render_quote_html_internal(pool, quote_id, lang.trim()).await
}

#[tauri::command]
pub async fn export_quote_html(
    db: State<'_, Db>,
    quote_id: i64,
    lang_code: Option<String>,
    output_path: String,
) -> Result<(), String> {
    let pool = &db.0;
    let lang = lang_code.unwrap_or_else(|| "es".to_string());
    let html = render_quote_html_internal(pool, quote_id, lang.trim()).await?;
    std::fs::write(&output_path, html).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_quote_pdf(
    db: State<'_, Db>,
    quote_id: i64,
    lang_code: Option<String>,
    output_path: String,
) -> Result<(), String> {
    let pool = &db.0;
    let lang = lang_code.unwrap_or_else(|| "es".to_string());
    let html = render_quote_html_internal(pool, quote_id, lang.trim()).await?;

    let mut tmp = std::env::temp_dir();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();
    tmp.push(format!("vaestra_quote_{quote_id}_{now}.html"));
    std::fs::write(&tmp, html).map_err(|e| e.to_string())?;

    let edge = find_edge_exe().ok_or_else(|| {
        "Microsoft Edge not found. Install Edge or export HTML and print to PDF manually.".to_string()
    })?;

    let out = std::path::PathBuf::from(&output_path);
    let url = file_url_from_path(&tmp);

    let status = tauri::async_runtime::spawn_blocking(move || {
        std::process::Command::new(edge)
            .arg("--headless")
            .arg("--disable-gpu")
            .arg(format!("--print-to-pdf={}", out.to_string_lossy()))
            .arg("--no-pdf-header-footer")
            .arg("--print-to-pdf-no-header")
            .arg("--run-all-compositor-stages-before-draw")
            .arg(url)
            .status()
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    let _ = std::fs::remove_file(&tmp);

    if !status.success() {
        return Err(format!("PDF generation failed with status: {status}"));
    }

    Ok(())
}

#[tauri::command]
pub async fn export_quote_docx(
    db: State<'_, Db>,
    quote_id: i64,
    lang_code: Option<String>,
    output_path: String,
) -> Result<(), String> {
    let pool = &db.0;
    let quote = fetch_quote(pool, quote_id).await?;
    let items = fetch_quote_items(pool, quote_id).await?;
    let template = if let Some(template_id) = quote.template_id {
        fetch_template_by_id(pool, template_id).await?
    } else {
        fetch_latest_template(pool).await?
    };
    let lang = lang_code.unwrap_or_else(|| "es".to_string());
    crate::word_export::write_quote_docx(
        std::path::Path::new(&output_path),
        &quote,
        &items,
        &template,
        lang.trim(),
    )
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
