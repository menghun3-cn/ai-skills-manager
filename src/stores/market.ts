import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { MarketItem } from "@/types";
import { useSettingsStore } from "./settings";

const CHECK_INTERVAL = 60 * 1000;

export const useMarketStore = defineStore("market", () => {
  const items = ref<MarketItem[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const searchQuery = ref("");
  const filterSource = ref<"all" | "awesome-claude-skills" | "skills.sh">("all");
  const lastFetched = ref<string | null>(null);
  const cacheAge = ref<string | null>(null);
  const usingCache = ref(false);

  let checkInterval: ReturnType<typeof setInterval> | null = null;

  const filteredItems = computed(() => {
    let result = items.value;

    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      result = result.filter(
        (item) =>
          item.name.toLowerCase().includes(query) ||
          item.description?.toLowerCase().includes(query)
      );
    }

    if (filterSource.value !== "all") {
      result = result.filter((item) => item.source === filterSource.value);
    }

    return result;
  });

  async function fetchMarketData(options: { forceRefresh?: boolean } = {}) {
    loading.value = true;
    error.value = null;
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      if (typeof invoke !== "function") {
        throw new Error("Tauri API not available");
      }

      if (options.forceRefresh) {
        await invoke("clear_market_cache");
      }

      const result = await invoke<{ items: MarketItem[]; cached: boolean }>("get_market_data_with_cache");
      items.value = result.items;
      usingCache.value = result.cached;
      lastFetched.value = new Date().toISOString();

      await updateCacheAge();
    } catch (err) {
      error.value = String(err);
      console.error("Failed to fetch market data:", err);
    } finally {
      loading.value = false;
    }
  }

  async function updateCacheAge() {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      if (typeof invoke === "function") {
        cacheAge.value = await invoke<string | null>("get_market_cache_age");
      }
    } catch {
      cacheAge.value = null;
    }
  }

  function startAutoRefresh() {
    if (checkInterval) return;
    checkInterval = setInterval(async () => {
      await updateCacheAge();
    }, CHECK_INTERVAL);
  }

  function stopAutoRefresh() {
    if (checkInterval) {
      clearInterval(checkInterval);
      checkInterval = null;
    }
  }

  async function installSkill(marketId: string) {
    const { invoke } = await import("@tauri-apps/api/core");
    if (typeof invoke !== "function") {
      throw new Error("Tauri API not available");
    }
    const settingsStore = useSettingsStore();
    const githubToken = settingsStore.settings.githubToken;
    await invoke("install_skill", { marketId, githubToken });

    const item = items.value.find(i => i.id === marketId);
    if (item) {
      item.installed = true;
    }
  }

  async function clearCache() {
    const { invoke } = await import("@tauri-apps/api/core");
    if (typeof invoke === "function") {
      await invoke("clear_market_cache");
    }
    items.value = [];
    lastFetched.value = null;
    cacheAge.value = null;
    usingCache.value = false;
  }

  return {
    items,
    loading,
    error,
    searchQuery,
    filterSource,
    lastFetched,
    cacheAge,
    usingCache,
    filteredItems,
    fetchMarketData,
    updateCacheAge,
    startAutoRefresh,
    stopAutoRefresh,
    installSkill,
    clearCache,
  };
});
