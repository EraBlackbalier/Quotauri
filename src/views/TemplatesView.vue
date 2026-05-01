<script setup>
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "../components/DataTable.vue";
import { SUPPORTED_LANGUAGES, language, t } from "../i18n";

const templateLoading = ref(false);
const templateErrorMsg = ref("");
const templateSearch = ref("");
const templates = ref([]);

const templateEditingId = ref(null);
const templateForm = ref({
  name: "",
  accent_color: "#396cd8",
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
        quote_number: "Q-000001",
        customer_name: "Cliente de ejemplo",
        subtotal: "1000.00",
        tax: "160.00",
        total: "1160.00",
        currency: "MXN",
      },
    },
    null,
    2
  )
);

const templatePreviewHtml = ref("");
let templatePreviewTimer = null;

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
    accent_color: "#396cd8",
    header_html: "<h2>Cotización {{quote.quote_number}}</h2><div>{{quote.customer_name}}</div>",
    body_html:
      "<p>Subtotal: {{quote.currency}} {{quote.subtotal}}</p><p>Impuestos: {{quote.currency}} {{quote.tax}}</p><h3>Total: {{quote.currency}} {{quote.total}}</h3>",
    footer_html: "<small>Gracias por tu preferencia</small>",
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
      template_id: templateEditingId.value,
      lang_code: langCode,
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

function startTemplateEdit(t) {
  templateEditingId.value = t.id;
  templateForm.value = {
    name: t.name ?? "",
    accent_color: t.accent_color ?? "#396cd8",
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

async function removeTemplate(t) {
  if (!confirm(t("templates.deleteConfirm", t.name))) return;
  templateLoading.value = true;
  templateErrorMsg.value = "";
  try {
    await invoke("delete_template", { id: t.id });
    if (templateEditingId.value === t.id) resetTemplateForm();
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
  return {
    name: translationForm.value.name.trim() ? translationForm.value.name.trim() : templateForm.value.name,
    accent_color: templateForm.value.accent_color,
    logo_data_url: templateForm.value.logo_data_url,
    header_html: translationForm.value.header_html,
    body_html: translationForm.value.body_html,
    footer_html: translationForm.value.footer_html,
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
            <input v-model="templateForm.accent_color" @input="scheduleTemplatePreview" placeholder="#396cd8" />
          </label>

          <label>
            {{ t("templates.logo") }}
            <input type="file" accept="image/*" @change="onLogoSelected" />
          </label>

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
