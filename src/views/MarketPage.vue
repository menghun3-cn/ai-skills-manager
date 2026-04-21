<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import type { MarketItem } from "@/types";
import { useMarketStore } from "@/stores/market";
import { useSkillsStore } from "@/stores/skills";
import MarketCard from "@/components/market/MarketCard.vue";
import MarketItemDetail from "@/components/market/MarketItemDetail.vue";
import SkeletonCard from "@/components/market/SkeletonCard.vue";
import EmptyState from "@/components/common/EmptyState.vue";
import PageHeader from "@/components/common/PageHeader.vue";
import { RefreshCw, Search, X, Store } from "lucide-vue-next";

const { t } = useI18n();
const marketStore = useMarketStore();
const skillsStore = useSkillsStore();

const selectedItem = ref<MarketItem | null>(null);

onMounted(() => {
  marketStore.fetchMarketData();
});

const hasActiveFilters = computed(() => {
  return marketStore.searchQuery || marketStore.filterSource !== "all";
});

const searchResultText = computed(() => {
  if (marketStore.searchQuery) {
    return t("market.searchResult", { query: marketStore.searchQuery });
  }
  return null;
});

const statistics = computed(() => {
  const installedCount = marketStore.items.filter(item => item.installed).length;
  return [
    { label: t("market.stats.total"), value: marketStore.items.length, highlight: true },
    { label: t("market.stats.installed"), value: installedCount },
  ];
});

async function handleRefresh() {
  await marketStore.fetchMarketData();
}

function clearFilters() {
  marketStore.searchQuery = "";
  marketStore.filterSource = "all";
}

function handleCardClick(item: MarketItem) {
  selectedItem.value = item;
}

function handleCloseDetail() {
  selectedItem.value = null;
}

async function handleInstall(itemId: string) {
  try {
    await marketStore.installSkill(itemId);
    await skillsStore.loadSkills();
  } catch (error) {
    console.error("[MarketPage] Install failed:", error);
    alert(String(error));
  }
}
</script>

<template>
  <div class="page">
    <PageHeader
      :title="t('market.title')"
      :icon="Store"
      :statistics="statistics"
    >
      <template #actions>
        <button
          @click="handleRefresh"
          :disabled="marketStore.loading"
          class="btn btn-secondary"
          :title="t('market.refresh')"
        >
          <RefreshCw
            class="h-4 w-4"
            :stroke-width="1.5"
            :class="{ 'animate-spin-slow': marketStore.loading }"
          />
        </button>
      </template>
    </PageHeader>

    <div class="toolbar">
      <div class="search-box">
        <Search class="search-icon" :stroke-width="1.5" />
        <input
          v-model="marketStore.searchQuery"
          type="text"
          :placeholder="t('market.searchPlaceholder')"
          class="input search-input"
        />
        <button
          v-if="marketStore.searchQuery"
          @click="marketStore.searchQuery = ''"
          class="search-clear"
        >
          <X class="h-4 w-4" :stroke-width="1.5" />
        </button>
      </div>

      <select
        v-model="marketStore.filterSource"
        class="input select-input"
      >
        <option value="all">{{ t("market.sourceAll") }}</option>
        <option value="awesome-claude-skills">{{ t("market.sourceAwesome") }}</option>
        <option value="skills.sh">{{ t("market.sourceSkillsSh") }}</option>
      </select>

      <button
        v-if="hasActiveFilters"
        @click="clearFilters"
        class="btn btn-ghost clear-btn"
      >
        <X class="h-3.5 w-3.5" :stroke-width="1.5" />
        {{ t("common.clear") }}
      </button>
    </div>

    <div v-if="searchResultText" class="search-result-text">
      {{ searchResultText }}
    </div>

    <div v-if="marketStore.loading" class="card-grid">
      <SkeletonCard v-for="i in 8" :key="i" />
    </div>

    <EmptyState
      v-else-if="marketStore.error"
      icon="alert"
      :title="t('market.loadError')"
      :description="String(marketStore.error)"
    >
      <button
        @click="handleRefresh"
        class="btn btn-primary mt-4"
      >
        <RefreshCw class="h-4 w-4" :stroke-width="1.5" />
        {{ t("market.retry") }}
      </button>
    </EmptyState>

    <EmptyState
      v-else-if="marketStore.filteredItems.length === 0 && hasActiveFilters"
      icon="search"
      :title="t('market.noResults')"
      :description="t('market.noResultsDesc')"
    >
      <button
        @click="clearFilters"
        class="btn btn-secondary mt-4"
      >
        {{ t("market.clearFilters") }}
      </button>
    </EmptyState>

    <EmptyState
      v-else-if="marketStore.items.length === 0"
      icon="package"
      :title="t('market.noSkills')"
      :description="t('market.noSkillsDesc')"
    >
      <button
        @click="handleRefresh"
        class="btn btn-primary mt-4"
      >
        <RefreshCw class="h-4 w-4" :stroke-width="1.5" />
        {{ t("market.refresh") }}
      </button>
    </EmptyState>

    <div
      v-else
      class="card-grid"
    >
      <MarketCard
        v-for="(item, index) in marketStore.filteredItems"
        :key="item.id"
        :item="item"
        :style="{ animationDelay: String(index * 50) + 'ms' }"
        class="animate-fade-in"
        @click="handleCardClick"
      />
    </div>

    <!-- 详情面板 -->
    <MarketItemDetail
      v-if="selectedItem"
      :item="selectedItem"
      @close="handleCloseDetail"
      @install="handleInstall"
    />
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
  padding: var(--spacing-lg);
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);
  border: 2px solid var(--color-border-subtle);
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
  transition: color var(--transition-fast) var(--ease-out);
}

.search-box:focus-within .search-icon {
  color: var(--honey);
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  padding-left: 40px;
  padding-right: 40px;
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  transition: all var(--transition-fast) var(--ease-out);
}

.search-input:hover {
  border-color: var(--cream-400);
}

.search-input:focus {
  outline: none;
  border-color: var(--honey);
  box-shadow: 0 0 0 4px rgba(255, 214, 102, 0.2);
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

.search-clear {
  position: absolute;
  right: var(--spacing-sm);
  top: 50%;
  transform: translateY(-50%);
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--color-text-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.search-clear:hover {
  color: var(--color-text-primary);
  background: var(--color-surface-subtle);
}

.select-input {
  width: auto;
  min-width: 160px;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.select-input:hover {
  border-color: var(--cream-400);
}

.select-input:focus {
  outline: none;
  border-color: var(--honey);
  box-shadow: 0 0 0 4px rgba(255, 214, 102, 0.2);
}

.clear-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  background: transparent;
  border: 2px solid var(--color-border);
  color: var(--color-text-secondary);
  font-size: var(--text-base);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  font-family: var(--font-body);
}

.clear-btn:hover {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  border-color: var(--cream-400);
}

.search-result-text {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-md) var(--spacing-lg);
  background: linear-gradient(135deg, rgba(255, 214, 102, 0.1), rgba(255, 214, 102, 0.05));
  border-radius: var(--radius-md);
  border: 2px solid rgba(255, 214, 102, 0.2);
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--spacing-lg);
  flex: 1;
  align-content: start;
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

.btn-primary {
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  color: white;
  box-shadow: var(--shadow-clay);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

.btn-primary:active {
  transform: scale(0.96);
  box-shadow: var(--shadow-pressed);
}

.btn-secondary {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  border-color: var(--color-border);
}

.btn-secondary:hover {
  background: var(--peach-50);
  border-color: var(--peach-200);
  color: var(--peach-500);
}

.btn-ghost {
  background: transparent;
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.btn-ghost:hover {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
}

.mt-4 {
  margin-top: var(--spacing-lg);
}
</style>
