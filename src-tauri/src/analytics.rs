use crate::db::Db;
use serde::Serialize;
use sqlx::FromRow;
use tauri::State;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SalesStats {
    pub total_quotes: i64,
    pub total_revenue_cents: i64,
    pub total_tax_cents: i64,
    pub average_quote_value_cents: i64,
    pub quotes_this_month: i64,
    pub revenue_this_month_cents: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ProductStats {
    pub product_id: i64,
    pub product_name: String,
    pub times_used: i64,
    pub total_quantity_sold: i64,
    pub total_revenue_cents: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CustomerStats {
    pub customer_name: Option<String>,
    pub total_quotes: i64,
    pub total_spent_cents: i64,
    pub last_quote_date: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MonthlyRevenue {
    pub month: String,
    pub total_cents: i64,
    pub quote_count: i64,
}

#[tauri::command]
pub async fn get_sales_stats(db: State<'_, Db>) -> Result<SalesStats, String> {
    let pool = &db.0;

    let current_month = chrono::Local::now()
        .format("%Y-%m")
        .to_string();

    let row = sqlx::query_as::<_, (i64, i64, i64, i64)>(
        r#"
SELECT 
  COUNT(*) as total_quotes,
  COALESCE(SUM(total_cents), 0) as total_revenue_cents,
  COALESCE(SUM(tax_cents), 0) as total_tax_cents,
  COALESCE(AVG(total_cents), 0) as average_value_cents
FROM quotes
WHERE status != 'draft'
"#,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let monthly_row = sqlx::query_as::<_, (i64, i64)>(
        r#"
SELECT 
  COUNT(*) as count,
  COALESCE(SUM(total_cents), 0) as total
FROM quotes
WHERE status != 'draft'
  AND strftime('%Y-%m', created_at) = ?1
"#,
    )
    .bind(&current_month)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(SalesStats {
        total_quotes: row.0,
        total_revenue_cents: row.1,
        total_tax_cents: row.2,
        average_quote_value_cents: row.3,
        quotes_this_month: monthly_row.0,
        revenue_this_month_cents: monthly_row.1,
    })
}

#[tauri::command]
pub async fn get_top_products(db: State<'_, Db>, limit: Option<i64>) -> Result<Vec<ProductStats>, String> {
    let pool = &db.0;
    let lim = limit.unwrap_or(10).max(1).min(100);

    sqlx::query_as::<_, ProductStats>(
        r#"
SELECT 
  p.id as product_id,
  p.name as product_name,
  COUNT(qi.id) as times_used,
  COALESCE(SUM(qi.quantity), 0) as total_quantity_sold,
  COALESCE(SUM(qi.line_total_cents), 0) as total_revenue_cents
FROM quote_items qi
INNER JOIN products p ON p.id = qi.product_id
GROUP BY p.id, p.name
ORDER BY total_revenue_cents DESC
LIMIT ?1
"#,
    )
    .bind(lim)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_top_customers(db: State<'_, Db>, limit: Option<i64>) -> Result<Vec<CustomerStats>, String> {
    let pool = &db.0;
    let lim = limit.unwrap_or(10).max(1).min(100);

    sqlx::query_as::<_, CustomerStats>(
        r#"
SELECT 
  customer_name,
  COUNT(*) as total_quotes,
  COALESCE(SUM(total_cents), 0) as total_spent_cents,
  MAX(created_at) as last_quote_date
FROM quotes
WHERE status != 'draft'
GROUP BY customer_name
ORDER BY total_spent_cents DESC
LIMIT ?1
"#,
    )
    .bind(lim)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_monthly_revenue(db: State<'_, Db>, months: Option<i64>) -> Result<Vec<MonthlyRevenue>, String> {
    let pool = &db.0;
    let m = months.unwrap_or(12).max(1).min(60);

    sqlx::query_as::<_, MonthlyRevenue>(
        r#"
SELECT 
  strftime('%Y-%m', created_at) as month,
  COALESCE(SUM(total_cents), 0) as total_cents,
  COUNT(*) as quote_count
FROM quotes
WHERE status != 'draft'
  AND created_at >= datetime('now', '-' || ?1 || ' months')
GROUP BY strftime('%Y-%m', created_at)
ORDER BY month DESC
"#,
    )
    .bind(m)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_quote_status_summary(db: State<'_, Db>) -> Result<Vec<(String, i64)>, String> {
    let pool = &db.0;

    sqlx::query_as::<_, (String, i64)>(
        r#"
SELECT status, COUNT(*) as count
FROM quotes
GROUP BY status
ORDER BY count DESC
"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}
