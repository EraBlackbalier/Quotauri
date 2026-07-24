#[derive(Debug, Clone, Copy)]
pub struct SeedTemplate {
    pub key: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub accent_color: &'static str,
    pub header_html: &'static str,
    pub body_html: &'static str,
    pub footer_html: &'static str,
}

pub fn default_templates() -> Vec<SeedTemplate> {
    vec![atelier(), ledger(), pulse(), editorial(), luxe(), vaestra_signature()]
}

pub fn find_template_design(key: &str) -> Option<SeedTemplate> {
    default_templates()
        .into_iter()
        .find(|template| template.key == key)
}

fn atelier() -> SeedTemplate {
    SeedTemplate {
        key: "atelier",
        name: "Atelier",
        description: "Editorial, amplio y elegante para propuestas premium.",
        accent_color: "#0f766e",
        header_html: r##"<style>
  .qt-atelier-ribbon { height: 12px; background: linear-gradient(90deg, {{accent_color}}, #f59e0b 48%, #111827); }
  .qt-atelier-head { padding: 34px 44px 22px; display: grid; grid-template-columns: 1fr auto; gap: 28px; align-items: start; }
  .qt-atelier-mark { width: 58px; height: 58px; border: 2px solid {{accent_color}}; display: grid; place-items: center; font-weight: 800; color: {{accent_color}}; }
  .qt-atelier-brand { display: flex; gap: 16px; align-items: center; }
  .qt-atelier-brand img { max-width: 118px; max-height: 58px; object-fit: contain; }
  .qt-atelier-brand h1 { margin: 0; font-size: 25px; letter-spacing: .04em; }
  .qt-atelier-brand p { margin: 5px 0 0; color: #64748b; font-size: 12px; text-transform: uppercase; letter-spacing: .12em; }
  .qt-atelier-meta { text-align: right; color: #334155; font-size: 12px; line-height: 1.7; }
  .qt-atelier-meta strong { display: block; color: #0f172a; font-size: 22px; line-height: 1.1; }
</style>
<div class="qt-atelier-ribbon"></div>
<header class="qt-atelier-head">
  <div class="qt-atelier-brand">
    {{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{else}}<div class="qt-atelier-mark">QT</div>{{/if}}
    <div><h1>{{title}}</h1><p>Propuesta comercial</p></div>
  </div>
  <div class="qt-atelier-meta">
    <strong>{{quote.quote_number_display}}</strong>
    {{quote.created_at}}<br/>{{quote.status}}
  </div>
</header>"##,
        body_html: r##"<style>
  .qt-atelier-body { padding: 10px 44px 26px; }
  .qt-atelier-client { display: grid; grid-template-columns: 1.1fr .9fr; gap: 22px; margin-bottom: 24px; }
  .qt-atelier-panel { border: 1px solid #dbe4ee; padding: 18px; background: #fbfdff; }
  .qt-atelier-label { margin: 0 0 8px; font-size: 10px; letter-spacing: .16em; text-transform: uppercase; color: {{accent_color}}; font-weight: 800; }
  .qt-atelier-name { margin: 0; font-size: 18px; font-weight: 800; color: #0f172a; }
  .qt-atelier-muted { margin: 3px 0 0; color: #64748b; font-size: 12px; }
  .qt-atelier-table { width: 100%; border-collapse: collapse; }
  .qt-atelier-table th { padding: 11px 10px; border-bottom: 2px solid #0f172a; text-align: left; font-size: 10px; text-transform: uppercase; letter-spacing: .12em; color: #0f172a; }
  .qt-atelier-table th:nth-child(n+3), .qt-atelier-table td:nth-child(n+3) { text-align: right; }
  .qt-atelier-table td { padding: 13px 10px; border-bottom: 1px solid #e5edf5; font-size: 12px; color: #334155; }
  .qt-atelier-item { font-weight: 800; color: #0f172a; }
  .qt-atelier-desc { margin-top: 3px; color: #64748b; font-size: 11px; }
  .qt-atelier-totalbox { margin-left: auto; margin-top: 18px; width: 285px; border: 1px solid #dbe4ee; padding: 14px 18px; }
  .qt-atelier-row { display: flex; justify-content: space-between; padding: 6px 0; font-size: 12px; color: #475569; }
  .qt-atelier-row.total { margin-top: 8px; padding-top: 12px; border-top: 2px solid {{accent_color}}; color: {{accent_color}}; font-size: 20px; font-weight: 900; }
</style>
<main class="qt-atelier-body">
  <section class="qt-atelier-client">
    <div class="qt-atelier-panel"><p class="qt-atelier-label">Cliente</p><p class="qt-atelier-name">{{quote.customer_name}}</p><p class="qt-atelier-muted">{{quote.customer_email}}</p></div>
    <div class="qt-atelier-panel"><p class="qt-atelier-label">Notas</p><p class="qt-atelier-muted">{{quote.notes}}</p></div>
  </section>
  <table class="qt-atelier-table">
    <thead><tr><th>Concepto</th><th>SKU</th><th>Cant.</th><th>Unitario</th><th>Importe</th></tr></thead>
    <tbody>{{#each items}}<tr><td><div class="qt-atelier-item">{{name}}</div>{{#if description}}<div class="qt-atelier-desc">{{description}}</div>{{/if}}</td><td>{{sku}}</td><td>{{quantity}}</td><td>{{currency}} {{unit_price}}</td><td>{{currency}} {{line_total}}</td></tr>{{/each}}</tbody>
  </table>
  <div class="qt-atelier-totalbox">
    <div class="qt-atelier-row"><span>Subtotal</span><span>{{quote.currency}} {{quote.subtotal}}</span></div>
    <div class="qt-atelier-row"><span>Impuestos</span><span>{{quote.currency}} {{quote.tax}}</span></div>
    <div class="qt-atelier-row total"><span>Total</span><span>{{quote.currency}} {{quote.total}}</span></div>
  </div>
</main>"##,
        footer_html: r##"<style>
  .qt-atelier-foot { position: absolute; left: 44px; right: 44px; bottom: 24px; display: flex; justify-content: space-between; align-items: center; color: #94a3b8; font-size: 10px; letter-spacing: .08em; text-transform: uppercase; }
  .qt-atelier-line { height: 1px; flex: 1; margin: 0 18px; background: linear-gradient(90deg, transparent, {{accent_color}}, transparent); }
</style>
<footer class="qt-atelier-foot"><span>Vaestra Cotizador</span><div class="qt-atelier-line"></div><span>{{quote.quote_number_display}}</span></footer>"##,
    }
}

fn ledger() -> SeedTemplate {
    SeedTemplate {
        key: "ledger",
        name: "Ledger Pro",
        description: "Limpio, corporativo y denso para cotizaciones recurrentes.",
        accent_color: "#2563eb",
        header_html: r##"<style>
  .qt-ledger-head { padding: 30px 38px 18px; border-bottom: 1px solid #d7e0eb; }
  .qt-ledger-top { display: flex; justify-content: space-between; align-items: start; gap: 20px; }
  .qt-ledger-brand { display: flex; gap: 14px; align-items: center; }
  .qt-ledger-brand img { max-width: 120px; max-height: 54px; object-fit: contain; }
  .qt-ledger-logo { width: 44px; height: 44px; border-radius: 8px; background: {{accent_color}}; color: white; display: grid; place-items: center; font-weight: 900; }
  .qt-ledger-title { margin: 0; font-size: 22px; color: #111827; }
  .qt-ledger-sub { margin: 4px 0 0; font-size: 12px; color: #6b7280; }
  .qt-ledger-stamp { border: 1px solid {{accent_color}}; color: {{accent_color}}; padding: 10px 14px; border-radius: 8px; text-align: right; }
  .qt-ledger-stamp strong { display: block; font-size: 18px; color: #111827; }
</style>
<header class="qt-ledger-head">
  <div class="qt-ledger-top">
    <div class="qt-ledger-brand">{{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{else}}<div class="qt-ledger-logo">Q</div>{{/if}}<div><h1 class="qt-ledger-title">{{title}}</h1><p class="qt-ledger-sub">Cotizacion / Invoice draft</p></div></div>
    <div class="qt-ledger-stamp"><strong>{{quote.quote_number_display}}</strong>{{quote.created_at}}</div>
  </div>
</header>"##,
        body_html: r##"<style>
  .qt-ledger-body { padding: 24px 38px; }
  .qt-ledger-info { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; margin-bottom: 20px; }
  .qt-ledger-cell { background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 12px; }
  .qt-ledger-k { font-size: 10px; color: #64748b; text-transform: uppercase; letter-spacing: .1em; margin-bottom: 5px; }
  .qt-ledger-v { color: #0f172a; font-weight: 750; font-size: 13px; }
  .qt-ledger-table { width: 100%; border-collapse: separate; border-spacing: 0; overflow: hidden; border: 1px solid #dbe4ee; border-radius: 8px; }
  .qt-ledger-table th { background: #edf4ff; color: #1d4ed8; padding: 10px; text-align: left; font-size: 10px; text-transform: uppercase; letter-spacing: .1em; }
  .qt-ledger-table td { padding: 10px; border-top: 1px solid #e5edf5; font-size: 12px; color: #334155; }
  .qt-ledger-table th:nth-child(n+3), .qt-ledger-table td:nth-child(n+3) { text-align: right; }
  .qt-ledger-desc { color: #64748b; font-size: 11px; margin-top: 2px; }
  .qt-ledger-summary { margin-left: auto; margin-top: 16px; width: 300px; }
  .qt-ledger-row { display: flex; justify-content: space-between; border-bottom: 1px solid #e2e8f0; padding: 8px 0; color: #475569; font-size: 12px; }
  .qt-ledger-row.total { border: 0; background: #0f172a; color: white; border-radius: 8px; margin-top: 10px; padding: 13px 14px; font-size: 18px; font-weight: 900; }
</style>
<main class="qt-ledger-body">
  <section class="qt-ledger-info"><div class="qt-ledger-cell"><div class="qt-ledger-k">Cliente</div><div class="qt-ledger-v">{{quote.customer_name}}</div></div><div class="qt-ledger-cell"><div class="qt-ledger-k">Email</div><div class="qt-ledger-v">{{quote.customer_email}}</div></div><div class="qt-ledger-cell"><div class="qt-ledger-k">Moneda</div><div class="qt-ledger-v">{{quote.currency}}</div></div></section>
  <table class="qt-ledger-table"><thead><tr><th>Descripcion</th><th>SKU</th><th>Cant.</th><th>Precio</th><th>Total</th></tr></thead><tbody>{{#each items}}<tr><td><strong>{{name}}</strong>{{#if description}}<div class="qt-ledger-desc">{{description}}</div>{{/if}}</td><td>{{sku}}</td><td>{{quantity}}</td><td>{{currency}} {{unit_price}}</td><td>{{currency}} {{line_total}}</td></tr>{{/each}}</tbody></table>
  <div class="qt-ledger-summary"><div class="qt-ledger-row"><span>Subtotal</span><span>{{quote.currency}} {{quote.subtotal}}</span></div><div class="qt-ledger-row"><span>Impuestos</span><span>{{quote.currency}} {{quote.tax}}</span></div><div class="qt-ledger-row total"><span>Total</span><span>{{quote.currency}} {{quote.total}}</span></div></div>
</main>"##,
        footer_html: r##"<style>.qt-ledger-foot{position:absolute;bottom:20px;left:38px;right:38px;color:#94a3b8;font-size:10px;display:flex;justify-content:space-between;border-top:1px solid #e2e8f0;padding-top:12px}</style><footer class="qt-ledger-foot"><span>{{quote.notes}}</span><span>Generado con Vaestra Cotizador</span></footer>"##,
    }
}

fn pulse() -> SeedTemplate {
    SeedTemplate {
        key: "pulse",
        name: "Pulse",
        description: "Alto contraste con energia visual para agencias y software.",
        accent_color: "#db2777",
        header_html: r##"<style>
  .qt-pulse-head { position: relative; padding: 38px 42px 30px; color: white; background: linear-gradient(135deg, #111827 0%, {{accent_color}} 100%); overflow: hidden; }
  .qt-pulse-head:before { content: ""; position: absolute; inset: 0; background: repeating-linear-gradient(135deg, rgba(255,255,255,.1) 0 1px, transparent 1px 18px); opacity: .45; }
  .qt-pulse-inner { position: relative; display: flex; justify-content: space-between; gap: 24px; align-items: start; }
  .qt-pulse-brand img { max-width: 118px; max-height: 54px; object-fit: contain; background: white; padding: 5px; border-radius: 8px; margin-bottom: 10px; }
  .qt-pulse-brand h1 { margin: 0; font-size: 29px; }
  .qt-pulse-brand p { margin: 6px 0 0; opacity: .78; font-size: 12px; letter-spacing: .12em; text-transform: uppercase; }
  .qt-pulse-number { text-align: right; font-weight: 900; font-size: 28px; }
  .qt-pulse-date { margin-top: 5px; font-size: 12px; opacity: .75; }
</style>
<header class="qt-pulse-head"><div class="qt-pulse-inner"><div class="qt-pulse-brand">{{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{/if}}<h1>{{title}}</h1><p>Propuesta creativa</p></div><div><div class="qt-pulse-number">{{quote.quote_number_display}}</div><div class="qt-pulse-date">{{quote.created_at}}</div></div></div></header>"##,
        body_html: r##"<style>
  .qt-pulse-body { padding: 28px 42px; }
  .qt-pulse-client { border-left: 6px solid {{accent_color}}; padding: 14px 18px; background: #fff1f7; margin-bottom: 22px; }
  .qt-pulse-client strong { display: block; color: #111827; font-size: 17px; }
  .qt-pulse-client span { color: #64748b; font-size: 12px; }
  .qt-pulse-table { width: 100%; border-collapse: collapse; }
  .qt-pulse-table th { color: #111827; padding: 11px 8px; border-bottom: 2px solid {{accent_color}}; text-align: left; font-size: 10px; text-transform: uppercase; letter-spacing: .12em; }
  .qt-pulse-table th:nth-child(n+3), .qt-pulse-table td:nth-child(n+3) { text-align: right; }
  .qt-pulse-table td { padding: 13px 8px; border-bottom: 1px solid #f0d9e4; color: #374151; font-size: 12px; }
  .qt-pulse-desc { margin-top: 2px; color: #7c899a; font-size: 11px; }
  .qt-pulse-total { margin-top: 20px; margin-left: auto; width: 310px; background: #111827; color: white; padding: 18px; border-radius: 14px; }
  .qt-pulse-row { display: flex; justify-content: space-between; padding: 5px 0; color: #cbd5e1; font-size: 12px; }
  .qt-pulse-row.total { color: white; font-size: 22px; font-weight: 900; border-top: 1px solid rgba(255,255,255,.22); margin-top: 8px; padding-top: 12px; }
</style>
<main class="qt-pulse-body"><section class="qt-pulse-client"><strong>{{quote.customer_name}}</strong><span>{{quote.customer_email}} / {{quote.status}}</span></section><table class="qt-pulse-table"><thead><tr><th>Item</th><th>SKU</th><th>Cant.</th><th>Unit.</th><th>Total</th></tr></thead><tbody>{{#each items}}<tr><td><strong>{{name}}</strong>{{#if description}}<div class="qt-pulse-desc">{{description}}</div>{{/if}}</td><td>{{sku}}</td><td>{{quantity}}</td><td>{{currency}} {{unit_price}}</td><td>{{currency}} {{line_total}}</td></tr>{{/each}}</tbody></table><div class="qt-pulse-total"><div class="qt-pulse-row"><span>Subtotal</span><span>{{quote.currency}} {{quote.subtotal}}</span></div><div class="qt-pulse-row"><span>Impuestos</span><span>{{quote.currency}} {{quote.tax}}</span></div><div class="qt-pulse-row total"><span>Total</span><span>{{quote.currency}} {{quote.total}}</span></div></div></main>"##,
        footer_html: r##"<style>.qt-pulse-foot{position:absolute;bottom:0;left:0;right:0;padding:18px 42px;background:#111827;color:#cbd5e1;font-size:10px;display:flex;justify-content:space-between}</style><footer class="qt-pulse-foot"><span>{{quote.notes}}</span><span>{{quote.quote_number_display}}</span></footer>"##,
    }
}

fn editorial() -> SeedTemplate {
    SeedTemplate {
        key: "editorial",
        name: "Editorial Grid",
        description: "Reticula sobria con detalles finos para consultoria.",
        accent_color: "#7c3aed",
        header_html: r##"<style>.qt-ed-head{padding:36px 46px 18px}.qt-ed-grid{display:grid;grid-template-columns:1fr 160px;gap:28px}.qt-ed-eyebrow{font-size:10px;letter-spacing:.22em;text-transform:uppercase;color:{{accent_color}};font-weight:900}.qt-ed-title{margin:8px 0 0;font-size:34px;line-height:1;color:#111827}.qt-ed-meta{border-left:1px solid #d8dee8;padding-left:18px;color:#64748b;font-size:12px}.qt-ed-meta strong{display:block;color:#111827;font-size:18px;margin-bottom:8px}.qt-ed-logo{max-width:120px;max-height:52px;object-fit:contain;margin-bottom:12px}</style><header class="qt-ed-head"><div class="qt-ed-grid"><div>{{#if logo_data_url}}<img class="qt-ed-logo" src="{{logo_data_url}}" alt="logo" />{{/if}}<div class="qt-ed-eyebrow">Cotizacion</div><h1 class="qt-ed-title">{{title}}</h1></div><div class="qt-ed-meta"><strong>{{quote.quote_number_display}}</strong>{{quote.created_at}}<br/>{{quote.currency}}</div></div></header>"##,
        body_html: r##"<style>.qt-ed-body{padding:16px 46px 28px}.qt-ed-client{padding:18px 0;border-top:2px solid #111827;border-bottom:1px solid #d8dee8;margin-bottom:18px;display:flex;justify-content:space-between;gap:20px}.qt-ed-client strong{font-size:17px;color:#111827}.qt-ed-client span{color:#64748b;font-size:12px}.qt-ed-table{width:100%;border-collapse:collapse}.qt-ed-table th{padding:12px 0;text-align:left;color:#64748b;font-size:10px;text-transform:uppercase;letter-spacing:.14em}.qt-ed-table th:nth-child(n+3),.qt-ed-table td:nth-child(n+3){text-align:right}.qt-ed-table td{padding:14px 0;border-top:1px solid #e2e8f0;font-size:12px;color:#334155}.qt-ed-table td:first-child{font-weight:800;color:#111827}.qt-ed-desc{font-weight:400;color:#64748b;font-size:11px;margin-top:3px}.qt-ed-total{display:grid;grid-template-columns:1fr 280px;gap:24px;margin-top:22px}.qt-ed-note{color:#64748b;font-size:12px;line-height:1.6}.qt-ed-box{border-top:3px solid {{accent_color}};padding-top:10px}.qt-ed-row{display:flex;justify-content:space-between;padding:6px 0;color:#475569;font-size:12px}.qt-ed-row.total{font-size:21px;font-weight:900;color:{{accent_color}}}</style><main class="qt-ed-body"><section class="qt-ed-client"><div><strong>{{quote.customer_name}}</strong><br/><span>{{quote.customer_email}}</span></div><span>{{quote.status}}</span></section><table class="qt-ed-table"><thead><tr><th>Concepto</th><th>SKU</th><th>Cant.</th><th>Precio</th><th>Total</th></tr></thead><tbody>{{#each items}}<tr><td>{{name}}{{#if description}}<div class="qt-ed-desc">{{description}}</div>{{/if}}</td><td>{{sku}}</td><td>{{quantity}}</td><td>{{currency}} {{unit_price}}</td><td>{{currency}} {{line_total}}</td></tr>{{/each}}</tbody></table><section class="qt-ed-total"><div class="qt-ed-note">{{quote.notes}}</div><div class="qt-ed-box"><div class="qt-ed-row"><span>Subtotal</span><span>{{quote.currency}} {{quote.subtotal}}</span></div><div class="qt-ed-row"><span>Impuestos</span><span>{{quote.currency}} {{quote.tax}}</span></div><div class="qt-ed-row total"><span>Total</span><span>{{quote.currency}} {{quote.total}}</span></div></div></section></main>"##,
        footer_html: r##"<style>.qt-ed-foot{position:absolute;bottom:24px;left:46px;right:46px;color:#94a3b8;font-size:10px;text-transform:uppercase;letter-spacing:.14em;display:flex;justify-content:space-between}</style><footer class="qt-ed-foot"><span>Vaestra Cotizador</span><span>{{quote.quote_number_display}}</span></footer>"##,
    }
}

fn luxe() -> SeedTemplate {
    SeedTemplate {
        key: "luxe",
        name: "Luxe Minimal",
        description: "Minimalista con acabado premium y acentos dorados.",
        accent_color: "#b45309",
        header_html: r##"<style>.qt-luxe-head{padding:38px 48px 22px;background:#fbfaf7;border-bottom:1px solid #eadfce}.qt-luxe-top{display:flex;justify-content:space-between;align-items:center}.qt-luxe-brand{display:flex;gap:14px;align-items:center}.qt-luxe-brand img{max-width:118px;max-height:56px;object-fit:contain}.qt-luxe-seal{width:50px;height:50px;border-radius:50%;border:1px solid {{accent_color}};display:grid;place-items:center;color:{{accent_color}};font-weight:900}.qt-luxe-title{margin:0;font-size:24px;color:#1c1917}.qt-luxe-sub{margin:5px 0 0;color:#78716c;font-size:12px}.qt-luxe-no{text-align:right;color:#78716c;font-size:12px}.qt-luxe-no strong{display:block;color:#1c1917;font-size:21px}</style><header class="qt-luxe-head"><div class="qt-luxe-top"><div class="qt-luxe-brand">{{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{else}}<div class="qt-luxe-seal">Q</div>{{/if}}<div><h1 class="qt-luxe-title">{{title}}</h1><p class="qt-luxe-sub">Documento comercial</p></div></div><div class="qt-luxe-no"><strong>{{quote.quote_number_display}}</strong>{{quote.created_at}}</div></div></header>"##,
        body_html: r##"<style>.qt-luxe-body{padding:28px 48px;color:#292524}.qt-luxe-client{margin-bottom:24px;padding-bottom:18px;border-bottom:1px solid #eadfce}.qt-luxe-label{font-size:10px;text-transform:uppercase;letter-spacing:.18em;color:{{accent_color}};font-weight:900;margin-bottom:8px}.qt-luxe-client strong{font-size:18px}.qt-luxe-client div:last-child{color:#78716c;font-size:12px;margin-top:3px}.qt-luxe-table{width:100%;border-collapse:collapse}.qt-luxe-table th{padding:10px 8px;text-align:left;font-size:10px;letter-spacing:.13em;text-transform:uppercase;color:#a16207;border-bottom:1px solid #b45309}.qt-luxe-table th:nth-child(n+3),.qt-luxe-table td:nth-child(n+3){text-align:right}.qt-luxe-table td{padding:13px 8px;border-bottom:1px solid #f0e7da;font-size:12px}.qt-luxe-desc{color:#78716c;font-size:11px;margin-top:3px}.qt-luxe-total{margin-left:auto;margin-top:20px;width:290px;background:#fbfaf7;border:1px solid #eadfce;padding:16px}.qt-luxe-row{display:flex;justify-content:space-between;padding:6px 0;color:#57534e;font-size:12px}.qt-luxe-row.total{border-top:1px solid #b45309;margin-top:8px;padding-top:12px;color:#92400e;font-weight:900;font-size:20px}</style><main class="qt-luxe-body"><section class="qt-luxe-client"><div class="qt-luxe-label">Preparado para</div><strong>{{quote.customer_name}}</strong><div>{{quote.customer_email}}</div></section><table class="qt-luxe-table"><thead><tr><th>Servicio</th><th>SKU</th><th>Cant.</th><th>Precio</th><th>Total</th></tr></thead><tbody>{{#each items}}<tr><td><strong>{{name}}</strong>{{#if description}}<div class="qt-luxe-desc">{{description}}</div>{{/if}}</td><td>{{sku}}</td><td>{{quantity}}</td><td>{{currency}} {{unit_price}}</td><td>{{currency}} {{line_total}}</td></tr>{{/each}}</tbody></table><div class="qt-luxe-total"><div class="qt-luxe-row"><span>Subtotal</span><span>{{quote.currency}} {{quote.subtotal}}</span></div><div class="qt-luxe-row"><span>Impuestos</span><span>{{quote.currency}} {{quote.tax}}</span></div><div class="qt-luxe-row total"><span>Total</span><span>{{quote.currency}} {{quote.total}}</span></div></div></main>"##,
        footer_html: r##"<style>.qt-luxe-foot{position:absolute;bottom:24px;left:48px;right:48px;border-top:1px solid #eadfce;padding-top:12px;color:#a8a29e;font-size:10px;display:flex;justify-content:space-between}</style><footer class="qt-luxe-foot"><span>{{quote.notes}}</span><span>Vaestra Cotizador</span></footer>"##,
    }
}

fn vaestra_signature() -> SeedTemplate {
    SeedTemplate {
        key: "vaestra-signature",
        name: "Vaestra Signature",
        description: "Plantilla oficial azul y rojo, lista para PDF, HTML y Word.",
        accent_color: "#0b3a82",
        header_html: r##"<style>
  .vt-band{height:10px;background:linear-gradient(90deg,#0b3a82 0 74%,#d51f3a 74% 100%)}
  .vt-head{padding:30px 42px 22px;display:flex;justify-content:space-between;align-items:flex-start;border-bottom:1px solid #d5deec}
  .vt-brand{display:flex;align-items:center;gap:14px}.vt-logo{max-width:112px;max-height:54px;object-fit:contain}
  .vt-mark{width:48px;height:48px;border-radius:14px 14px 14px 5px;display:grid;place-items:center;background:linear-gradient(145deg,#0b3a82 0 68%,#d51f3a 68%);color:#fff;font-size:22px;font-weight:900}
  .vt-title{margin:0;color:#0b3a82;font-size:25px;line-height:1}.vt-title span{color:#d51f3a}.vt-sub{margin:6px 0 0;color:#60708a;font-size:11px;letter-spacing:.14em;text-transform:uppercase}
  .vt-folio{text-align:right;color:#60708a;font-size:11px}.vt-folio strong{display:block;color:#12213a;font-size:22px;margin-bottom:5px}
</style><div class="vt-band"></div><header class="vt-head"><div class="vt-brand">{{#if logo_data_url}}<img class="vt-logo" src="{{logo_data_url}}" alt="logo"/>{{else}}<div class="vt-mark">V</div>{{/if}}<div><h1 class="vt-title">Vaestra <span>Cotizador</span></h1><p class="vt-sub">Propuesta comercial</p></div></div><div class="vt-folio"><strong>{{quote.quote_number_display}}</strong>{{quote.created_at}}</div></header>"##,
        body_html: r##"<style>
  .vt-body{padding:24px 42px 28px;color:#12213a}.vt-info{display:grid;grid-template-columns:1.3fr .7fr;gap:14px;margin-bottom:22px}
  .vt-card{padding:15px 17px;border:1px solid #d5deec;border-radius:10px;background:#f8fafd}.vt-card.red{border-left:4px solid #d51f3a}.vt-card.blue{border-left:4px solid #0b3a82}
  .vt-k{font-size:9px;font-weight:900;letter-spacing:.14em;text-transform:uppercase;color:#60708a;margin-bottom:6px}.vt-v{font-size:15px;font-weight:800}.vt-small{font-size:11px;color:#60708a;margin-top:3px}
  .vt-table{width:100%;border-collapse:collapse}.vt-table th{padding:10px 9px;background:#0b3a82;color:#fff;text-align:left;font-size:9px;letter-spacing:.1em;text-transform:uppercase}.vt-table th:last-child{background:#d51f3a}.vt-table th:nth-child(n+3),.vt-table td:nth-child(n+3){text-align:right}
  .vt-table td{padding:12px 9px;border-bottom:1px solid #dbe4f0;font-size:11px}.vt-table tbody tr:nth-child(even){background:#f8fafd}.vt-desc{margin-top:3px;color:#60708a;font-size:10px}
  .vt-summary{width:300px;margin:18px 0 0 auto}.vt-row{display:flex;justify-content:space-between;padding:7px 10px;color:#60708a;font-size:11px}.vt-row.total{margin-top:4px;border-radius:8px;background:#d51f3a;color:#fff;font-size:18px;font-weight:900;padding:12px}
</style><main class="vt-body"><section class="vt-info"><div class="vt-card blue"><div class="vt-k">Preparado para</div><div class="vt-v">{{quote.customer_name}}</div><div class="vt-small">{{quote.customer_email}}</div></div><div class="vt-card red"><div class="vt-k">Moneda / Estado</div><div class="vt-v">{{quote.currency}}</div><div class="vt-small">{{quote.status}}</div></div></section><table class="vt-table"><thead><tr><th>Concepto</th><th>SKU</th><th>Cant.</th><th>Precio</th><th>Importe</th></tr></thead><tbody>{{#each items}}<tr><td><strong>{{name}}</strong>{{#if description}}<div class="vt-desc">{{description}}</div>{{/if}}</td><td>{{sku}}</td><td>{{quantity}}</td><td>{{currency}} {{unit_price}}</td><td><strong>{{currency}} {{line_total}}</strong></td></tr>{{/each}}</tbody></table><div class="vt-summary"><div class="vt-row"><span>Subtotal</span><strong>{{quote.currency}} {{quote.subtotal}}</strong></div><div class="vt-row"><span>Impuestos</span><strong>{{quote.currency}} {{quote.tax}}</strong></div><div class="vt-row total"><span>Total</span><span>{{quote.currency}} {{quote.total}}</span></div></div></main>"##,
        footer_html: r##"<style>.vt-foot{position:absolute;bottom:22px;left:42px;right:42px;padding-top:10px;border-top:1px solid #d5deec;display:flex;justify-content:space-between;color:#60708a;font-size:9px}.vt-foot strong{color:#0b3a82}.vt-dot{color:#d51f3a}</style><footer class="vt-foot"><span>{{quote.notes}}</span><strong>VAESTRA <span class="vt-dot">COTIZADOR</span></strong></footer>"##,
    }
}
