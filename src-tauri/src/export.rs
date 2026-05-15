use serde::Serialize;
use sqlx::Row;
use tauri::Manager;

#[derive(Serialize)]
pub struct DatabaseInfo {
    path: String,
    exists: bool,
    size_bytes: u64,
}

#[tauri::command]
pub async fn export_data_json(app: tauri::AppHandle) -> Result<String, String> {
    let db = app.state::<crate::db::Db>();

    let products = sqlx::query(
        "SELECT id, sku, name, description, unit_price_cents, currency, created_at, updated_at FROM products",
    )
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    let quotes = sqlx::query(
        "SELECT id, quote_number, customer_name, customer_email, notes, subtotal_cents, tax_cents, total_cents, status, created_at, updated_at FROM quotes",
    )
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    let payload = serde_json::json!({
        "products": products
            .into_iter()
            .map(|r| serde_json::json!({
                "id": r.get::<i64, _>("id"),
                "sku": r.get::<Option<String>, _>("sku"),
                "name": r.get::<String, _>("name"),
                "description": r.get::<Option<String>, _>("description"),
                "unit_price_cents": r.get::<i64, _>("unit_price_cents"),
                "currency": r.get::<String, _>("currency"),
                "created_at": r.get::<String, _>("created_at"),
                "updated_at": r.get::<String, _>("updated_at")
            }))
            .collect::<Vec<_>>(),
        "quotes": quotes
            .into_iter()
            .map(|r| serde_json::json!({
                "id": r.get::<i64, _>("id"),
                "quote_number": r.get::<Option<String>, _>("quote_number"),
                "customer_name": r.get::<Option<String>, _>("customer_name"),
                "customer_email": r.get::<Option<String>, _>("customer_email"),
                "notes": r.get::<Option<String>, _>("notes"),
                "subtotal_cents": r.get::<i64, _>("subtotal_cents"),
                "tax_cents": r.get::<i64, _>("tax_cents"),
                "total_cents": r.get::<i64, _>("total_cents"),
                "status": r.get::<String, _>("status"),
                "created_at": r.get::<String, _>("created_at"),
                "updated_at": r.get::<String, _>("updated_at")
            }))
            .collect::<Vec<_>>()
    });

    serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_quotes_csv(app: tauri::AppHandle) -> Result<String, String> {
    let db = app.state::<crate::db::Db>();

    let rows = sqlx::query(
        "SELECT id, quote_number, customer_name, customer_email, subtotal_cents, tax_cents, total_cents, status, created_at FROM quotes ORDER BY id DESC",
    )
    .fetch_all(&db.0)
    .await
    .map_err(|e| e.to_string())?;

    let mut csv = String::from(
        "id,quote_number,customer_name,customer_email,subtotal_cents,tax_cents,total_cents,status,created_at\n",
    );

    for row in rows {
        let id: i64 = row.get("id");
        let quote_number: Option<String> = row.get("quote_number");
        let customer_name: Option<String> = row.get("customer_name");
        let customer_email: Option<String> = row.get("customer_email");
        let subtotal_cents: i64 = row.get("subtotal_cents");
        let tax_cents: i64 = row.get("tax_cents");
        let total_cents: i64 = row.get("total_cents");
        let status: String = row.get("status");
        let created_at: String = row.get("created_at");

        let line = format!(
            "{},{},{},{},{},{},{},{},{}\n",
            id,
            escape_csv_field(quote_number.as_deref().unwrap_or("")),
            escape_csv_field(customer_name.as_deref().unwrap_or("")),
            escape_csv_field(customer_email.as_deref().unwrap_or("")),
            subtotal_cents,
            tax_cents,
            total_cents,
            escape_csv_field(&status),
            escape_csv_field(&created_at)
        );

        csv.push_str(&line);
    }

    Ok(csv)
}

#[tauri::command]
pub fn get_database_info(app: tauri::AppHandle) -> Result<DatabaseInfo, String> {
    let path = crate::db::db_path(&app).map_err(|e| e.to_string())?;
    let metadata = std::fs::metadata(&path).ok();

    Ok(DatabaseInfo {
        path: path.display().to_string(),
        exists: metadata.is_some(),
        size_bytes: metadata.map(|m| m.len()).unwrap_or(0),
    })
}

fn escape_csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
