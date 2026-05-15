<script setup>
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DataTable from "../components/DataTable.vue";
import ProductAutocomplete from "../components/ProductAutocomplete.vue";
import { save } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { language, t } from "../i18n";

const quoteLoading = ref(false);
const quoteErrorMsg = ref("");

const quoteSearch = ref("");
const quotes = ref([]);
const selectedQuoteId = ref(null);
const selectedQuote = ref(null);

const exporting = ref(false);

const quoteForm = ref({
  quote_number: "",
  customer_name: "",
  customer_email: "",
  notes: "",
  tax_rate_bps: 1600,
  currency: "MXN",
});

const quoteItems = ref([]);

const quoteColumns = computed(() => [
  { key: "folio", label: t("quotes.folio"), width: "150px" },
  { key: "customer", label: t("quotes.customer") },
  { key: "total", label: t("quotes.total"), width: "160px" },
]);

const quoteItemColumns = computed(() => [
  { key: "name", label: t("products.name") },
  { key: "qty", label: t("quotes.qty"), width: "90px" },
  { key: "unit", label: t("products.price"), width: "160px" },
  { key: "line", label: t("quotes.total"), width: "160px" },
]);

function formatMoney(cents) {
  const v = Number(cents || 0) / 100;
  return v.toFixed(2);
}

async function exportSelectedQuoteHtml() {
  if (!selectedQuote.value?.quote?.id) return;
  exporting.value = true;
  quoteErrorMsg.value = "";
  try {
    const folio = selectedQuote.value.quote.quote_number || `Q-${selectedQuote.value.quote.id}`;
    const path = await save({
      defaultPath: `cotizacion-${folio}.html`,
      filters: [{ name: "HTML", extensions: ["html"] }],
    });
    if (!path) return;

    await invoke("export_quote_html", {
      quoteId: selectedQuote.value.quote.id,
      langCode: language.value,
      outputPath: path,
    });

    await openPath(path);
  } catch (e) {
    quoteErrorMsg.value = String(e);
  } finally {
    exporting.value = false;
  }
}

async function exportSelectedQuotePdf() {
  if (!selectedQuote.value?.quote?.id) return;
  exporting.value = true;
  quoteErrorMsg.value = "";
  try {
    const folio = selectedQuote.value.quote.quote_number || `Q-${selectedQuote.value.quote.id}`;
    const path = await save({
      defaultPath: `cotizacion-${folio}.pdf`,
      filters: [{ name: "PDF", extensions: ["pdf"] }],
    });
    if (!path) return;

    await invoke("export_quote_pdf", {
      quoteId: selectedQuote.value.quote.id,
      langCode: language.value,
      outputPath: path,
    });

    await openPath(path);
  } catch (e) {
    quoteErrorMsg.value = String(e);
  } finally {
    exporting.value = false;
  }
}

const quoteSubtotalCents = computed(() => {
  return quoteItems.value.reduce((acc, it) => {
    const qty = Number(it.quantity || 0);
    const unit = Number(it.unit_price_cents || 0);
    return acc + qty * unit;
  }, 0);
});

const quoteTaxCents = computed(() => {
  const rate = Number(quoteForm.value.tax_rate_bps || 0);
  const subtotal = Number(quoteSubtotalCents.value || 0);
  return Math.floor((subtotal * rate + 5000) / 10000);
});

const quoteTotalCents = computed(() => {
  return Number(quoteSubtotalCents.value || 0) + Number(quoteTaxCents.value || 0);
});

function resetQuoteBuilder() {
  quoteForm.value = {
    quote_number: "",
    customer_name: "",
    customer_email: "",
    notes: "",
    tax_rate_bps: 1600,
    currency: "MXN",
  };
  quoteItems.value = [];
}

async function refreshQuotes() {
  quoteLoading.value = true;
  quoteErrorMsg.value = "";
  try {
    const s = quoteSearch.value.trim();
    quotes.value = await invoke("list_quotes", { search: s.length ? s : null });
  } catch (e) {
    quoteErrorMsg.value = String(e);
  } finally {
    quoteLoading.value = false;
  }
}

async function loadQuoteDetails(id) {
  quoteLoading.value = true;
  quoteErrorMsg.value = "";
  try {
    selectedQuoteId.value = id;
    selectedQuote.value = await invoke("get_quote", { id });
  } catch (e) {
    quoteErrorMsg.value = String(e);
  } finally {
    quoteLoading.value = false;
  }
}

function addProductToQuote(p) {
  quoteItems.value.push({
    product_id: p.id,
    sku: p.sku ?? null,
    name: p.name ?? "",
    description: p.description ?? null,
    quantity: 1,
    unit_price_cents: Number(p.unit_price_cents ?? 0),
    currency: p.currency ?? quoteForm.value.currency ?? "MXN",
  });
}

function removeQuoteItem(idx) {
  quoteItems.value.splice(idx, 1);
}

async function saveQuote() {
  quoteLoading.value = true;
  quoteErrorMsg.value = "";
  try {
    if (!quoteItems.value.length) {
      quoteErrorMsg.value = t("quotes.addAtLeastOne");
      return;
    }

    const payload = {
      quote_number: quoteForm.value.quote_number.trim() ? quoteForm.value.quote_number.trim() : null,
      customer_name: quoteForm.value.customer_name.trim() ? quoteForm.value.customer_name.trim() : null,
      customer_email: quoteForm.value.customer_email.trim() ? quoteForm.value.customer_email.trim() : null,
      notes: quoteForm.value.notes.trim() ? quoteForm.value.notes.trim() : null,
      tax_rate_bps: Number(quoteForm.value.tax_rate_bps || 0),
      currency: quoteForm.value.currency.trim() ? quoteForm.value.currency.trim() : null,
      status: "draft",
      template_id: null,
      items: quoteItems.value.map((it) => ({
        product_id: it.product_id ?? null,
        sku: it.sku && String(it.sku).trim().length ? String(it.sku).trim() : null,
        name: String(it.name || "").trim(),
        description: it.description && String(it.description).trim().length ? String(it.description).trim() : null,
        quantity: Number(it.quantity || 0),
        unit_price_cents: Number(it.unit_price_cents || 0),
        currency: it.currency && String(it.currency).trim().length ? String(it.currency).trim() : null,
      })),
    };

    const created = await invoke("create_quote", { input: payload });
    await refreshQuotes();
    resetQuoteBuilder();
    if (created?.quote?.id) {
      await loadQuoteDetails(created.quote.id);
    }
  } catch (e) {
    quoteErrorMsg.value = String(e);
  } finally {
    quoteLoading.value = false;
  }
}

onMounted(async () => {
  await refreshQuotes();
});
</script>

<template>
  <section>
    <div class="app-header">
      <div>
        <h1>{{ t("quotes.title") }}</h1>
        <p class="muted">{{ t("quotes.subtitle") }}</p>
      </div>

      <div class="header-actions">
        <input v-model="quoteSearch" class="search" :placeholder="t('quotes.searchPlaceholder')" @input="refreshQuotes" />
        <button type="button" @click="refreshQuotes" :disabled="quoteLoading">{{ t("common.refresh") }}</button>
      </div>
    </div>

    <div v-if="quoteErrorMsg" class="error">{{ quoteErrorMsg }}</div>

    <div class="grid-quotes">
      <section class="card">
        <div class="card-title">{{ t("quotes.newTitle") }}</div>

        <form class="form" @submit.prevent="saveQuote">
          <div class="row-2">
            <label>
              {{ t("quotes.folio") }}
              <input v-model="quoteForm.quote_number" placeholder="(auto si vacío)" />
            </label>

            <label>
              {{ t("quotes.taxBps") }}
              <input v-model.number="quoteForm.tax_rate_bps" type="number" min="0" />
            </label>
          </div>

          <div class="row-2">
            <label>
              {{ t("quotes.customer") }}
              <input v-model="quoteForm.customer_name" :placeholder="t('products.skuOptional')" />
            </label>
            <label>
              {{ t("quotes.email") }}
              <input v-model="quoteForm.customer_email" :placeholder="t('products.skuOptional')" />
            </label>
          </div>

          <label>
            {{ t("quotes.notes") }}
            <textarea v-model="quoteForm.notes" rows="3" :placeholder="t('products.skuOptional')" />
          </label>

          <div class="row-2">
            <label>
              {{ t("products.currency") }}
              <input v-model="quoteForm.currency" placeholder="MXN" />
            </label>
            <div></div>
          </div>

          <div class="card-title" style="margin-top: 8px">{{ t("quotes.addProducts") }}</div>

          <ProductAutocomplete
            :disabled="quoteLoading"
            :placeholder="t('quotes.productSearch')"
            :loading-text="t('common.searching')"
            :empty-text="t('common.noResults')"
            @select="addProductToQuote"
          />

          <div v-if="quoteItems.length" class="quote-items">
            <div class="quote-item" v-for="(it, idx) in quoteItems" :key="`qi-${idx}`">
              <div class="quote-item-main">
                <div class="item-title">
                  {{ it.name }}
                  <span v-if="it.sku" class="pill">{{ it.sku }}</span>
                </div>
                <div class="item-sub">{{ it.currency }} {{ formatMoney(it.unit_price_cents) }}</div>
              </div>

              <div class="quote-item-edit">
                <label>
                  {{ t("quotes.qty") }}
                  <input v-model.number="it.quantity" type="number" min="1" />
                </label>
                <label>
                  {{ t("quotes.unitPriceCents") }}
                  <input v-model.number="it.unit_price_cents" type="number" min="0" />
                </label>
                <label>
                  {{ t("quotes.total") }}
                  <input :value="formatMoney((it.quantity || 0) * (it.unit_price_cents || 0))" disabled />
                </label>
                <button type="button" class="danger" @click="removeQuoteItem(idx)">{{ t("quotes.remove") }}</button>
              </div>
            </div>
          </div>

          <div class="totals">
            <div class="totals-row">
              <div class="muted">{{ t("quotes.subtotal") }}</div>
              <div>{{ quoteForm.currency }} {{ formatMoney(quoteSubtotalCents) }}</div>
            </div>
            <div class="totals-row">
              <div class="muted">{{ t("quotes.taxes") }}</div>
              <div>{{ quoteForm.currency }} {{ formatMoney(quoteTaxCents) }}</div>
            </div>
            <div class="totals-row total">
              <div>{{ t("quotes.total") }}</div>
              <div>{{ quoteForm.currency }} {{ formatMoney(quoteTotalCents) }}</div>
            </div>
          </div>

          <div class="actions">
            <button type="submit" :disabled="quoteLoading">{{ t("quotes.saveQuote") }}</button>
            <button type="button" class="secondary" @click="resetQuoteBuilder" :disabled="quoteLoading">{{ t("common.clear") }}</button>
          </div>
        </form>
      </section>

      <section class="card">
        <div class="card-title">{{ t("quotes.savedTitle") }}</div>

        <DataTable
          :columns="quoteColumns"
          :rows="quotes"
          :loading="quoteLoading"
          :loading-text="t('common.loading')"
          :empty-text="t('quotes.empty')"
          clickable
          row-key="id"
          :selected-key="selectedQuoteId"
          @rowClick="(row) => loadQuoteDetails(row.id)"
        >
          <template #cell-folio="{ row }">
            <div class="item-title">
              {{ row.quote_number || `Q-${row.id}` }}
              <span class="pill">{{ row.status }}</span>
            </div>
          </template>

          <template #cell-customer="{ row }">{{ row.customer_name || t("quotes.withoutCustomer") }}</template>

          <template #cell-total="{ row }">
            {{ row.currency }} {{ formatMoney(row.total_cents) }}
          </template>
        </DataTable>
      </section>

      <section class="card" v-if="selectedQuote">
        <div class="card-title">{{ t("quotes.detailTitle") }}</div>

        <div class="actions" style="margin-bottom: 10px">
          <button type="button" @click="exportSelectedQuotePdf" :disabled="quoteLoading || exporting">
            {{ exporting ? t("quotes.exporting") : t("quotes.exportPdf") }}
          </button>
          <button type="button" class="secondary" @click="exportSelectedQuoteHtml" :disabled="quoteLoading || exporting">
            {{ exporting ? t("quotes.exporting") : t("quotes.exportHtml") }}
          </button>
        </div>

        <div class="detail">
          <div class="detail-row">
            <div class="muted">{{ t("quotes.folio") }}</div>
            <div>{{ selectedQuote.quote.quote_number || `Q-${selectedQuote.quote.id}` }}</div>
          </div>
          <div class="detail-row">
            <div class="muted">{{ t("quotes.customer") }}</div>
            <div>{{ selectedQuote.quote.customer_name || t("quotes.withoutCustomer") }}</div>
          </div>
          <div class="detail-row">
            <div class="muted">{{ t("quotes.email") }}</div>
            <div>{{ selectedQuote.quote.customer_email || "" }}</div>
          </div>
        </div>

        <div style="margin-top: 10px">
          <DataTable
            :columns="quoteItemColumns"
            :rows="selectedQuote.items"
            :loading="quoteLoading"
            :loading-text="t('common.loading')"
            :empty-text="t('quotes.emptyItems')"
          >
            <template #cell-name="{ row }">
              <div class="item-title">
                {{ row.name }}
                <span v-if="row.sku" class="pill">{{ row.sku }}</span>
              </div>
            </template>
            <template #cell-qty="{ row }">{{ row.quantity }}</template>
            <template #cell-unit="{ row }">
              {{ selectedQuote.quote.currency }} {{ formatMoney(row.unit_price_cents) }}
            </template>
            <template #cell-line="{ row }">
              {{ selectedQuote.quote.currency }} {{ formatMoney(row.line_total_cents) }}
            </template>
          </DataTable>
        </div>

        <div class="totals" style="margin-top: 10px">
          <div class="totals-row">
            <div class="muted">{{ t("quotes.subtotal") }}</div>
            <div>{{ selectedQuote.quote.currency }} {{ formatMoney(selectedQuote.quote.subtotal_cents) }}</div>
          </div>
          <div class="totals-row">
            <div class="muted">{{ t("quotes.taxes") }}</div>
            <div>{{ selectedQuote.quote.currency }} {{ formatMoney(selectedQuote.quote.tax_cents) }}</div>
          </div>
          <div class="totals-row total">
            <div>{{ t("quotes.total") }}</div>
            <div>{{ selectedQuote.quote.currency }} {{ formatMoney(selectedQuote.quote.total_cents) }}</div>
          </div>
        </div>
      </section>
    </div>
  </section>
</template>
