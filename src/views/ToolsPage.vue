<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useToolsStore } from "@/stores/tools";
import ToolCard from "@/components/tools/ToolCard.vue";
import AddToolDialog from "@/components/tools/AddToolDialog.vue";
import EmptyState from "@/components/common/EmptyState.vue";
import PageHeader from "@/components/common/PageHeader.vue";
import { RefreshCw, ToggleLeft, ToggleRight, Wrench, Search, X } from "lucide-vue-next";

const { t } = useI18n();
const toolsStore = useToolsStore();

const statistics = computed(() => {
  const disabledCount = toolsStore.tools.filter(t => !t.enabled).length;
  return [
    { label: t("tools.stats.total"), value: toolsStore.tools.length, highlight: true },
    { label: t("tools.stats.enabled"), value: toolsStore.enabledCount },
    { label: t("tools.stats.disabled"), value: disabledCount },
  ];
});

onMounted(() => {
  toolsStore.detectTools();
});

async function handleRefresh() {
  await toolsStore.detectTools();
}

async function handleEnableAll() {
  await toolsStore.enableAll();
}

async function handleDisableAll() {
  await toolsStore.disableAll();
}
</script>

<template>
  <div class="page">
    <PageHeader
      :title="t('tools.title')"
      :icon="Wrench"
      :statistics="statistics"
    >
      <template #actions>
        <button
          @click="handleRefresh"
          class="btn btn-secondary"
        >
          <RefreshCw class="h-4 w-4" :stroke-width="1.5" />
          {{ t("tools.refresh") }}
        </button>
        <button
          @click="handleEnableAll"
          class="btn btn-secondary"
        >
          <ToggleRight class="h-4 w-4" :stroke-width="1.5" />
          {{ t("tools.enableAll") }}
        </button>
        <button
          @click="handleDisableAll"
          class="btn btn-secondary"
        >
          <ToggleLeft class="h-4 w-4" :stroke-width="1.5" />
          {{ t("tools.disableAll") }}
        </button>
      </template>
    </PageHeader>

    <!-- 搜索框 -->
    <div class="toolbar">
      <div class="search-box">
        <Search class="search-icon" :stroke-width="1.5" />
        <input
          v-model="toolsStore.searchQuery"
          type="text"
          :placeholder="t('tools.searchPlaceholder')"
          class="input search-input"
        />
        <button
          v-if="toolsStore.searchQuery"
          @click="toolsStore.searchQuery = ''"
          class="search-clear"
        >
          <X class="h-4 w-4" :stroke-width="1.5" />
        </button>
      </div>
      <p class="page-description">
        {{ t("tools.detected", { count: toolsStore.filteredTools.length }) }}
      </p>
    </div>

    <div v-if="toolsStore.loading" class="loading-state">
      <div class="loading-spinner">
        <div class="spinner-dot"></div>
        <div class="spinner-dot"></div>
        <div class="spinner-dot"></div>
      </div>
      <span class="loading-text">{{ t("common.loading") }}</span>
    </div>

    <EmptyState
      v-else-if="toolsStore.tools.length === 0"
      :title="t('tools.noTools')"
      :description="t('tools.noToolsDesc')"
    />

    <div v-else class="tool-list">
      <ToolCard
        v-for="(tool, index) in toolsStore.filteredTools"
        :key="tool.id"
        :tool="tool"
        :style="{ animationDelay: String(index * 60) + 'ms' }"
        class="animate-fade-in"
      />

      <AddToolDialog />
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  flex-wrap: wrap;
}

.search-box {
  position: relative;
  flex: 1;
  max-width: 320px;
  min-width: 200px;
}

.search-icon {
  position: absolute;
  left: var(--spacing-md);
  top: 50%;
  transform: translateY(-50%);
  width: 18px;
  height: 18px;
  color: var(--color-text-tertiary);
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding-left: 40px;
  padding-right: 36px;
}

.search-clear {
  position: absolute;
  right: var(--spacing-sm);
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: var(--color-surface-subtle);
  border: none;
  border-radius: var(--radius-sm);
  color: var(--color-text-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.search-clear:hover {
  background: var(--strawberry-100);
  color: var(--strawberry);
}

.page-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-surface-subtle);
  border-radius: var(--radius-md);
  border: 2px solid var(--color-border-subtle);
  margin: 0;
}

.loading-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-lg);
  padding: var(--spacing-5xl);
}

.loading-spinner {
  display: flex;
  gap: var(--spacing-sm);
}

.spinner-dot {
  width: 12px;
  height: 12px;
  background: linear-gradient(135deg, var(--lavender-300), var(--lavender-400));
  border-radius: var(--radius-full);
  animation: bounce 1.4s ease-in-out infinite both;
}

.spinner-dot:nth-child(1) {
  animation-delay: -0.32s;
}

.spinner-dot:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0.6);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

.loading-text {
  color: var(--color-text-secondary);
  font-size: var(--text-base);
  font-weight: 500;
}

.tool-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: var(--spacing-lg);
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: var(--text-base);
  font-weight: 500;
  transition: all var(--transition-fast) var(--ease-out);
  border: 2px solid transparent;
  cursor: pointer;
  font-family: var(--font-body);
}

.btn-secondary {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  border-color: var(--color-border);
}

.btn-secondary:hover {
  background: var(--lavender-50);
  border-color: var(--lavender-200);
  color: var(--lavender-500);
}
</style>
