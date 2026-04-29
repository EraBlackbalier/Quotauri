use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub unit_price_cents: i64,
    pub currency: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProductUpsertInput {
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub unit_price_cents: i64,
    pub currency: Option<String>,
}

async fn fetch_product(pool: &SqlitePool, id: i64) -> Result<Product, String> {
    sqlx::query_as::<_, Product>(
        r#"
SELECT id, sku, name, description, unit_price_cents, currency, created_at, updated_at
FROM products
WHERE id = ?1
"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_products(db: State<'_, Db>, search: Option<String>) -> Result<Vec<Product>, String> {
    let pool = &db.0;

    if let Some(search) = search {
        let pattern = format!("%{}%", search);
        sqlx::query_as::<_, Product>(
            r#"
SELECT id, sku, name, description, unit_price_cents, currency, created_at, updated_at
FROM products
WHERE name LIKE ?1 OR sku LIKE ?1
ORDER BY name ASC
LIMIT 500
"#,
        )
        .bind(pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    } else {
        sqlx::query_as::<_, Product>(
            r#"
SELECT id, sku, name, description, unit_price_cents, currency, created_at, updated_at
FROM products
ORDER BY name ASC
LIMIT 500
"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_product(db: State<'_, Db>, id: i64) -> Result<Product, String> {
    let pool = &db.0;

    fetch_product(pool, id).await
}

#[tauri::command]
pub async fn create_product(db: State<'_, Db>, input: ProductUpsertInput) -> Result<Product, String> {
    let pool = &db.0;

    let currency = input.currency.unwrap_or_else(|| "MXN".to_string());

    let result = sqlx::query(
        r#"
INSERT INTO products (sku, name, description, unit_price_cents, currency, created_at, updated_at)
VALUES (?1, ?2, ?3, ?4, ?5, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
"#,
    )
    .bind(input.sku)
    .bind(input.name)
    .bind(input.description)
    .bind(input.unit_price_cents)
    .bind(currency)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();
    fetch_product(pool, id).await
}

#[tauri::command]
pub async fn update_product(
    db: State<'_, Db>,
    id: i64,
    input: ProductUpsertInput,
) -> Result<Product, String> {
    let pool = &db.0;

    let currency = input.currency.unwrap_or_else(|| "MXN".to_string());

    sqlx::query(
        r#"
UPDATE products
SET sku = ?1,
    name = ?2,
    description = ?3,
    unit_price_cents = ?4,
    currency = ?5,
    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
WHERE id = ?6
"#,
    )
    .bind(input.sku)
    .bind(input.name)
    .bind(input.description)
    .bind(input.unit_price_cents)
    .bind(currency)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    fetch_product(pool, id).await
}

#[tauri::command]
pub async fn delete_product(db: State<'_, Db>, id: i64) -> Result<(), String> {
    let pool = &db.0;

    sqlx::query(
        r#"
DELETE FROM products
WHERE id = ?1
"#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}
