pub struct SeedTemplate {
    pub name: &'static str,
    pub accent_color: &'static str,
    pub header_html: &'static str,
    pub body_html: &'static str,
    pub footer_html: &'static str,
}

pub fn default_templates() -> Vec<SeedTemplate> {
    vec![elegant(), modern(), bold()]
}

fn elegant() -> SeedTemplate {
    SeedTemplate {
        name: "Elegant",
        accent_color: "#2c3e50",
        header_html: r##"<style>
  .elegant-header { padding: 40px 50px 30px; display: flex; justify-content: space-between; align-items: flex-start; border-bottom: 1px solid #e0e0e0; }
  .elegant-brand { display: flex; align-items: center; gap: 16px; }
  .elegant-brand img { max-height: 56px; max-width: 180px; object-fit: contain; }
  .elegant-brand-name { font-size: 26px; font-weight: 300; letter-spacing: 2px; color: {{accent_color}}; text-transform: uppercase; }
  .elegant-quote-info { text-align: right; }
  .elegant-quote-title { font-size: 28px; font-weight: 300; letter-spacing: 3px; color: {{accent_color}}; text-transform: uppercase; margin: 0 0 8px; }
  .elegant-meta { font-size: 12px; color: #777; line-height: 1.8; }
  .elegant-meta strong { color: #333; font-weight: 600; }
</style>
<div class="elegant-header">
  <div class="elegant-brand">
    {{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{/if}}
    <span class="elegant-brand-name">{{title}}</span>
  </div>
  <div class="elegant-quote-info">
    <h1 class="elegant-quote-title">Cotización</h1>
    <div class="elegant-meta">
      <strong>No.</strong> {{quote.quote_number_display}}<br/>
      <strong>Fecha:</strong> {{quote.created_at}}<br/>
      <strong>Estado:</strong> {{quote.status}}
    </div>
  </div>
</div>"##,
        body_html: r##"<style>
  .elegant-body { padding: 30px 50px; }
  .elegant-client { margin-bottom: 30px; padding: 20px 24px; background: #fafafa; border-left: 3px solid {{accent_color}}; }
  .elegant-client-title { font-size: 11px; text-transform: uppercase; letter-spacing: 1.5px; color: #999; margin: 0 0 8px; }
  .elegant-client-name { font-size: 18px; font-weight: 600; color: #222; margin: 0 0 4px; }
  .elegant-client-email { font-size: 13px; color: #666; }
  .elegant-table { width: 100%; border-collapse: collapse; margin-bottom: 30px; }
  .elegant-table thead th { background: {{accent_color}}; color: #fff; padding: 12px 16px; font-size: 11px; text-transform: uppercase; letter-spacing: 1px; font-weight: 600; text-align: left; }
  .elegant-table thead th:last-child, .elegant-table thead th:nth-child(3), .elegant-table thead th:nth-child(4) { text-align: right; }
  .elegant-table tbody td { padding: 12px 16px; border-bottom: 1px solid #eee; font-size: 13px; color: #333; }
  .elegant-table tbody td:last-child, .elegant-table tbody td:nth-child(3), .elegant-table tbody td:nth-child(4) { text-align: right; }
  .elegant-table tbody tr:nth-child(even) { background: #fafafa; }
  .elegant-item-desc { font-size: 11px; color: #888; margin-top: 2px; }
  .elegant-totals { display: flex; justify-content: flex-end; }
  .elegant-totals-box { width: 280px; }
  .elegant-totals-row { display: flex; justify-content: space-between; padding: 8px 0; font-size: 13px; color: #555; border-bottom: 1px solid #eee; }
  .elegant-totals-row.total { font-size: 18px; font-weight: 700; color: {{accent_color}}; border-bottom: none; border-top: 2px solid {{accent_color}}; padding-top: 12px; }
  .elegant-notes { margin-top: 30px; padding: 16px 20px; background: #fafafa; border-radius: 4px; font-size: 12px; color: #666; line-height: 1.6; }
  .elegant-notes-title { font-size: 11px; text-transform: uppercase; letter-spacing: 1px; color: #999; margin: 0 0 6px; }
</style>
<div class="elegant-body">
  <div class="elegant-client">
    <p class="elegant-client-title">Cliente</p>
    <p class="elegant-client-name">{{quote.customer_name}}</p>
    <p class="elegant-client-email">{{quote.customer_email}}</p>
  </div>
  <table class="elegant-table">
    <thead>
      <tr>
        <th>Descripción</th>
        <th>SKU</th>
        <th>Cant.</th>
        <th>Precio Unit.</th>
        <th>Total</th>
      </tr>
    </thead>
    <tbody>
      {{#each items}}
      <tr>
        <td>{{this.name}}{{#if this.description}}<div class="elegant-item-desc">{{this.description}}</div>{{/if}}</td>
        <td>{{this.sku}}</td>
        <td>{{this.quantity}}</td>
        <td>${{this.unit_price}} {{this.currency}}</td>
        <td>${{this.line_total}} {{this.currency}}</td>
      </tr>
      {{/each}}
    </tbody>
  </table>
  <div class="elegant-totals">
    <div class="elegant-totals-box">
      <div class="elegant-totals-row"><span>Subtotal</span><span>${{quote.subtotal}} {{quote.currency}}</span></div>
      <div class="elegant-totals-row"><span>Impuestos</span><span>${{quote.tax}} {{quote.currency}}</span></div>
      <div class="elegant-totals-row total"><span>Total</span><span>${{quote.total}} {{quote.currency}}</span></div>
    </div>
  </div>
  {{#if quote.notes}}
  <div class="elegant-notes">
    <p class="elegant-notes-title">Notas</p>
    <p>{{quote.notes}}</p>
  </div>
  {{/if}}
</div>"##,
        footer_html: r##"<style>
  .elegant-footer { position: absolute; bottom: 0; left: 0; right: 0; padding: 20px 50px; border-top: 1px solid #e0e0e0; display: flex; justify-content: space-between; align-items: center; }
  .elegant-footer-text { font-size: 10px; color: #aaa; letter-spacing: 0.5px; }
  .elegant-footer-accent { width: 40px; height: 3px; background: {{accent_color}}; }
</style>
<div class="elegant-footer">
  <span class="elegant-footer-text">Generado con Quotauri</span>
  <div class="elegant-footer-accent"></div>
  <span class="elegant-footer-text">{{quote.quote_number_display}}</span>
</div>"##,
    }
}

fn modern() -> SeedTemplate {
    SeedTemplate {
        name: "Modern",
        accent_color: "#6366f1",
        header_html: r##"<style>
  .modern-header { position: relative; padding: 0; }
  .modern-accent-bar { height: 8px; background: linear-gradient(90deg, {{accent_color}}, {{accent_color}}cc, {{accent_color}}66); }
  .modern-header-content { padding: 32px 50px 24px; display: flex; justify-content: space-between; align-items: flex-start; }
  .modern-brand { display: flex; align-items: center; gap: 14px; }
  .modern-brand img { max-height: 48px; max-width: 160px; object-fit: contain; border-radius: 8px; }
  .modern-brand-text h2 { margin: 0; font-size: 22px; font-weight: 700; color: #1e1e2e; }
  .modern-brand-text p { margin: 4px 0 0; font-size: 12px; color: #888; }
  .modern-quote-badge { display: inline-flex; align-items: center; gap: 8px; background: {{accent_color}}12; border: 1px solid {{accent_color}}33; border-radius: 20px; padding: 8px 20px; }
  .modern-quote-badge-label { font-size: 11px; text-transform: uppercase; letter-spacing: 1px; color: {{accent_color}}; font-weight: 600; }
  .modern-quote-badge-number { font-size: 16px; font-weight: 700; color: {{accent_color}}; }
  .modern-meta-row { padding: 0 50px 20px; display: flex; gap: 32px; }
  .modern-meta-item { font-size: 12px; color: #666; }
  .modern-meta-item strong { color: #333; }
</style>
<div class="modern-header">
  <div class="modern-accent-bar"></div>
  <div class="modern-header-content">
    <div class="modern-brand">
      {{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{/if}}
      <div class="modern-brand-text">
        <h2>{{title}}</h2>
        <p>Cotización Profesional</p>
      </div>
    </div>
    <div class="modern-quote-badge">
      <span class="modern-quote-badge-label">Cotización</span>
      <span class="modern-quote-badge-number">#{{quote.quote_number_display}}</span>
    </div>
  </div>
  <div class="modern-meta-row">
    <div class="modern-meta-item"><strong>Fecha:</strong> {{quote.created_at}}</div>
    <div class="modern-meta-item"><strong>Estado:</strong> {{quote.status}}</div>
  </div>
</div>"##,
        body_html: r##"<style>
  .modern-body { padding: 0 50px 30px; }
  .modern-client-card { background: linear-gradient(135deg, #f8f9ff 0%, #f0f1ff 100%); border-radius: 12px; padding: 20px 24px; margin-bottom: 28px; border: 1px solid {{accent_color}}15; }
  .modern-client-label { font-size: 10px; text-transform: uppercase; letter-spacing: 1.5px; color: {{accent_color}}; font-weight: 700; margin: 0 0 10px; }
  .modern-client-name { font-size: 17px; font-weight: 600; color: #1e1e2e; margin: 0 0 4px; }
  .modern-client-email { font-size: 13px; color: #666; margin: 0; }
  .modern-table { width: 100%; border-collapse: separate; border-spacing: 0; margin-bottom: 28px; border-radius: 10px; overflow: hidden; border: 1px solid #e8e8ef; }
  .modern-table thead th { background: {{accent_color}}; color: #fff; padding: 14px 16px; font-size: 11px; text-transform: uppercase; letter-spacing: 0.8px; font-weight: 600; text-align: left; }
  .modern-table thead th:nth-child(3), .modern-table thead th:nth-child(4), .modern-table thead th:last-child { text-align: right; }
  .modern-table tbody td { padding: 14px 16px; font-size: 13px; color: #444; border-bottom: 1px solid #f0f0f5; }
  .modern-table tbody td:nth-child(3), .modern-table tbody td:nth-child(4), .modern-table tbody td:last-child { text-align: right; }
  .modern-table tbody tr:last-child td { border-bottom: none; }
  .modern-table tbody tr:hover { background: #fafaff; }
  .modern-item-desc { font-size: 11px; color: #999; margin-top: 3px; }
  .modern-summary { display: flex; justify-content: flex-end; }
  .modern-summary-box { width: 300px; background: #fafafe; border-radius: 10px; padding: 16px 20px; border: 1px solid #e8e8ef; }
  .modern-summary-row { display: flex; justify-content: space-between; padding: 8px 0; font-size: 13px; color: #555; }
  .modern-summary-row.total { font-size: 20px; font-weight: 800; color: {{accent_color}}; padding-top: 14px; margin-top: 6px; border-top: 2px solid {{accent_color}}33; }
  .modern-notes { margin-top: 24px; padding: 16px 20px; background: #fffcf0; border-radius: 8px; border-left: 4px solid #f5c542; font-size: 12px; color: #666; line-height: 1.6; }
  .modern-notes strong { display: block; font-size: 11px; text-transform: uppercase; letter-spacing: 1px; color: #c49b20; margin-bottom: 4px; }
</style>
<div class="modern-body">
  <div class="modern-client-card">
    <p class="modern-client-label">Facturar a</p>
    <p class="modern-client-name">{{quote.customer_name}}</p>
    <p class="modern-client-email">{{quote.customer_email}}</p>
  </div>
  <table class="modern-table">
    <thead>
      <tr>
        <th>Artículo</th>
        <th>SKU</th>
        <th>Cant.</th>
        <th>Precio</th>
        <th>Importe</th>
      </tr>
    </thead>
    <tbody>
      {{#each items}}
      <tr>
        <td>{{this.name}}{{#if this.description}}<div class="modern-item-desc">{{this.description}}</div>{{/if}}</td>
        <td>{{this.sku}}</td>
        <td>{{this.quantity}}</td>
        <td>${{this.unit_price}}</td>
        <td>${{this.line_total}}</td>
      </tr>
      {{/each}}
    </tbody>
  </table>
  <div class="modern-summary">
    <div class="modern-summary-box">
      <div class="modern-summary-row"><span>Subtotal</span><span>${{quote.subtotal}} {{quote.currency}}</span></div>
      <div class="modern-summary-row"><span>Impuestos</span><span>${{quote.tax}} {{quote.currency}}</span></div>
      <div class="modern-summary-row total"><span>Total</span><span>${{quote.total}} {{quote.currency}}</span></div>
    </div>
  </div>
  {{#if quote.notes}}
  <div class="modern-notes">
    <strong>Notas</strong>
    {{quote.notes}}
  </div>
  {{/if}}
</div>"##,
        footer_html: r##"<style>
  .modern-footer { position: absolute; bottom: 0; left: 0; right: 0; padding: 16px 50px; background: #fafaff; border-top: 1px solid #e8e8ef; display: flex; justify-content: space-between; align-items: center; }
  .modern-footer-text { font-size: 10px; color: #aaa; }
  .modern-footer-dots { display: flex; gap: 4px; }
  .modern-footer-dot { width: 6px; height: 6px; border-radius: 50%; background: {{accent_color}}44; }
  .modern-footer-dot:first-child { background: {{accent_color}}; }
</style>
<div class="modern-footer">
  <span class="modern-footer-text">Generado con Quotauri</span>
  <div class="modern-footer-dots"><div class="modern-footer-dot"></div><div class="modern-footer-dot"></div><div class="modern-footer-dot"></div></div>
  <span class="modern-footer-text">{{quote.quote_number_display}} · {{quote.created_at}}</span>
</div>"##,
    }
}

fn bold() -> SeedTemplate {
    SeedTemplate {
        name: "Bold",
        accent_color: "#dc2626",
        header_html: r##"<style>
  .bold-header { background: {{accent_color}}; color: #fff; padding: 40px 50px 32px; position: relative; overflow: hidden; }
  .bold-header::after { content: ''; position: absolute; top: -60px; right: -60px; width: 200px; height: 200px; background: rgba(255,255,255,0.08); border-radius: 50%; }
  .bold-header::before { content: ''; position: absolute; bottom: -40px; left: 30%; width: 120px; height: 120px; background: rgba(255,255,255,0.05); border-radius: 50%; }
  .bold-header-top { display: flex; justify-content: space-between; align-items: flex-start; position: relative; z-index: 1; }
  .bold-brand { display: flex; align-items: center; gap: 16px; }
  .bold-brand img { max-height: 52px; max-width: 180px; object-fit: contain; background: #fff; border-radius: 8px; padding: 4px 8px; }
  .bold-brand-name { font-size: 28px; font-weight: 800; letter-spacing: 1px; }
  .bold-quote-num { text-align: right; }
  .bold-quote-num h1 { margin: 0; font-size: 36px; font-weight: 900; letter-spacing: 2px; opacity: 0.95; }
  .bold-quote-num p { margin: 4px 0 0; font-size: 12px; opacity: 0.7; letter-spacing: 1px; text-transform: uppercase; }
  .bold-meta { display: flex; gap: 40px; margin-top: 20px; position: relative; z-index: 1; }
  .bold-meta-item { font-size: 12px; opacity: 0.85; }
  .bold-meta-item strong { display: block; font-size: 10px; text-transform: uppercase; letter-spacing: 1px; opacity: 0.6; margin-bottom: 2px; }
</style>
<div class="bold-header">
  <div class="bold-header-top">
    <div class="bold-brand">
      {{#if logo_data_url}}<img src="{{logo_data_url}}" alt="logo" />{{/if}}
      <span class="bold-brand-name">{{title}}</span>
    </div>
    <div class="bold-quote-num">
      <h1>{{quote.quote_number_display}}</h1>
      <p>Cotización</p>
    </div>
  </div>
  <div class="bold-meta">
    <div class="bold-meta-item"><strong>Fecha</strong>{{quote.created_at}}</div>
    <div class="bold-meta-item"><strong>Estado</strong>{{quote.status}}</div>
    <div class="bold-meta-item"><strong>Moneda</strong>{{quote.currency}}</div>
  </div>
</div>"##,
        body_html: r##"<style>
  .bold-body { padding: 32px 50px; }
  .bold-client { display: flex; gap: 40px; margin-bottom: 28px; padding-bottom: 20px; border-bottom: 2px solid #f0f0f0; }
  .bold-client-section { flex: 1; }
  .bold-client-label { font-size: 10px; text-transform: uppercase; letter-spacing: 1.5px; color: {{accent_color}}; font-weight: 700; margin: 0 0 8px; }
  .bold-client-name { font-size: 16px; font-weight: 600; color: #1a1a1a; margin: 0 0 4px; }
  .bold-client-email { font-size: 13px; color: #666; margin: 0; }
  .bold-table { width: 100%; border-collapse: collapse; margin-bottom: 28px; }
  .bold-table thead th { padding: 14px 16px; font-size: 11px; text-transform: uppercase; letter-spacing: 1px; font-weight: 700; color: {{accent_color}}; border-bottom: 3px solid {{accent_color}}; text-align: left; background: transparent; }
  .bold-table thead th:nth-child(3), .bold-table thead th:nth-child(4), .bold-table thead th:last-child { text-align: right; }
  .bold-table tbody td { padding: 14px 16px; font-size: 13px; color: #333; border-bottom: 1px solid #eee; }
  .bold-table tbody td:nth-child(3), .bold-table tbody td:nth-child(4), .bold-table tbody td:last-child { text-align: right; }
  .bold-table tbody tr:hover { background: #fef2f2; }
  .bold-item-desc { font-size: 11px; color: #999; margin-top: 2px; }
  .bold-totals { display: flex; justify-content: flex-end; }
  .bold-totals-box { width: 300px; }
  .bold-totals-row { display: flex; justify-content: space-between; padding: 10px 0; font-size: 14px; color: #444; }
  .bold-totals-row.total { font-size: 22px; font-weight: 900; color: #fff; background: {{accent_color}}; margin: 8px -16px 0; padding: 14px 16px; border-radius: 8px; }
  .bold-notes { margin-top: 28px; padding: 16px 20px; background: #f9f9f9; border-radius: 8px; border-left: 4px solid {{accent_color}}; font-size: 12px; color: #555; line-height: 1.7; }
  .bold-notes strong { display: block; font-size: 11px; text-transform: uppercase; letter-spacing: 1px; color: {{accent_color}}; margin-bottom: 6px; }
</style>
<div class="bold-body">
  <div class="bold-client">
    <div class="bold-client-section">
      <p class="bold-client-label">Cliente</p>
      <p class="bold-client-name">{{quote.customer_name}}</p>
      <p class="bold-client-email">{{quote.customer_email}}</p>
    </div>
  </div>
  <table class="bold-table">
    <thead>
      <tr>
        <th>Producto / Servicio</th>
        <th>SKU</th>
        <th>Cant.</th>
        <th>P. Unitario</th>
        <th>Importe</th>
      </tr>
    </thead>
    <tbody>
      {{#each items}}
      <tr>
        <td>{{this.name}}{{#if this.description}}<div class="bold-item-desc">{{this.description}}</div>{{/if}}</td>
        <td>{{this.sku}}</td>
        <td>{{this.quantity}}</td>
        <td>${{this.unit_price}}</td>
        <td>${{this.line_total}}</td>
      </tr>
      {{/each}}
    </tbody>
  </table>
  <div class="bold-totals">
    <div class="bold-totals-box">
      <div class="bold-totals-row"><span>Subtotal</span><span>${{quote.subtotal}} {{quote.currency}}</span></div>
      <div class="bold-totals-row"><span>Impuestos</span><span>${{quote.tax}} {{quote.currency}}</span></div>
      <div class="bold-totals-row total"><span>Total</span><span>${{quote.total}} {{quote.currency}}</span></div>
    </div>
  </div>
  {{#if quote.notes}}
  <div class="bold-notes">
    <strong>Notas</strong>
    {{quote.notes}}
  </div>
  {{/if}}
</div>"##,
        footer_html: r##"<style>
  .bold-footer { position: absolute; bottom: 0; left: 0; right: 0; }
  .bold-footer-content { padding: 14px 50px; display: flex; justify-content: space-between; align-items: center; font-size: 10px; color: #999; }
  .bold-footer-bar { height: 6px; background: linear-gradient(90deg, {{accent_color}}, {{accent_color}}88, {{accent_color}}33); }
</style>
<div class="bold-footer">
  <div class="bold-footer-content">
    <span>Generado con Quotauri</span>
    <span>{{quote.quote_number_display}}</span>
  </div>
  <div class="bold-footer-bar"></div>
</div>"##,
    }
}
