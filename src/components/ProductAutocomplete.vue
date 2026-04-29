<script setup>
import { onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  placeholder: { type: String, default: "Buscar producto" },
  disabled: { type: Boolean, default: false },
  minChars: { type: Number, default: 1 },
  debounceMs: { type: Number, default: 250 },
  clearOnSelect: { type: Boolean, default: true },
});

const emit = defineEmits(["select"]);

const rootEl = ref(null);
const query = ref("");
const loading = ref(false);
const error = ref("");
const open = ref(false);
const results = ref([]);
let timer = null;

async function runSearch() {
  const s = String(query.value || "").trim();
  if (s.length < props.minChars) {
    results.value = [];
    loading.value = false;
    error.value = "";
    return;
  }

  loading.value = true;
  error.value = "";
  try {
    results.value = await invoke("list_products", { search: s.length ? s : null });
  } catch (e) {
    error.value = String(e);
    results.value = [];
  } finally {
    loading.value = false;
  }
}

function scheduleSearch() {
  if (timer) clearTimeout(timer);
  timer = setTimeout(runSearch, props.debounceMs);
}

function pick(p) {
  emit("select", p);
  open.value = false;
  results.value = [];
  if (props.clearOnSelect) query.value = "";
}

function onDocMouseDown(evt) {
  const el = rootEl.value;
  if (!el) return;
  if (!el.contains(evt.target)) {
    open.value = false;
  }
}

onMounted(() => {
  document.addEventListener("mousedown", onDocMouseDown);
});

onUnmounted(() => {
  document.removeEventListener("mousedown", onDocMouseDown);
});
</script>

<template>
  <div ref="rootEl" class="pa" :data-disabled="disabled">
    <input
      v-model="query"
      class="search"
      :placeholder="placeholder"
      :disabled="disabled"
      @focus="open = true"
      @input="
        open = true;
        scheduleSearch();
      "
    />

    <div v-if="open" class="pa-panel">
      <div v-if="error" class="error" style="margin: 8px">{{ error }}</div>
      <div v-else-if="loading" class="muted" style="padding: 8px 10px">Buscando...</div>
      <div v-else-if="!results.length" class="muted" style="padding: 8px 10px">Sin resultados.</div>
      <button v-for="p in results" :key="`pa-${p.id}`" class="pa-item" type="button" @click="pick(p)">
        <div class="pa-title">
          <span>{{ p.name }}</span>
          <span v-if="p.sku" class="pill">{{ p.sku }}</span>
        </div>
        <div class="pa-sub">{{ p.currency }} {{ (Number(p.unit_price_cents || 0) / 100).toFixed(2) }}</div>
      </button>
    </div>
  </div>
</template>

<style scoped>
.pa {
  position: relative;
  width: 100%;
}

.pa-panel {
  position: absolute;
  z-index: 50;
  top: calc(100% + 6px);
  left: 0;
  right: 0;
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 12px;
  box-shadow: var(--shadow);
  max-height: 260px;
  overflow: auto;
}

.pa-item {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 2px;
  border: none;
  border-bottom: 1px solid var(--border);
  border-radius: 0;
  background: transparent;
  padding: 10px 12px;
  box-shadow: none;
}

.pa-item:hover {
  border-color: transparent;
  background: rgba(57, 108, 216, 0.08);
}

.pa-item:last-child {
  border-bottom: none;
}

.pa-title {
  display: flex;
  gap: 8px;
  align-items: center;
  font-weight: 600;
}

.pa-sub {
  font-size: 0.9em;
  color: var(--muted);
}
</style>
