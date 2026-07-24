<script setup>
import { computed, onMounted, ref } from "vue";
import ProductsView from "./views/ProductsView.vue";
import QuotesView from "./views/QuotesView.vue";
import TemplatesView from "./views/TemplatesView.vue";
import {
  SUPPORTED_LANGUAGES,
  initLanguage,
  language,
  setLanguage,
  t,
  toggleLanguage,
} from "./i18n";

const route = ref("products");
const isDark = ref(false);
 
const routes = computed(() => [
  { key: "products", label: t("nav.products"), component: ProductsView },
  { key: "quotes", label: t("nav.quotes"), component: QuotesView },
  { key: "templates", label: t("nav.templates"), component: TemplatesView },
]);
 
const currentComponent = computed(() => {
  return routes.value.find((r) => r.key === route.value)?.component ?? ProductsView;
});

function readRouteFromHash() {
  const raw = String(window.location.hash || "").replace(/^#/, "").trim();
  const hit = routes.value.find((r) => r.key === raw);
  route.value = hit ? hit.key : "products";
}

function navigate(to) {
  if (!routes.value.some((r) => r.key === to)) return;
  window.location.hash = to;
}

function onLanguageChange(evt) {
  const next = evt?.target?.value;
  setLanguage(next);
}
 
function applyTheme() {
  const root = document.documentElement;
  if (isDark.value) root.classList.add("dark");
  else root.classList.remove("dark");
}

function toggleTheme() {
   isDark.value = !isDark.value;
   window.localStorage.setItem("theme", isDark.value ? "dark" : "light");
   applyTheme();
 }
 
 onMounted(async () => {
   await initLanguage();

   const stored = window.localStorage.getItem("theme");
   if (stored === "dark") isDark.value = true;
   else if (stored === "light") isDark.value = false;
   else isDark.value = window.matchMedia?.("(prefers-color-scheme: dark)")?.matches ?? false;

   applyTheme();
   readRouteFromHash();
   window.addEventListener("hashchange", readRouteFromHash);
 });
</script>

<template>
  <main class="container">
    <div class="app-shell">
      <aside class="sidebar">
        <div class="brand-lockup">
          <div class="brand-mark" aria-hidden="true"><span>V</span></div>
          <div>
            <div class="brand">Vaestra</div>
            <div class="brand-subtitle">Cotizador</div>
          </div>
        </div>

        <nav class="nav">
          <button
            v-for="r in routes"
            :key="`nav-${r.key}`"
            type="button"
            class="nav-btn"
            :data-active="route === r.key"
            @click="navigate(r.key)"
          >
            {{ r.label }}
          </button>
        </nav>

        <div>
          <div class="muted" style="margin-bottom: 6px">{{ t("language.label") }}</div>
          <select :value="language" @change="onLanguageChange" style="width: 100%">
            <option v-for="l in SUPPORTED_LANGUAGES" :key="`lang-${l.code}`" :value="l.code">
              {{ l.label }}
            </option>
          </select>
          <button type="button" class="secondary" style="margin-top: 8px; width: 100%" @click="toggleLanguage">
            {{ t("language.quickSwitch") }}
          </button>
        </div>

        <div class="sidebar-footer">
          <button type="button" class="secondary" @click="toggleTheme">
            {{ isDark ? t("theme.light") : t("theme.dark") }}
          </button>
        </div>
      </aside>

      <section class="content">
        <component :is="currentComponent" />
      </section>
    </div>
  </main>
</template>

<style scoped>
.brand-lockup {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 4px 4px 16px;
  border-bottom: 1px solid var(--border);
}
.brand-mark {
  width: 44px;
  height: 44px;
  border-radius: 14px 14px 14px 5px;
  display: grid;
  place-items: center;
  color: white;
  font-size: 22px;
  font-weight: 900;
  background: linear-gradient(145deg, var(--brand-blue) 0 66%, var(--brand-red) 66% 100%);
  box-shadow: 0 10px 22px rgba(11, 58, 130, .24);
}
.brand {
  font-weight: 800;
  letter-spacing: -0.5px;
  line-height: 1;
}
.brand-subtitle {
  margin-top: 4px;
  color: var(--brand-red);
  font-size: 11px;
  font-weight: 800;
  letter-spacing: .16em;
  text-transform: uppercase;
}
</style>
