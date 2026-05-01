use crate::db::Db;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, SqlitePool};
use tauri::State;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Template {
    pub id: i64,
    pub name: String,
    pub logo_path: Option<String>,
    pub logo_data_url: Option<String>,
    pub accent_color: Option<String>,
    pub header_html: Option<String>,
    pub body_html: Option<String>,
    pub footer_html: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TemplateTranslation {
    pub id: i64,
    pub template_id: i64,
    pub lang_code: String,
    pub name: Option<String>,
    pub header_html: Option<String>,
    pub body_html: Option<String>,
    pub footer_html: Option<String>,
    pub variables_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateUpsertInput {
    pub name: String,
    pub logo_path: Option<String>,
    pub logo_data_url: Option<String>,
    pub accent_color: Option<String>,
    pub header_html: Option<String>,
    pub body_html: Option<String>,
    pub footer_html: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateTranslationUpsertInput {
    pub template_id: i64,
    pub lang_code: String,
    pub name: Option<String>,
    pub header_html: Option<String>,
    pub body_html: Option<String>,
    pub footer_html: Option<String>,
    pub variables_json: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenderTemplateInput {
    pub name: Option<String>,
    pub logo_data_url: Option<String>,
    pub accent_color: Option<String>,
    pub header_html: Option<String>,
    pub body_html: Option<String>,
    pub footer_html: Option<String>,
    pub variables: Value,
}

async fn fetch_template(pool: &SqlitePool, id: i64) -> Result<Template, String> {
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

pub(crate) fn render_html(input: RenderTemplateInput) -> Result<String, String> {
    let accent = input.accent_color.unwrap_or_else(|| "#396cd8".to_string());
    let logo_data_url = input.logo_data_url.unwrap_or_default();
    let title = input.name.unwrap_or_else(|| "Quotauri".to_string());

    let mut variables = input.variables;
    if variables.is_null() {
        variables = Value::Object(serde_json::Map::new());
    }
    if let Value::Object(map) = &mut variables {
        map.insert("accent_color".to_string(), Value::String(accent.clone()));
        map.insert("logo_data_url".to_string(), Value::String(logo_data_url));
        map.insert("title".to_string(), Value::String(title.clone()));
    }

    let mut hb = Handlebars::new();
    hb.register_escape_fn(handlebars::no_escape);

    let header_raw = input.header_html.unwrap_or_default();
    let body_raw = input.body_html.unwrap_or_default();
    let footer_raw = input.footer_html.unwrap_or_default();

    let header = hb
        .render_template(&header_raw, &variables)
        .map_err(|e| e.to_string())?;
    let body = hb
        .render_template(&body_raw, &variables)
        .map_err(|e| e.to_string())?;
    let footer = hb
        .render_template(&footer_raw, &variables)
        .map_err(|e| e.to_string())?;

    Ok(format!(
        r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <style>
    @page {{ size: A4; margin: 0; }}
    * {{ box-sizing: border-box; }}
    body {{ margin: 0; padding: 0; font-family: 'Segoe UI', Inter, Arial, sans-serif; color: #1a1a2e; background: #fff; -webkit-print-color-adjust: exact; print-color-adjust: exact; }}
    .page {{ width: 210mm; min-height: 297mm; margin: 0 auto; position: relative; overflow: hidden; }}
    @media screen {{ .page {{ border: 1px solid #ddd; margin: 20px auto; box-shadow: 0 2px 20px rgba(0,0,0,0.08); }} }}
  </style>
</head>
<body>
  <div class="page">
    {header}
    {body}
    {footer}
  </div>
</body>
</html>"#,
        title = html_escape(&title),
        header = header,
        body = body,
        footer = footer
    ))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[tauri::command]
pub async fn list_templates(db: State<'_, Db>, search: Option<String>) -> Result<Vec<Template>, String> {
    let pool = &db.0;

    if let Some(search) = search {
        let pattern = format!("%{}%", search);
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
WHERE name LIKE ?1
ORDER BY updated_at DESC
LIMIT 200
"#,
        )
        .bind(pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    } else {
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
LIMIT 200
"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn get_template(db: State<'_, Db>, id: i64) -> Result<Template, String> {
    let pool = &db.0;
    fetch_template(pool, id).await
}

#[tauri::command]
pub async fn create_template(db: State<'_, Db>, input: TemplateUpsertInput) -> Result<Template, String> {
    let pool = &db.0;

    let res = sqlx::query(
        r#"
INSERT INTO templates (
  name,
  logo_path,
  logo_data_url,
  accent_color,
  header_html,
  body_html,
  footer_html,
  created_at,
  updated_at
)
VALUES (
  ?1, ?2, ?3, ?4, ?5, ?6, ?7,
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
)
"#,
    )
    .bind(input.name)
    .bind(input.logo_path)
    .bind(input.logo_data_url)
    .bind(input.accent_color)
    .bind(input.header_html)
    .bind(input.body_html)
    .bind(input.footer_html)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    let id = res.last_insert_rowid();
    fetch_template(pool, id).await
}

#[tauri::command]
pub async fn update_template(
    db: State<'_, Db>,
    id: i64,
    input: TemplateUpsertInput,
) -> Result<Template, String> {
    let pool = &db.0;

    sqlx::query(
        r#"
UPDATE templates
SET name = ?1,
    logo_path = ?2,
    logo_data_url = ?3,
    accent_color = ?4,
    header_html = ?5,
    body_html = ?6,
    footer_html = ?7,
    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
WHERE id = ?8
"#,
    )
    .bind(input.name)
    .bind(input.logo_path)
    .bind(input.logo_data_url)
    .bind(input.accent_color)
    .bind(input.header_html)
    .bind(input.body_html)
    .bind(input.footer_html)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    fetch_template(pool, id).await
}

#[tauri::command]
pub async fn delete_template(db: State<'_, Db>, id: i64) -> Result<(), String> {
    let pool = &db.0;

    sqlx::query(
        r#"
DELETE FROM templates
WHERE id = ?1
"#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_template_translation(
    db: State<'_, Db>,
    template_id: i64,
    lang_code: String,
) -> Result<Option<TemplateTranslation>, String> {
    let pool = &db.0;
    fetch_template_translation(pool, template_id, &lang_code).await
}

#[tauri::command]
pub async fn upsert_template_translation(
    db: State<'_, Db>,
    input: TemplateTranslationUpsertInput,
) -> Result<TemplateTranslation, String> {
    let pool = &db.0;

    let code = input.lang_code.trim().to_lowercase();
    if code.is_empty() {
        return Err("lang_code is required".to_string());
    }

    sqlx::query(
        r#"
INSERT INTO template_translations (
  template_id,
  lang_code,
  name,
  header_html,
  body_html,
  footer_html,
  variables_json,
  created_at,
  updated_at
)
VALUES (
  ?1, ?2, ?3, ?4, ?5, ?6, ?7,
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
  strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
)
ON CONFLICT(template_id, lang_code)
DO UPDATE SET
  name = excluded.name,
  header_html = excluded.header_html,
  body_html = excluded.body_html,
  footer_html = excluded.footer_html,
  variables_json = excluded.variables_json,
  updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now');
"#,
    )
    .bind(input.template_id)
    .bind(&code)
    .bind(input.name)
    .bind(input.header_html)
    .bind(input.body_html)
    .bind(input.footer_html)
    .bind(input.variables_json)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    let saved = fetch_template_translation(pool, input.template_id, &code).await?;
    saved.ok_or_else(|| "failed to fetch template translation".to_string())
}

#[tauri::command]
pub fn render_template_preview(input: RenderTemplateInput) -> Result<String, String> {
    render_html(input)
}
