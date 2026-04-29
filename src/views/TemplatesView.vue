<script setup>
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "../components/DataTable.vue";

const templateLoading = ref(false);
const templateErrorMsg = ref("");
const templateSearch = ref("");
const templates = ref([]);

const templateEditingId = ref(null);
const templateForm = ref({
  name: "",
  accent_color: "#396cd8",
  header_html: "",
  body_html: "",
  footer_html: "",
  logo_data_url: "",
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

const columns = [
  { key: "name", label: "Nombre" },
  { key: "updated_at", label: "Actualizado", width: "190px" },
  { key: "actions", label: "", width: "180px" },
];

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
  scheduleTemplatePreview();
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
  scheduleTemplatePreview();
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
      templateErrorMsg.value = "El nombre es requerido";
      return;
    }

    if (isEditingTemplate.value) {
      await invoke("update_template", { id: templateEditingId.value, input: payload });
    } else {
      await invoke("create_template", { input: payload });
    }

    await refreshTemplates();
  } catch (e) {
    templateErrorMsg.value = String(e);
  } finally {
    templateLoading.value = false;
  }
}

async function removeTemplate(t) {
  if (!confirm(`Eliminar template "${t.name}"?`)) return;
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

function scheduleTemplatePreview() {
  if (templatePreviewTimer) clearTimeout(templatePreviewTimer);
  templatePreviewTimer = setTimeout(async () => {
    templateLoading.value = true;
    templateErrorMsg.value = "";
    try {
      const variables = parseVariablesJson();
      templatePreviewHtml.value = await invoke("render_template_preview", {
        input: {
          name: templateForm.value.name.trim() ? templateForm.value.name.trim() : null,
          logo_data_url: templateForm.value.logo_data_url.trim() ? templateForm.value.logo_data_url.trim() : null,
          accent_color: templateForm.value.accent_color.trim() ? templateForm.value.accent_color.trim() : null,
          header_html: templateForm.value.header_html,
          body_html: templateForm.value.body_html,
          footer_html: templateForm.value.footer_html,
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
  resetTemplateForm();
  await refreshTemplates();
});
</script>

<template>
  <section>
    <div class="app-header">
      <div>
        <h1>Templates</h1>
        <p class="muted">Edita HTML y previsualiza con variables.</p>
      </div>

      <div class="header-actions">
        <input v-model="templateSearch" class="search" placeholder="Buscar templates" @input="refreshTemplates" />
        <button type="button" @click="refreshTemplates" :disabled="templateLoading">Refrescar</button>
      </div>
    </div>

    <div v-if="templateErrorMsg" class="error">{{ templateErrorMsg }}</div>

    <div class="grid-templates">
      <section class="card">
        <div class="card-title">{{ isEditingTemplate ? "Editar template" : "Nuevo template" }}</div>

        <form class="form" @submit.prevent="saveTemplate">
          <label>
            Nombre
            <input v-model="templateForm.name" @input="scheduleTemplatePreview" />
          </label>

          <label>
            Color acento
            <input v-model="templateForm.accent_color" @input="scheduleTemplatePreview" placeholder="#396cd8" />
          </label>

          <label>
            Logo
            <input type="file" accept="image/*" @change="onLogoSelected" />
          </label>

          <label>
            Header HTML
            <textarea v-model="templateForm.header_html" rows="4" @input="scheduleTemplatePreview" />
          </label>

          <label>
            Body HTML
            <textarea v-model="templateForm.body_html" rows="8" @input="scheduleTemplatePreview" />
          </label>

          <label>
            Footer HTML
            <textarea v-model="templateForm.footer_html" rows="4" @input="scheduleTemplatePreview" />
          </label>

          <label>
            Variables (JSON)
            <textarea v-model="templateVariablesJson" rows="8" @input="scheduleTemplatePreview" />
          </label>

          <div class="actions">
            <button type="submit" :disabled="templateLoading">{{ isEditingTemplate ? "Guardar" : "Crear" }}</button>
            <button type="button" class="secondary" @click="resetTemplateForm" :disabled="templateLoading">Nuevo</button>
          </div>
        </form>
      </section>

      <section class="card">
        <div class="card-title">Lista</div>

        <DataTable :columns="columns" :rows="templates" :loading="templateLoading" empty-text="No hay templates.">
          <template #cell-actions="{ row }">
            <div class="actions">
              <button type="button" class="secondary" @click="startTemplateEdit(row)">Editar</button>
              <button type="button" class="danger" @click="removeTemplate(row)">Eliminar</button>
            </div>
          </template>
        </DataTable>
      </section>

      <section class="card">
        <div class="card-title">Preview</div>
        <div v-if="templateLoading" class="muted">Renderizando...</div>
        <iframe class="preview" sandbox="allow-same-origin" :srcdoc="templatePreviewHtml"></iframe>
      </section>
    </div>
  </section>
</template>
