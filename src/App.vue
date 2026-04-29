<script setup>
 import { computed, onMounted, ref } from "vue";
 import ProductsView from "./views/ProductsView.vue";
 import QuotesView from "./views/QuotesView.vue";
 import TemplatesView from "./views/TemplatesView.vue";
 
 const route = ref("products");
 const isDark = ref(false);
 
 const routes = [
   { key: "products", label: "Productos", component: ProductsView },
   { key: "quotes", label: "Cotizaciones", component: QuotesView },
   { key: "templates", label: "Templates", component: TemplatesView },
 ];
 
 const currentComponent = computed(() => {
   return routes.find((r) => r.key === route.value)?.component ?? ProductsView;
 });
 
 function readRouteFromHash() {
   const raw = String(window.location.hash || "").replace(/^#/, "").trim();
   const hit = routes.find((r) => r.key === raw);
   route.value = hit ? hit.key : "products";
 }
 
 function navigate(to) {
   if (!routes.some((r) => r.key === to)) return;
   window.location.hash = to;
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
 
 onMounted(() => {
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
        <div class="brand">Quotauri</div>

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

        <div class="sidebar-footer">
          <button type="button" class="secondary" @click="toggleTheme">
            {{ isDark ? "Modo claro" : "Modo oscuro" }}
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
.brand {
  font-weight: 800;
  letter-spacing: 0.2px;
}
</style>
