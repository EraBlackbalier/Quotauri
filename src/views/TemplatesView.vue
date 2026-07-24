<script setup>
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "../components/DataTable.vue";
import { SUPPORTED_LANGUAGES, language, t } from "../i18n";

const templateLoading = ref(false);
const templateErrorMsg = ref("");
const templateSearch = ref("");
const templates = ref([]);
const templateDesigns = ref([]);
const selectedDesignKey = ref("");

const templateEditingId = ref(null);
const templateForm = ref({
  name: "",
  accent_color: "#0b3a82",
  logo_data_url: "",
});

const translationLang = ref("es");
const translationForm = ref({
  name: "",
  header_html: "",
  body_html: "",
  footer_html: "",
});

const templateVariablesJson = ref(
  JSON.stringify(
    {
      quote: {
        quote_number_display: "Q-000001",
        customer_name: "Empresa Demo S.A. de C.V.",
        customer_email: "contacto@empresa-demo.mx",
        notes: "Cotización válida por 30 días.",
        status: "draft",
        created_at: new Date().toISOString().split("T")[0],
        subtotal: "10,000.00",
        tax: "1,600.00",
        total: "11,600.00",
        currency: "MXN",
      },
      items: [
        { name: "Servicio de consultoría", sku: "SRV-001", description: "Asesoría técnica especializada", quantity: 2, unit_price: "3,500.00", line_total: "7,000.00", currency: "MXN" },
        { name: "Licencia de software", sku: "LIC-042", description: "Licencia anual premium", quantity: 1, unit_price: "2,500.00", line_total: "2,500.00", currency: "MXN" },
        { name: "Soporte técnico", sku: "SUP-010", description: null, quantity: 1, unit_price: "500.00", line_total: "500.00", currency: "MXN" },
      ],
    },
    null,
    2
  )
);

const templatePreviewHtml = ref("");
let templatePreviewTimer = null;

const DEFAULT_HEADER_HTML = `
<header style="padding:32px 36px 20px;border-bottom:3px solid {{accent_color}};">
  <div style="display:flex;justify-content:space-between;gap:16px;align-items:flex-start;">
    <div>
      <h1 style="margin:0;font-size:26px;line-height:1.2;">{{title}}</h1>
      <div style="margin-top:8px;color:#475569;">Propuesta comercial</div>
    </div>
    {{#if logo_data_url}}
    <img src="{{logo_data_url}}" alt="logo" style="max-width:110px;max-height:64px;object-fit:contain;" />
    {{/if}}
  </div>
</header>`;

const DEFAULT_BODY_HTML = `
<main style="padding:26px 36px;">
  <section style="display:grid;grid-template-columns:1fr 1fr;gap:18px;margin-bottom:22px;">
    <div>
      <div style="font-size:12px;color:#64748b;text-transform:uppercase;letter-spacing:.08em;">Cliente</div>
      <div style="font-size:16px;font-weight:700;margin-top:6px;">{{quote.customer_name}}</div>
      <div style="margin-top:2px;color:#334155;">{{quote.customer_email}}</div>
    </div>
    <div style="text-align:right;">
      <div style="font-size:12px;color:#64748b;text-transform:uppercase;letter-spacing:.08em;">Folio</div>
      <div style="font-size:16px;font-weight:700;margin-top:6px;">{{quote.quote_number_display}}</div>
      <div style="margin-top:2px;color:#334155;">{{quote.created_at}}</div>
    </div>
  </section>
  <table style="width:100%;border-collapse:collapse;">
    <thead>
      <tr style="background:#f1f5f9;">
        <th style="padding:10px;text-align:left;">Concepto</th>
        <th style="padding:10px;text-align:center;">Cant.</th>
        <th style="padding:10px;text-align:right;">P. unitario</th>
        <th style="padding:10px;text-align:right;">Importe</th>
      </tr>
    </thead>
    <tbody>
      {{#each items}}
      <tr>
        <td style="padding:10px;border-bottom:1px solid #e2e8f0;">
          <div style="font-weight:600;">{{name}}</div>
          {{#if description}}<div style="font-size:12px;color:#64748b;">{{description}}</div>{{/if}}
        </td>
        <td style="padding:10px;text-align:center;border-bottom:1px solid #e2e8f0;">{{quantity}}</td>
        <td style="padding:10px;text-align:right;border-bottom:1px solid #e2e8f0;">{{currency}} {{unit_price}}</td>
        <td style="padding:10px;text-align:right;border-bottom:1px solid #e2e8f0;">{{currency}} {{line_total}}</td>
      </tr>
      {{/each}}
    </tbody>
  </table>
</main>`;

const DEFAULT_FOOTER_HTML = `
<footer style="padding:16px 36px 30px;">
  <div style="margin-left:auto;max-width:260px;border-top:1px solid #e2e8f0;padding-top:10px;">
    <div style="display:flex;justify-content:space-between;padding:4px 0;">
      <span style="color:#64748b;">Subtotal</span><strong>{{quote.currency}} {{quote.subtotal}}</strong>
    </div>
    <div style="display:flex;justify-content:space-between;padding:4px 0;">
      <span style="color:#64748b;">Impuestos</span><strong>{{quote.currency}} {{quote.tax}}</strong>
    </div>
    <div style="display:flex;justify-content:space-between;padding:6px 0;font-size:18px;">
      <span>Total</span><strong style="color:{{accent_color}};">{{quote.currency}} {{quote.total}}</strong>
    </div>
  </div>
  <p style="margin:20px 0 0;color:#64748b;font-size:12px;">{{quote.notes}}</p>
</footer>`;

const isEditingTemplate = computed(() => templateEditingId.value !== null);

const columns = computed(() => [
  { key: "name", label: t("templates.name") },
  { key: "updated_at", label: t("templates.updatedAt"), width: "190px" },
  { key: "actions", label: "", width: "180px" },
]);

function defaultTranslationFromBase() {
  translationForm.value = {
    name: "",
    header_html: templateForm.value.header_html || "",
    body_html: templateForm.value.body_html || "",
    footer_html: templateForm.value.footer_html || "",
  };
}

function resetTemplateForm() {
  templateEditingId.value = null;
  templateForm.value = {
    name: "",
    accent_color: "#0b3a82",
    header_html: "",
    body_html: "",
    footer_html: "",
    logo_data_url: "",
  };

  defaultTranslationFromBase();
  scheduleTemplatePreview();
}

async function loadTemplateTranslation() {
  if (!templateEditingId.value) return;
  templateLoading.value = true;
  templateErrorMsg.value = "";
  try {
    const langCode = String(translationLang.value || "").trim();
    const existing = await invoke("get_template_translation", {
      templateId: templateEditingId.value,
      langCode,
    });

    if (existing) {
      translationForm.value = {
        name: existing.name ?? "",
        header_html: existing.header_html ?? templateForm.value.header_html ?? "",
        body_html: existing.body_html ?? templateForm.value.body_html ?? "",
        footer_html: existing.footer_html ?? templateForm.value.footer_html ?? "",
      };
      templateVariablesJson.value = existing.variables_json?.length
        ? String(existing.variables_json)
        : templateVariablesJson.value;
    } else {
      defaultTranslationFromBase();
    }

    scheduleTemplatePreview();
  } catch (e) {
    templateErrorMsg.value = String(e);
  } finally {
    templateLoading.value = false;
  }
}

async function refreshTemplates() {
  templateLoading.value = true;
  templateErrorMsg.value = "";
  try {
    const s = templateSearch.value.trim();
    templates.value = await invoke("list_templates", { search: s.length ? s : null });
  } catch (e) {
    templateErrorMsg.value = String(e);
  } finally {
    templateLoading.value = false;
  }
}

async function refreshTemplateDesigns() {
  try {
    templateDesigns.value = await invoke("list_template_designs");
    selectedDesignKey.value = templateDesigns.value.find((design) => design.key === "vaestra-signature")?.key
      ?? templateDesigns.value?.[0]?.key
      ?? "";
  } catch (e) {
    templateErrorMsg.value = String(e);
  }
}

async function applySelectedDesign() {
  const design = templateDesigns.value.find((d) => d.key === selectedDesignKey.value);
  if (!design) return;

  if (!templateForm.value.name.trim()) templateForm.value.name = design.name;
  templateForm.value.accent_color = design.accent_color || templateForm.value.accent_color;
  templateForm.value.header_html = design.header_html || "";
  templateForm.value.body_html = design.body_html || "";
  templateForm.value.footer_html = design.footer_html || "";
  defaultTranslationFromBase();

  if (templateEditingId.value) {
    templateLoading.value = true;
    templateErrorMsg.value = "";
    try {
      await invoke("apply_template_design", {
        id: templateEditingId.value,
        designKey: selectedDesignKey.value,
      });
      await refreshTemplates();
    } catch (e) {
      templateErrorMsg.value = String(e);
    } finally {
      templateLoading.value = false;
    }
  }

  scheduleTemplatePreview();
}

function startTemplateEdit(t) {
  templateEditingId.value = t.id;
  templateForm.value = {
    name: t.name ?? "",
    accent_color: t.accent_color ?? "#0b3a82",
    header_html: t.header_html ?? "",
    body_html: t.body_html ?? "",
    footer_html: t.footer_html ?? "",
    logo_data_url: t.logo_data_url ?? "",
  };
  defaultTranslationFromBase();
  loadTemplateTranslation();
}

async function saveTemplate() {
  templateLoading.value = true;
  templateErrorMsg.value = "";
  try {
    const payload = {
      name: String(templateForm.value.name || "").trim(),
      logo_path: null,
      logo_data_url: templateForm.value.logo_data_url.trim() ? templateForm.value.logo_data_url.trim() : null,
      accent_color: templateForm.value.accent_color.trim() ? templateForm.value.accent_color.trim() : null,
      header_html: templateForm.value.header_html.length ? templateForm.value.header_html : null,
      body_html: templateForm.value.body_html.length ? templateForm.value.body_html : null,
      footer_html: templateForm.value.footer_html.length ? templateForm.value.footer_html : null,
    };

    if (!payload.name.length) {
      templateErrorMsg.value = t("templates.nameRequired");
      return;
    }

    let templateId = templateEditingId.value;
    if (isEditingTemplate.value) {
      await invoke("update_template", { id: templateEditingId.value, input: payload });
    } else {
      const created = await invoke("create_template", { input: payload });
      templateId = created?.id ?? created?.template?.id ?? created?.template_id ?? null;
      templateEditingId.value = templateId;
    }

    const langCode = String(translationLang.value || "").trim();
    await invoke("upsert_template_translation", {
      input: {
        template_id: templateId,
        lang_code: langCode,
        name: translationForm.value.name.trim() ? translationForm.value.name.trim() : null,
        header_html: translationForm.value.header_html.length ? translationForm.value.header_html : null,
        body_html: translationForm.value.body_html.length ? translationForm.value.body_html : null,
        footer_html: translationForm.value.footer_html.length ? translationForm.value.footer_html : null,
        variables_json: String(templateVariablesJson.value || "").trim().length
          ? String(templateVariablesJson.value || "")
          : null,
      },
    });

    await refreshTemplates();
  } catch (e) {
    templateErrorMsg.value = String(e);
  } finally {
    templateLoading.value = false;
  }
}

async function removeTemplate(tpl) {
  if (!confirm(t("templates.deleteConfirm", tpl.name))) return;
  templateLoading.value = true;
  templateErrorMsg.value = "";
  try {
    await invoke("delete_template", { id: tpl.id });
    if (templateEditingId.value === tpl.id) resetTemplateForm();
    await refreshTemplates();
  } catch (e) {
    templateErrorMsg.value = String(e);
  } finally {
    templateLoading.value = false;
  }
}

function onLogoSelected(evt) {
  const file = evt?.target?.files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => {
    templateForm.value.logo_data_url = String(reader.result || "");
    scheduleTemplatePreview();
  };
  reader.readAsDataURL(file);
}

function parseVariablesJson() {
  const raw = String(templateVariablesJson.value || "").trim();
  if (!raw.length) return {};
  return JSON.parse(raw);
}

function mergedForPreview() {
  const header = translationForm.value.header_html?.trim()
    ? translationForm.value.header_html
    : templateForm.value.header_html?.trim()
      ? templateForm.value.header_html
      : DEFAULT_HEADER_HTML;
  const body = translationForm.value.body_html?.trim()
    ? translationForm.value.body_html
    : templateForm.value.body_html?.trim()
      ? templateForm.value.body_html
      : DEFAULT_BODY_HTML;
  const footer = translationForm.value.footer_html?.trim()
    ? translationForm.value.footer_html
    : templateForm.value.footer_html?.trim()
      ? templateForm.value.footer_html
      : DEFAULT_FOOTER_HTML;

  return {
    name: translationForm.value.name.trim() ? translationForm.value.name.trim() : templateForm.value.name,
    accent_color: templateForm.value.accent_color,
    logo_data_url: templateForm.value.logo_data_url,
    header_html: header,
    body_html: body,
    footer_html: footer,
  };
}

function scheduleTemplatePreview() {
  if (templatePreviewTimer) clearTimeout(templatePreviewTimer);
  templatePreviewTimer = setTimeout(async () => {
    templateLoading.value = true;
    templateErrorMsg.value = "";
    try {
      const variables = parseVariablesJson();
      templatePreviewHtml.value = await invoke("render_template_preview", {
        input: {
          name: mergedForPreview().name.trim() ? mergedForPreview().name.trim() : null,
          logo_data_url: mergedForPreview().logo_data_url.trim() ? mergedForPreview().logo_data_url.trim() : null,
          accent_color: mergedForPreview().accent_color.trim() ? mergedForPreview().accent_color.trim() : null,
          header_html: mergedForPreview().header_html,
          body_html: mergedForPreview().body_html,
          footer_html: mergedForPreview().footer_html,
          variables,
        },
      });
    } catch (e) {
      templateErrorMsg.value = String(e);
    } finally {
      templateLoading.value = false;
    }
  }, 250);
}

onMounted(async () => {
  translationLang.value = String(language.value || "es");
  await refreshTemplateDesigns();
  resetTemplateForm();
  await refreshTemplates();
});

watch(
  () => language.value,
  (v) => {
    translationLang.value = String(v || "es");
  }
);

watch(
  () => translationLang.value,
  () => {
    loadTemplateTranslation();
  }
);
</script>

<template>
  <section>
    <div class="app-header">
      <div>
        <h1>{{ t("templates.title") }}</h1>
        <p class="muted">{{ t("templates.subtitle") }}</p>
      </div>

      <div class="header-actions">
        <input v-model="templateSearch" class="search" :placeholder="t('templates.searchPlaceholder')" @input="refreshTemplates" />
        <button type="button" @click="refreshTemplates" :disabled="templateLoading">{{ t("common.refresh") }}</button>
      </div>
    </div>

    <div v-if="templateErrorMsg" class="error">{{ templateErrorMsg }}</div>

    <div class="grid-templates">
      <section class="card">
        <div class="card-title">{{ isEditingTemplate ? t("templates.editTitle") : t("templates.newTitle") }}</div>

        <form class="form" @submit.prevent="saveTemplate">
          <label>
            {{ t("templates.name") }}
            <input v-model="templateForm.name" @input="scheduleTemplatePreview" />
          </label>

          <label>
            {{ t("templates.accent") }}
            <input v-model="templateForm.accent_color" @input="scheduleTemplatePreview" placeholder="#0b3a82" />
          </label>

          <label>
            {{ t("templates.logo") }}
            <input type="file" accept="image/*" @change="onLogoSelected" />
          </label>

          <div class="template-design-picker">
            <label>
              {{ t("templates.designPreset") }}
              <select v-model="selectedDesignKey">
                <option v-for="d in templateDesigns" :key="d.key" :value="d.key">
                  {{ d.name }}
                </option>
              </select>
            </label>
            <button type="button" class="secondary" @click="applySelectedDesign" :disabled="templateLoading">
              {{ t("templates.applyDesign") }}
            </button>
          </div>

          <label>
            {{ t("language.label") }}
            <select v-model="translationLang" @change="scheduleTemplatePreview">
              <option v-for="l in SUPPORTED_LANGUAGES" :key="`tl-${l.code}`" :value="l.code">
                {{ l.label }}
              </option>
            </select>
          </label>

          <label>
            {{ t("templates.name") }} ({{ translationLang }})
            <input v-model="translationForm.name" @input="scheduleTemplatePreview" :placeholder="templateForm.name" />
          </label>

          <label>
            {{ t("templates.headerHtml") }} ({{ translationLang }})
            <textarea v-model="translationForm.header_html" rows="4" @input="scheduleTemplatePreview" />
          </label>

          <label>
            {{ t("templates.bodyHtml") }} ({{ translationLang }})
            <textarea v-model="translationForm.body_html" rows="8" @input="scheduleTemplatePreview" />
          </label>

          <label>
            {{ t("templates.footerHtml") }} ({{ translationLang }})
            <textarea v-model="translationForm.footer_html" rows="4" @input="scheduleTemplatePreview" />
          </label>

          <label>
            {{ t("templates.variablesJson") }} ({{ translationLang }})
            <textarea v-model="templateVariablesJson" rows="8" @input="scheduleTemplatePreview" />
          </label>

          <div class="actions">
            <button type="submit" :disabled="templateLoading">{{ isEditingTemplate ? t("common.save") : t("common.create") }}</button>
            <button type="button" class="secondary" @click="resetTemplateForm" :disabled="templateLoading">{{ t("common.new") }}</button>
          </div>
        </form>
      </section>

      <section class="card">
        <div class="card-title">{{ t("templates.listTitle") }}</div>

        <DataTable
          :columns="columns"
          :rows="templates"
          :loading="templateLoading"
          :loading-text="t('common.loading')"
          :empty-text="t('templates.empty')"
        >
          <template #cell-actions="{ row }">
            <div class="actions">
              <button type="button" class="secondary" @click="startTemplateEdit(row)">{{ t("common.edit") }}</button>
              <button type="button" class="danger" @click="removeTemplate(row)">{{ t("common.delete") }}</button>
            </div>
          </template>
        </DataTable>
      </section>

      <section class="card">
        <div class="card-title">{{ t("templates.previewTitle") }}</div>
        <div v-if="templateLoading" class="muted">{{ t("templates.rendering") }}</div>
        <iframe class="preview" sandbox="allow-same-origin" :srcdoc="templatePreviewHtml"></iframe>
      </section>
    </div>
  </section>
</template>
