use anyhow::Context;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::path::PathBuf;
use tauri::Manager;

pub struct Db(pub SqlitePool);

pub fn db_path(handle: &tauri::AppHandle) -> anyhow::Result<PathBuf> {
    let mut dir = handle.path().app_data_dir()?;
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("failed to create app data dir: {}", dir.display()))?;

    dir.push("quotauri.sqlite3");
    Ok(dir)
}

pub async fn init(handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let db_path = db_path(handle)?;

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .with_context(|| format!("failed to open sqlite db at {}", db_path.display()))?;

    create_tables(&pool).await?;

    Ok(pool)
}

async fn create_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS products (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sku TEXT,
  name TEXT NOT NULL,
  description TEXT,
  unit_price_cents INTEGER NOT NULL DEFAULT 0,
  currency TEXT NOT NULL DEFAULT 'MXN',
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
"#,
    )
    .execute(pool)
    .await
    .context("failed to create products table")?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS templates (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  logo_path TEXT,
  logo_data_url TEXT,
  accent_color TEXT,
  header_html TEXT,
  body_html TEXT,
  footer_html TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
"#,
    )
    .execute(pool)
    .await
    .context("failed to create templates table")?;

    ensure_templates_columns(pool).await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS template_translations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  template_id INTEGER NOT NULL,
  lang_code TEXT NOT NULL,
  name TEXT,
  header_html TEXT,
  body_html TEXT,
  footer_html TEXT,
  variables_json TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  UNIQUE(template_id, lang_code),
  FOREIGN KEY (template_id) REFERENCES templates (id) ON DELETE CASCADE
);
"#,
    )
    .execute(pool)
    .await
    .context("failed to create template_translations table")?;

    ensure_template_translations_columns(pool).await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
"#,
    )
    .execute(pool)
    .await
    .context("failed to create settings table")?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS quotes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  quote_number TEXT,
  customer_name TEXT,
  customer_email TEXT,
  notes TEXT,
  subtotal_cents INTEGER NOT NULL DEFAULT 0,
  tax_cents INTEGER NOT NULL DEFAULT 0,
  total_cents INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'draft',
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  template_id INTEGER,
  FOREIGN KEY (template_id) REFERENCES templates (id)
);
"#,
    )
    .execute(pool)
    .await
    .context("failed to create quotes table")?;

    ensure_quotes_columns(pool).await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS quote_items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  quote_id INTEGER NOT NULL,
  product_id INTEGER,
  position INTEGER NOT NULL DEFAULT 0,
  sku TEXT,
  name TEXT NOT NULL,
  description TEXT,
  quantity INTEGER NOT NULL DEFAULT 1,
  unit_price_cents INTEGER NOT NULL DEFAULT 0,
  line_total_cents INTEGER NOT NULL DEFAULT 0,
  currency TEXT NOT NULL DEFAULT 'MXN',
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  FOREIGN KEY (quote_id) REFERENCES quotes (id) ON DELETE CASCADE,
  FOREIGN KEY (product_id) REFERENCES products (id)
);
"#,
    )
    .execute(pool)
    .await
    .context("failed to create quote_items table")?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quote_items_quote_id ON quote_items(quote_id);")
        .execute(pool)
        .await
        .context("failed to create idx_quote_items_quote_id index")?;

    Ok(())
}

async fn ensure_template_translations_columns(pool: &SqlitePool) -> anyhow::Result<()> {
    use sqlx::Row;

    let rows = sqlx::query("PRAGMA table_info(template_translations);")
        .fetch_all(pool)
        .await
        .context("failed to read template_translations table schema")?;

    let mut has_variables_json = false;
    let mut has_name = false;

    for row in rows {
        let name: String = row.try_get("name")?;
        match name.as_str() {
            "variables_json" => has_variables_json = true,
            "name" => has_name = true,
            _ => {}
        }
    }

    if !has_variables_json {
        sqlx::query("ALTER TABLE template_translations ADD COLUMN variables_json TEXT;")
            .execute(pool)
            .await
            .context("failed to add variables_json column to template_translations")?;
    }

    if !has_name {
        sqlx::query("ALTER TABLE template_translations ADD COLUMN name TEXT;")
            .execute(pool)
            .await
            .context("failed to add name column to template_translations")?;
    }

    Ok(())
}

async fn ensure_templates_columns(pool: &SqlitePool) -> anyhow::Result<()> {
    use sqlx::Row;

    let rows = sqlx::query("PRAGMA table_info(templates);")
        .fetch_all(pool)
        .await
        .context("failed to read templates table schema")?;

    let mut has_logo_data_url = false;
    let mut has_body_html = false;

    for row in rows {
        let name: String = row.try_get("name")?;
        match name.as_str() {
            "logo_data_url" => has_logo_data_url = true,
            "body_html" => has_body_html = true,
            _ => {}
        }
    }

    if !has_logo_data_url {
        sqlx::query("ALTER TABLE templates ADD COLUMN logo_data_url TEXT;")
            .execute(pool)
            .await
            .context("failed to add logo_data_url column to templates")?;
    }

    if !has_body_html {
        sqlx::query("ALTER TABLE templates ADD COLUMN body_html TEXT;")
            .execute(pool)
            .await
            .context("failed to add body_html column to templates")?;
    }

    Ok(())
}

async fn ensure_quotes_columns(pool: &SqlitePool) -> anyhow::Result<()> {
    use sqlx::Row;

    let rows = sqlx::query("PRAGMA table_info(quotes);")
        .fetch_all(pool)
        .await
        .context("failed to read quotes table schema")?;

    let mut has_tax_rate_bps = false;
    let mut has_currency = false;

    for row in rows {
        let name: String = row.try_get("name")?;
        match name.as_str() {
            "tax_rate_bps" => has_tax_rate_bps = true,
            "currency" => has_currency = true,
            _ => {}
        }
    }

    if !has_tax_rate_bps {
        sqlx::query("ALTER TABLE quotes ADD COLUMN tax_rate_bps INTEGER NOT NULL DEFAULT 0;")
            .execute(pool)
            .await
            .context("failed to add tax_rate_bps column to quotes")?;
    }

    if !has_currency {
        sqlx::query("ALTER TABLE quotes ADD COLUMN currency TEXT NOT NULL DEFAULT 'MXN';")
            .execute(pool)
            .await
            .context("failed to add currency column to quotes")?;
    }

    Ok(())
}
