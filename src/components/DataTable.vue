<script setup>
const props = defineProps({
  columns: { type: Array, required: true },
  rows: { type: Array, required: true },
  rowKey: { type: [String, Function], default: "id" },
  loading: { type: Boolean, default: false },
  loadingText: { type: String, default: "Cargando..." },
  emptyText: { type: String, default: "Sin registros." },
  clickable: { type: Boolean, default: false },
  selectedKey: { type: [String, Number, null], default: null },
});

const emit = defineEmits(["rowClick"]);

function keyForRow(row, idx) {
  if (typeof props.rowKey === "function") return props.rowKey(row, idx);
  if (typeof props.rowKey === "string") return row?.[props.rowKey] ?? idx;
  return idx;
}

function onRowClick(row, idx) {
  if (!props.clickable) return;
  emit("rowClick", row, idx);
}
</script>

<template>
  <div class="dt">
    <div v-if="loading" class="muted">{{ loadingText }}</div>
    <div v-else-if="!rows.length" class="muted">{{ emptyText }}</div>

    <table v-else class="dt-table">
      <thead>
        <tr>
          <th v-for="col in columns" :key="`h-${String(col.key)}`" :style="col.width ? { width: col.width } : null">
            {{ col.label ?? "" }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(row, idx) in rows"
          :key="`r-${String(keyForRow(row, idx))}`"
          :data-clickable="clickable"
          :data-selected="selectedKey !== null && selectedKey === keyForRow(row, idx)"
          @click="onRowClick(row, idx)"
        >
          <td v-for="col in columns" :key="`c-${String(col.key)}-${String(keyForRow(row, idx))}`">
            <slot :name="`cell-${String(col.key)}`" :row="row" :value="row?.[col.key]" :index="idx">
              {{ row?.[col.key] ?? "" }}
            </slot>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.dt {
  width: 100%;
}

.dt-table {
  width: 100%;
  border-collapse: collapse;
  border: 1px solid var(--border);
  border-radius: 12px;
  overflow: hidden;
}

th,
td {
  text-align: left;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
  vertical-align: top;
}

th {
  font-size: 0.85em;
  color: var(--muted);
  background: rgba(0, 0, 0, 0.02);
}

:root.dark th {
  background: rgba(255, 255, 255, 0.04);
}

tr[data-clickable="true"] {
  cursor: pointer;
}

tr[data-selected="true"] {
  outline: 2px solid rgba(57, 108, 216, 0.35);
  outline-offset: -2px;
}

tr:last-child td {
  border-bottom: none;
}
</style>
