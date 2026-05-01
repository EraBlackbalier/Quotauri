import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const SUPPORTED_LANGUAGES = [
  { code: "es", label: "Español" },
  { code: "en", label: "English" },
];

const translations = {
  es: {
    nav: {
      products: "Productos",
      quotes: "Cotizaciones",
      templates: "Templates",
    },
    common: {
      refresh: "Refrescar",
      save: "Guardar",
      create: "Crear",
      cancel: "Cancelar",
      new: "Nuevo",
      delete: "Eliminar",
      edit: "Editar",
      loading: "Cargando...",
      search: "Buscar",
      clear: "Limpiar",
      searching: "Buscando...",
      noResults: "Sin resultados.",
      optional: "Opcional",
    },
    theme: {
      light: "Modo claro",
      dark: "Modo oscuro",
    },
    language: {
      label: "Idioma",
      quickSwitch: "Cambiar idioma",
    },
    products: {
      title: "Productos",
      subtitle: "Administra tu catálogo para cotizaciones.",
      searchPlaceholder: "Buscar por nombre o SKU",
      formTitleNew: "Nuevo producto",
      formTitleEdit: "Editar producto",
      sku: "SKU",
      skuOptional: "Opcional",
      name: "Nombre",
      namePlaceholder: "Ej. Servicio de diseño",
      description: "Descripción",
      priceCents: "Precio (centavos)",
      currency: "Moneda",
      listTitle: "Lista",
      empty: "No hay productos.",
      nameRequired: "El nombre es requerido",
      deleteConfirm: (name) => `Eliminar producto "${name}"?`,
      price: "Precio",
      actions: "",
    },
    quotes: {
      title: "Cotizaciones",
      subtitle: "Arma una cotización, guárdala y consulta el historial.",
      searchPlaceholder: "Buscar por folio o cliente",
      newTitle: "Nueva cotización",
      savedTitle: "Guardadas",
      detailTitle: "Detalle",
      folio: "Folio",
      taxBps: "Impuesto (bps)",
      customer: "Cliente",
      email: "Email",
      notes: "Notas",
      addProducts: "Agregar productos",
      productSearch: "Buscar productos para agregar",
      qty: "Cant.",
      unitPriceCents: "Precio (centavos)",
      total: "Total",
      remove: "Quitar",
      subtotal: "Subtotal",
      taxes: "Impuestos",
      saveQuote: "Guardar cotización",
      empty: "No hay cotizaciones.",
      emptyItems: "Sin items.",
      withoutCustomer: "(sin cliente)",
      addAtLeastOne: "Agrega al menos un producto",
      exportPdf: "Exportar PDF",
      exportHtml: "Exportar HTML",
      exporting: "Exportando...",
      exportDone: "Exportación lista",
    },
    templates: {
      title: "Templates",
      subtitle: "Edita HTML y previsualiza con variables.",
      searchPlaceholder: "Buscar templates",
      editTitle: "Editar template",
      newTitle: "Nuevo template",
      name: "Nombre",
      accent: "Color acento",
      logo: "Logo",
      headerHtml: "Header HTML",
      bodyHtml: "Body HTML",
      footerHtml: "Footer HTML",
      variablesJson: "Variables (JSON)",
      listTitle: "Lista",
      previewTitle: "Preview",
      rendering: "Renderizando...",
      nameRequired: "El nombre es requerido",
      deleteConfirm: (name) => `Eliminar template "${name}"?`,
      empty: "No hay templates.",
      updatedAt: "Actualizado",
    },
  },
  en: {
    nav: {
      products: "Products",
      quotes: "Quotations",
      templates: "Templates",
    },
    common: {
      refresh: "Refresh",
      save: "Save",
      create: "Create",
      cancel: "Cancel",
      new: "New",
      delete: "Delete",
      edit: "Edit",
      loading: "Loading...",
      search: "Search",
      clear: "Clear",
      searching: "Searching...",
      noResults: "No results.",
      optional: "Optional",
    },
    theme: {
      light: "Light mode",
      dark: "Dark mode",
    },
    language: {
      label: "Language",
      quickSwitch: "Switch language",
    },
    products: {
      title: "Products",
      subtitle: "Manage your catalog for quotations.",
      searchPlaceholder: "Search by name or SKU",
      formTitleNew: "New product",
      formTitleEdit: "Edit product",
      sku: "SKU",
      skuOptional: "Optional",
      name: "Name",
      namePlaceholder: "e.g. Design service",
      description: "Description",
      priceCents: "Price (cents)",
      currency: "Currency",
      listTitle: "List",
      empty: "No products.",
      nameRequired: "Name is required",
      deleteConfirm: (name) => `Delete product "${name}"?`,
      price: "Price",
      actions: "",
    },
    quotes: {
      title: "Quotations",
      subtitle: "Build a quotation, save it and consult history.",
      searchPlaceholder: "Search by folio or customer",
      newTitle: "New quotation",
      savedTitle: "Saved",
      detailTitle: "Details",
      folio: "Folio",
      taxBps: "Tax (bps)",
      customer: "Customer",
      email: "Email",
      notes: "Notes",
      addProducts: "Add products",
      productSearch: "Search products to add",
      qty: "Qty",
      unitPriceCents: "Price (cents)",
      total: "Total",
      remove: "Remove",
      subtotal: "Subtotal",
      taxes: "Taxes",
      saveQuote: "Save quotation",
      empty: "No quotations.",
      emptyItems: "No items.",
      withoutCustomer: "(no customer)",
      addAtLeastOne: "Add at least one product",
      exportPdf: "Export PDF",
      exportHtml: "Export HTML",
      exporting: "Exporting...",
      exportDone: "Export ready",
    },
    templates: {
      title: "Templates",
      subtitle: "Edit HTML and preview with variables.",
      searchPlaceholder: "Search templates",
      editTitle: "Edit template",
      newTitle: "New template",
      name: "Name",
      accent: "Accent color",
      logo: "Logo",
      headerHtml: "Header HTML",
      bodyHtml: "Body HTML",
      footerHtml: "Footer HTML",
      variablesJson: "Variables (JSON)",
      listTitle: "List",
      previewTitle: "Preview",
      rendering: "Rendering...",
      nameRequired: "Name is required",
      deleteConfirm: (name) => `Delete template "${name}"?`,
      empty: "No templates.",
      updatedAt: "Updated",
    },
  },
};

function getByPath(obj, path) {
  const parts = String(path || "").split(".").filter(Boolean);
  let cur = obj;
  for (const p of parts) {
    cur = cur?.[p];
    if (cur === undefined || cur === null) return undefined;
  }
  return cur;
}

function normalizeLanguage(code) {
  const c = String(code || "").toLowerCase();
  const base = c.split("-")[0];
  return SUPPORTED_LANGUAGES.some((l) => l.code === base) ? base : "es";
}

export const language = ref("es");

export function t(key, ...args) {
  const lang = language.value;
  const entry = getByPath(translations?.[lang], key) ?? getByPath(translations?.es, key);
  if (typeof entry === "function") return entry(...args);
  if (typeof entry === "string") return entry;
  return String(key);
}

async function loadLanguageFromSettings() {
  try {
    const v = await invoke("get_setting", { key: "language" });
    if (v) return String(v);
  } catch {
    // ignore
  }
  return null;
}

async function saveLanguageToSettings(code) {
  try {
    await invoke("set_setting", { key: "language", value: code });
  } catch {
    // ignore
  }
}

export async function initLanguage() {
  const storedLocal = window.localStorage.getItem("language");
  const storedDb = await loadLanguageFromSettings();
  const detected = navigator.language || navigator.userLanguage || "es";

  const initial = normalizeLanguage(storedDb || storedLocal || detected);
  language.value = initial;
  window.localStorage.setItem("language", initial);
  await saveLanguageToSettings(initial);
}

export async function setLanguage(code) {
  const normalized = normalizeLanguage(code);
  language.value = normalized;
  window.localStorage.setItem("language", normalized);
  await saveLanguageToSettings(normalized);
}

export function toggleLanguage() {
  const next = language.value === "es" ? "en" : "es";
  setLanguage(next);
}
