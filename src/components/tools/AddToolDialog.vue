<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useToolsStore } from "@/stores/tools";
import { FolderPlus, Plus, Loader2 } from "lucide-vue-next";

const { t } = useI18n();
const toolsStore = useToolsStore();

const path = ref("");
const loading = ref(false);

async function handleBrowse() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const selected = await invoke<string>("browse_directory");
    if (selected) {
      path.value = selected;
    }
  } catch (error) {
    console.error("Failed to browse:", error);
  }
}

async function handleAdd() {
  if (!path.value.trim()) return;

  loading.value = true;
  try {
    await toolsStore.addCustomTool(path.value.trim());
    path.value = "";
  } catch (error) {
    console.error("Failed to add tool:", error);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="rounded-xl border border-dashed border-[var(--color-border)] p-4 text-center">
    <p class="mb-3 text-sm text-[var(--color-text-secondary)]">{{ t("tools.addCustom") }}</p>
    <div class="flex items-center gap-2">
      <input
        v-model="path"
        type="text"
        :placeholder="t('tools.customToolPathPlaceholder')"
        class="input flex-1"
      />
      <button
        @click="handleBrowse"
        class="btn btn-secondary"
      >
        <FolderPlus class="h-4 w-4" :stroke-width="1.5" />
        {{ t("tools.browse") }}
      </button>
      <button
        @click="handleAdd"
        :disabled="!path.trim() || loading"
        class="btn btn-primary"
      >
        <Loader2 v-if="loading" class="h-4 w-4 animate-spin" :stroke-width="1.5" />
        <Plus v-else class="h-4 w-4" :stroke-width="1.5" />
        {{ t("tools.add") }}
      </button>
    </div>
  </div>
</template>
