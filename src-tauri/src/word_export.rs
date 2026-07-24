use crate::quotes::{Quote, QuoteItem};
use crate::templates::Template;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const BRAND_BLUE: &str = "0B3A82";
const BRAND_RED: &str = "D51F3A";
const INK: &str = "12213A";
const MUTED: &str = "60708A";
const LIGHT_BLUE: &str = "E8F0FC";
const LIGHT_RED: &str = "FDE8EC";
const BORDER: &str = "D5DEEC";

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn money(cents: i64) -> String {
    format!("{:.2}", cents as f64 / 100.0)
}

fn safe_color(value: Option<&str>) -> String {
    let raw = value.unwrap_or("").trim().trim_start_matches('#');
    if raw.len() == 6 && raw.chars().all(|ch| ch.is_ascii_hexdigit()) {
        raw.to_ascii_uppercase()
    } else {
        BRAND_BLUE.to_string()
    }
}

fn run(text: &str, bold: bool, color: &str, size_half_points: u32) -> String {
    format!(
        r#"<w:r><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/><w:b w:val="{}"/><w:color w:val="{}"/><w:sz w:val="{}"/><w:szCs w:val="{}"/></w:rPr><w:t xml:space="preserve">{}</w:t></w:r>"#,
        if bold { "1" } else { "0" },
        color,
        size_half_points,
        size_half_points,
        xml_escape(text)
    )
}

fn paragraph(inner: String, alignment: &str, before: u32, after: u32) -> String {
    format!(
        r#"<w:p><w:pPr><w:jc w:val="{}"/><w:spacing w:before="{}" w:after="{}" w:line="264" w:lineRule="auto"/></w:pPr>{}</w:p>"#,
        alignment, before, after, inner
    )
}

fn text_paragraph(text: &str, bold: bool, color: &str, size: u32, alignment: &str) -> String {
    paragraph(run(text, bold, color, size), alignment, 0, 80)
}

fn cell(inner: String, width: u32, fill: Option<&str>) -> String {
    let shading = fill
        .map(|color| {
            format!(
                r#"<w:shd w:val="clear" w:color="auto" w:fill="{}"/>"#,
                color
            )
        })
        .unwrap_or_default();
    format!(
        r#"<w:tc><w:tcPr><w:tcW w:w="{}" w:type="dxa"/><w:tcMar><w:top w:w="100" w:type="dxa"/><w:start w:w="120" w:type="dxa"/><w:bottom w:w="100" w:type="dxa"/><w:end w:w="120" w:type="dxa"/></w:tcMar><w:vAlign w:val="center"/>{}</w:tcPr>{}</w:tc>"#,
        width, shading, inner
    )
}

fn table(rows: String, widths: &[u32], indent: u32) -> String {
    let grid = widths
        .iter()
        .map(|width| format!(r#"<w:gridCol w:w="{}"/>"#, width))
        .collect::<String>();
    format!(
        r#"<w:tbl><w:tblPr><w:tblW w:w="9360" w:type="dxa"/><w:tblInd w:w="{}" w:type="dxa"/><w:tblLayout w:type="fixed"/><w:tblBorders><w:top w:val="single" w:sz="6" w:color="{}"/><w:left w:val="single" w:sz="6" w:color="{}"/><w:bottom w:val="single" w:sz="6" w:color="{}"/><w:right w:val="single" w:sz="6" w:color="{}"/><w:insideH w:val="single" w:sz="4" w:color="{}"/><w:insideV w:val="single" w:sz="4" w:color="{}"/></w:tblBorders></w:tblPr><w:tblGrid>{}</w:tblGrid>{}</w:tbl>"#,
        indent, BORDER, BORDER, BORDER, BORDER, BORDER, BORDER, grid, rows
    )
}

fn label_value(label: &str, value: &str, fill: &str) -> String {
    let inner = format!(
        "{}{}",
        paragraph(run(label, true, BRAND_RED, 18), "left", 0, 20),
        paragraph(run(value, true, INK, 22), "left", 0, 40)
    );
    cell(inner, 4680, Some(fill))
}

fn build_document(quote: &Quote, items: &[QuoteItem], template: &Template, lang: &str) -> String {
    let is_en = lang.eq_ignore_ascii_case("en");
    let accent = safe_color(template.accent_color.as_deref());
    let folio = quote
        .quote_number
        .clone()
        .unwrap_or_else(|| format!("Q-{:06}", quote.id));
    let date = quote
        .created_at
        .split('T')
        .next()
        .unwrap_or(&quote.created_at);

    let mut body = String::new();
    body.push_str(&paragraph(
        format!(
            "{}{}",
            run("VAESTRA ", true, BRAND_BLUE, 48),
            run(
                if is_en { "QUOTER" } else { "COTIZADOR" },
                true,
                BRAND_RED,
                48
            )
        ),
        "left",
        0,
        20,
    ));
    body.push_str(&paragraph(
        format!(
            "{}{}",
            run(
                if is_en {
                    "COMMERCIAL QUOTATION"
                } else {
                    "COTIZACION COMERCIAL"
                },
                true,
                &accent,
                20,
            ),
            run(&format!("    {}", folio), true, INK, 20)
        ),
        "left",
        0,
        220,
    ));

    let info_rows = format!(
        "<w:tr>{}{}</w:tr><w:tr>{}{}</w:tr>",
        label_value(
            if is_en { "CUSTOMER" } else { "CLIENTE" },
            quote.customer_name.as_deref().unwrap_or("-"),
            LIGHT_BLUE
        ),
        label_value(
            if is_en { "EMAIL" } else { "CORREO" },
            quote.customer_email.as_deref().unwrap_or("-"),
            LIGHT_RED
        ),
        label_value(if is_en { "DATE" } else { "FECHA" }, date, LIGHT_RED),
        label_value(
            if is_en {
                "CURRENCY / TEMPLATE"
            } else {
                "MONEDA / PLANTILLA"
            },
            &format!("{} / {}", quote.currency, template.name),
            LIGHT_BLUE
        )
    );
    body.push_str(&table(info_rows, &[4680, 4680], 120));
    body.push_str(&paragraph(String::new(), "left", 0, 120));

    let headers = if is_en {
        ["DESCRIPTION", "SKU", "QTY.", "UNIT PRICE", "AMOUNT"]
    } else {
        ["CONCEPTO", "SKU", "CANT.", "P. UNITARIO", "IMPORTE"]
    };
    let widths = [3800, 1100, 800, 1830, 1830];
    let mut item_rows = String::from("<w:tr><w:trPr><w:tblHeader/></w:trPr>");
    for (idx, label) in headers.iter().enumerate() {
        item_rows.push_str(&cell(
            text_paragraph(
                label,
                true,
                "FFFFFF",
                18,
                if idx >= 2 { "right" } else { "left" },
            ),
            widths[idx],
            Some(BRAND_BLUE),
        ));
    }
    item_rows.push_str("</w:tr>");

    for (position, item) in items.iter().enumerate() {
        let fill = if position % 2 == 0 {
            Some("F8FAFD")
        } else {
            None
        };
        let mut description = text_paragraph(&item.name, true, INK, 20, "left");
        if let Some(detail) = item.description.as_deref().filter(|v| !v.trim().is_empty()) {
            description.push_str(&text_paragraph(detail, false, MUTED, 18, "left"));
        }
        item_rows.push_str("<w:tr>");
        item_rows.push_str(&cell(description, widths[0], fill));
        item_rows.push_str(&cell(
            text_paragraph(item.sku.as_deref().unwrap_or("-"), false, MUTED, 18, "left"),
            widths[1],
            fill,
        ));
        item_rows.push_str(&cell(
            text_paragraph(&item.quantity.to_string(), false, INK, 20, "right"),
            widths[2],
            fill,
        ));
        item_rows.push_str(&cell(
            text_paragraph(
                &format!("{} {}", item.currency, money(item.unit_price_cents)),
                false,
                INK,
                20,
                "right",
            ),
            widths[3],
            fill,
        ));
        item_rows.push_str(&cell(
            text_paragraph(
                &format!("{} {}", item.currency, money(item.line_total_cents)),
                true,
                INK,
                20,
                "right",
            ),
            widths[4],
            fill,
        ));
        item_rows.push_str("</w:tr>");
    }
    body.push_str(&table(item_rows, &widths, 120));
    body.push_str(&paragraph(String::new(), "left", 0, 120));

    let totals = [
        (
            if is_en { "Subtotal" } else { "Subtotal" },
            quote.subtotal_cents,
            false,
        ),
        (
            if is_en { "Taxes" } else { "Impuestos" },
            quote.tax_cents,
            false,
        ),
        (
            if is_en { "TOTAL" } else { "TOTAL" },
            quote.total_cents,
            true,
        ),
    ];
    let mut total_rows = String::new();
    for (label, amount, is_total) in totals {
        let fill = if is_total {
            Some(BRAND_RED)
        } else {
            Some("F8FAFD")
        };
        let color = if is_total { "FFFFFF" } else { INK };
        total_rows.push_str("<w:tr>");
        total_rows.push_str(&cell(
            text_paragraph(
                label,
                is_total,
                color,
                if is_total { 24 } else { 20 },
                "right",
            ),
            7530,
            fill,
        ));
        total_rows.push_str(&cell(
            text_paragraph(
                &format!("{} {}", quote.currency, money(amount)),
                true,
                color,
                if is_total { 24 } else { 20 },
                "right",
            ),
            1830,
            fill,
        ));
        total_rows.push_str("</w:tr>");
    }
    body.push_str(&table(total_rows, &[7530, 1830], 120));

    if let Some(notes) = quote
        .notes
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        body.push_str(&paragraph(
            run(if is_en { "NOTES" } else { "NOTAS" }, true, BRAND_RED, 18),
            "left",
            240,
            60,
        ));
        body.push_str(&paragraph(run(notes, false, MUTED, 20), "left", 0, 120));
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><w:body>{}<w:sectPr><w:footerReference w:type="default" r:id="rId1"/><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="708" w:footer="708" w:gutter="0"/><w:cols w:space="720"/><w:docGrid w:linePitch="360"/></w:sectPr></w:body></w:document>"#,
        body
    )
}

pub fn write_quote_docx(
    output_path: &Path,
    quote: &Quote,
    items: &[QuoteItem],
    template: &Template,
    lang: &str,
) -> Result<(), String> {
    let file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let document = build_document(quote, items, template, lang);

    let parts = [
        ("[Content_Types].xml", CONTENT_TYPES.to_string()),
        ("_rels/.rels", ROOT_RELS.to_string()),
        ("docProps/core.xml", CORE.to_string()),
        ("docProps/app.xml", APP.to_string()),
        ("word/document.xml", document),
        ("word/styles.xml", STYLES.to_string()),
        ("word/_rels/document.xml.rels", DOCUMENT_RELS.to_string()),
        ("word/footer1.xml", FOOTER.to_string()),
    ];

    for (path, content) in parts {
        zip.start_file(path, options).map_err(|e| e.to_string())?;
        zip.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/><Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/><Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/><Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/></Types>"#;
const ROOT_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/><Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/></Relationships>"#;
const DOCUMENT_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/></Relationships>"#;
const CORE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><dc:title>Cotizacion Vaestra</dc:title><dc:creator>Vaestra Cotizador</dc:creator><cp:lastModifiedBy>Vaestra Cotizador</cp:lastModifiedBy><dcterms:created xsi:type="dcterms:W3CDTF">2026-01-01T00:00:00Z</dcterms:created><dcterms:modified xsi:type="dcterms:W3CDTF">2026-01-01T00:00:00Z</dcterms:modified></cp:coreProperties>"#;
const APP: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"><Application>Vaestra Cotizador</Application><AppVersion>1.0</AppVersion></Properties>"#;
const STYLES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/><w:sz w:val="22"/><w:szCs w:val="22"/><w:color w:val="12213A"/></w:rPr></w:rPrDefault><w:pPrDefault><w:pPr><w:spacing w:after="120" w:line="264" w:lineRule="auto"/></w:pPr></w:pPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/><w:qFormat/><w:pPr><w:spacing w:after="120" w:line="264" w:lineRule="auto"/></w:pPr><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/><w:sz w:val="22"/><w:szCs w:val="22"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Title"><w:name w:val="Title"/><w:basedOn w:val="Normal"/><w:qFormat/><w:pPr><w:spacing w:after="80"/></w:pPr><w:rPr><w:b/><w:color w:val="0B3A82"/><w:sz w:val="48"/><w:szCs w:val="48"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/><w:pPr><w:keepNext/><w:spacing w:before="320" w:after="160"/></w:pPr><w:rPr><w:b/><w:color w:val="0B3A82"/><w:sz w:val="32"/><w:szCs w:val="32"/></w:rPr></w:style></w:styles>"#;
const FOOTER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:p><w:pPr><w:jc w:val="center"/><w:pBdr><w:top w:val="single" w:sz="8" w:color="0B3A82"/></w:pBdr><w:spacing w:before="100"/></w:pPr><w:r><w:rPr><w:color w:val="60708A"/><w:sz w:val="18"/></w:rPr><w:t>Vaestra Cotizador  |  Documento generado profesionalmente  |  Pagina </w:t></w:r><w:fldSimple w:instr="PAGE"><w:r><w:rPr><w:color w:val="D51F3A"/><w:b/><w:sz w:val="18"/></w:rPr><w:t>1</w:t></w:r></w:fldSimple></w:p></w:ftr>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_valid_qa_document() {
        let quote = Quote {
            id: 7,
            quote_number: Some("VAE-2026-007".to_string()),
            customer_name: Some("Constructora Horizonte, S.A. de C.V.".to_string()),
            customer_email: Some("compras@horizonte.mx".to_string()),
            notes: Some(
                "Cotizacion valida por 30 dias. Precios expresados en moneda nacional.".to_string(),
            ),
            subtotal_cents: 189_000,
            tax_rate_bps: 1600,
            tax_cents: 30_240,
            total_cents: 219_240,
            status: "draft".to_string(),
            created_at: "2026-07-20T18:30:00Z".to_string(),
            updated_at: "2026-07-20T18:30:00Z".to_string(),
            template_id: Some(1),
            currency: "MXN".to_string(),
        };
        let items = vec![
            QuoteItem {
                id: 1,
                quote_id: 7,
                product_id: Some(1),
                position: 0,
                sku: Some("CONS-01".to_string()),
                name: "Consultoria estrategica".to_string(),
                description: Some(
                    "Diagnostico, plan de implementacion y sesion ejecutiva.".to_string(),
                ),
                quantity: 2,
                unit_price_cents: 65_000,
                line_total_cents: 130_000,
                currency: "MXN".to_string(),
                created_at: quote.created_at.clone(),
                updated_at: quote.updated_at.clone(),
            },
            QuoteItem {
                id: 2,
                quote_id: 7,
                product_id: Some(2),
                position: 1,
                sku: Some("LIC-12".to_string()),
                name: "Licencia anual profesional".to_string(),
                description: Some(
                    "Acceso para cinco usuarios, soporte y actualizaciones.".to_string(),
                ),
                quantity: 1,
                unit_price_cents: 42_000,
                line_total_cents: 42_000,
                currency: "MXN".to_string(),
                created_at: quote.created_at.clone(),
                updated_at: quote.updated_at.clone(),
            },
            QuoteItem {
                id: 3,
                quote_id: 7,
                product_id: Some(3),
                position: 2,
                sku: Some("CAP-04".to_string()),
                name: "Capacitacion remota".to_string(),
                description: Some(
                    "Taller practico de cuatro horas para el equipo operativo.".to_string(),
                ),
                quantity: 1,
                unit_price_cents: 17_000,
                line_total_cents: 17_000,
                currency: "MXN".to_string(),
                created_at: quote.created_at.clone(),
                updated_at: quote.updated_at.clone(),
            },
        ];
        let template = Template {
            id: 1,
            name: "Vaestra Signature".to_string(),
            logo_path: None,
            logo_data_url: None,
            accent_color: Some("#0b3a82".to_string()),
            header_html: None,
            body_html: None,
            footer_html: None,
            created_at: quote.created_at.clone(),
            updated_at: quote.updated_at.clone(),
        };
        let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("qa");
        std::fs::create_dir_all(&output_dir).unwrap();
        let output = output_dir.join("vaestra-cotizacion-muestra.docx");
        write_quote_docx(&output, &quote, &items, &template, "es").unwrap();
        assert!(output.metadata().unwrap().len() > 2_000);
    }
}
