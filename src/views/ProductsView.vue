<script setup>
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "../components/DataTable.vue";
import { t } from "../i18n";

const loading = ref(false);
const errorMsg = ref("");

const search = ref("");
const products = ref([]);

const editingId = ref(null);
const form = ref({
  sku: "",
  name: "",
  description: "",
  unit_price_cents: 0,
  currency: "MXN",
});

const isEditing = computed(() => editingId.value !== null);

const columns = computed(() => [
  { key: "name", label: t("products.name") },
  { key: "sku", label: t("products.sku"), width: "110px" },
  { key: "price", label: t("products.price"), width: "160px" },
  { key: "actions", label: t("products.actions"), width: "180px" },
]);

function formatMoney(cents) {
  const v = Number(cents || 0) / 100;
  return v.toFixed(2);
}

function resetForm() {
  editingId.value = null;
  form.value = {
    sku: "",
    name: "",
    description: "",
    unit_price_cents: 0,
    currency: "MXN",
  };
}

async function refreshProducts() {
  loading.value = true;
  errorMsg.value = "";
  try {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const s = search.value.trim();
    products.value = await invoke("list_products", {
      search: s.length ? s : null,
    });
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    loading.value = false;
  }
}

function startEdit(p) {
  editingId.value = p.id;
  form.value = {
    sku: p.sku ?? "",
    name: p.name ?? "",
    description: p.description ?? "",
    unit_price_cents: Number(p.unit_price_cents ?? 0),
    currency: p.currency ?? "MXN",
  };
}

async function saveProduct() {
  loading.value = true;
  errorMsg.value = "";
  try {
    const payload = {
      sku: form.value.sku.trim() ? form.value.sku.trim() : null,
      name: form.value.name.trim(),
      description: form.value.description.trim() ? form.value.description.trim() : null,
      unit_price_cents: Number(form.value.unit_price_cents || 0),
      currency: form.value.currency.trim() ? form.value.currency.trim() : null,
    };

    if (!payload.name.length) {
      errorMsg.value = t("products.nameRequired");
      return;
    }

    if (isEditing.value) {
      await invoke("update_product", {
        id: editingId.value,
        input: payload,
      });
    } else {
      await invoke("create_product", { input: payload });
    }

    resetForm();
    await refreshProducts();
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function removeProduct(p) {
  if (!confirm(t("products.deleteConfirm", p.name))) return;
  loading.value = true;
  errorMsg.value = "";
  try {
    await invoke("delete_product", { id: p.id });
    await refreshProducts();
    if (editingId.value === p.id) resetForm();
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    loading.value = false;
  }
}

onMounted(async () => {
  await refreshProducts();
});
</script>

<template>
  <section>
    <div class="app-header">
      <div>
        <h1>{{ t("products.title") }}</h1>
        <p class="muted">{{ t("products.subtitle") }}</p>
      </div>

      <div class="header-actions">
        <input v-model="search" class="search" :placeholder="t('products.searchPlaceholder')" @input="refreshProducts" />
        <button type="button" @click="refreshProducts" :disabled="loading">{{ t("common.refresh") }}</button>
      </div>
    </div>

    <div v-if="errorMsg" class="error">{{ errorMsg }}</div>

    <div class="grid">
      <section class="card">
        <div class="card-title">{{ isEditing ? t("products.formTitleEdit") : t("products.formTitleNew") }}</div>

        <form class="form" @submit.prevent="saveProduct">
          <label>
            {{ t("products.sku") }}
            <input v-model="form.sku" :placeholder="t('products.skuOptional')" />
          </label>

          <label>
            {{ t("products.name") }}
            <input v-model="form.name" :placeholder="t('products.namePlaceholder')" />
          </label>

          <label>
            {{ t("products.description") }}
            <textarea v-model="form.description" rows="4" :placeholder="t('products.skuOptional')" />
          </label>

          <div class="row-2">
            <label>
              {{ t("products.priceCents") }}
              <input v-model.number="form.unit_price_cents" type="number" min="0" />
            </label>

            <label>
              {{ t("products.currency") }}
              <input v-model="form.currency" placeholder="MXN" />
            </label>
          </div>

          <div class="actions">
            <button type="submit" :disabled="loading">{{ isEditing ? t("common.save") : t("common.create") }}</button>
            <button type="button" class="secondary" @click="resetForm" :disabled="loading">{{ t("common.cancel") }}</button>
          </div>
        </form>
      </section>

      <section class="card">
        <div class="card-title">{{ t("products.listTitle") }}</div>

        <DataTable
          :columns="columns"
          :rows="products"
          :loading="loading"
          :loading-text="t('common.loading')"
          :empty-text="t('products.empty')"
        >
          <template #cell-sku="{ row }">
            <span v-if="row.sku" class="pill">{{ row.sku }}</span>
            <span v-else class="muted">-</span>
          </template>

          <template #cell-price="{ row }">{{ row.currency }} {{ formatMoney(row.unit_price_cents) }}</template>

          <template #cell-actions="{ row }">
            <div class="actions">
              <button type="button" class="secondary" @click="startEdit(row)" :disabled="loading">{{ t("common.edit") }}</button>
              <button type="button" class="danger" @click="removeProduct(row)" :disabled="loading">{{ t("common.delete") }}</button>
            </div>
          </template>
        </DataTable>
      </section>
    </div>
  </section>
</template>
