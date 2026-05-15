use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, field: &str, message: &str) {
        self.is_valid = false;
        self.errors.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        });
    }

    pub fn to_result(self) -> Result<(), String> {
        if self.is_valid {
            Ok(())
        } else {
            let messages: Vec<String> = self
                .errors
                .iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect();
            Err(messages.join("; "))
        }
    }
}

pub fn validate_product_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Product name cannot be empty".to_string());
    }
    if trimmed.len() > 255 {
        return Err("Product name cannot exceed 255 characters".to_string());
    }
    Ok(())
}

pub fn validate_sku(sku: &str) -> Result<(), String> {
    let trimmed = sku.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    if trimmed.len() > 100 {
        return Err("SKU cannot exceed 100 characters".to_string());
    }
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("SKU can only contain alphanumeric characters, dashes, and underscores".to_string());
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), String> {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    if !trimmed.contains('@') || !trimmed.contains('.') {
        return Err("Invalid email format".to_string());
    }
    if trimmed.len() > 254 {
        return Err("Email cannot exceed 254 characters".to_string());
    }
    Ok(())
}

pub fn validate_currency(currency: &str) -> Result<(), String> {
    let trimmed = currency.trim();
    if trimmed.is_empty() {
        return Err("Currency cannot be empty".to_string());
    }
    if trimmed.len() != 3 {
        return Err("Currency code must be 3 characters (e.g., USD, MXN)".to_string());
    }
    if !trimmed.chars().all(|c| c.is_ascii_uppercase()) {
        return Err("Currency code must be uppercase".to_string());
    }
    Ok(())
}

pub fn validate_price(price_cents: i64) -> Result<(), String> {
    if price_cents < 0 {
        return Err("Price cannot be negative".to_string());
    }
    if price_cents > 9_999_999_999 {
        return Err("Price exceeds maximum value".to_string());
    }
    Ok(())
}

pub fn validate_tax_rate(tax_rate_bps: i64) -> Result<(), String> {
    if tax_rate_bps < 0 {
        return Err("Tax rate cannot be negative".to_string());
    }
    if tax_rate_bps > 100_000 {
        return Err("Tax rate cannot exceed 1000% (100000 bps)".to_string());
    }
    Ok(())
}

pub fn validate_quantity(quantity: i64) -> Result<(), String> {
    if quantity <= 0 {
        return Err("Quantity must be greater than 0".to_string());
    }
    if quantity > 1_000_000 {
        return Err("Quantity exceeds maximum value".to_string());
    }
    Ok(())
}

pub fn validate_hex_color(color: &str) -> Result<(), String> {
    let trimmed = color.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    if !trimmed.starts_with('#') {
        return Err("Color must start with #".to_string());
    }
    if trimmed.len() != 7 {
        return Err("Color must be in format #RRGGBB".to_string());
    }
    if !trimmed[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Color contains invalid hexadecimal characters".to_string());
    }
    Ok(())
}

pub fn format_currency(cents: i64, currency: &str) -> String {
    let amount = (cents as f64) / 100.0;
    format!("{} {:.2}", currency, amount)
}

pub fn parse_currency_to_cents(amount: f64) -> i64 {
    (amount * 100.0).round() as i64
}

pub fn sanitize_html(html: &str) -> String {
    html.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn generate_quote_number(quote_id: i64) -> String {
    format!("Q-{:06}", quote_id)
}

pub fn calculate_discount_amount(total_cents: i64, discount_percent: f64) -> i64 {
    ((total_cents as f64) * (discount_percent / 100.0)).round() as i64
}

pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    groups
}
