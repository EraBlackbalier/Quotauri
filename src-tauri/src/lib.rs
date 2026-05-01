// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod products;
mod quotes;
mod settings;
mod templates;

use tauri::Manager;

use products::{
    create_product, delete_product, get_product, list_products, update_product,
};

use quotes::{
    create_quote, get_quote, list_quotes, export_quote_html, export_quote_pdf, render_quote_html,
};

use templates::{
    create_template, delete_template, get_template, list_templates, render_template_preview,
    update_template,
    get_template_translation, upsert_template_translation,
};

use settings::{get_setting, set_setting};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            let pool = tauri::async_runtime::block_on(async move { db::init(&handle).await })
                .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

            app.manage(db::Db(pool));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_products,
            get_product,
            create_product,
            update_product,
            delete_product,
            list_quotes,
            get_quote,
            create_quote,
            render_quote_html,
            export_quote_html,
            export_quote_pdf,
            list_templates,
            get_template,
            create_template,
            update_template,
            delete_template,
            render_template_preview,
            get_setting,
            set_setting,
            get_template_translation,
            upsert_template_translation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
