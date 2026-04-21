<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import GeneralSettings from "@/components/settings/GeneralSettings.vue";
import MarketSettings from "@/components/settings/MarketSettings.vue";
import AppearanceSettings from "@/components/settings/AppearanceSettings.vue";
import AboutPage from "@/components/settings/AboutPage.vue";
import { useSettingsStore } from "@/stores/settings";
import PageHeader from "@/components/common/PageHeader.vue";
import { Settings as SettingsIcon } from "lucide-vue-next";

const { t } = useI18n();
const settingsStore = useSettingsStore();

const activeTab = ref("general");
const hasError = ref(false);
const errorMsg = ref("");

onMounted(async () => {
  try {
    await settingsStore.loadSettings();
  } catch (e: any) {
    hasError.value = true;
    errorMsg.value = e?.message || String(e);
    console.error("[SettingsPage] loadSettings failed:", e);
  }
});

const tabs = [
  { id: "general", label: "settings.general" },
  { id: "market", label: "settings.market" },
  { id: "appearance", label: "settings.appearance" },
  { id: "about", label: "settings.about" },
];
</script>

<template>
  <div class="page">
    <PageHeader
      :title="t('settings.title')"
      :icon="SettingsIcon"
    />

    <div v-if="hasError" class="error-state">
      <div class="error-icon">⚠️</div>
      <p class="error-text">{{ t("settings.loadError") }}: {{ errorMsg }}</p>
    </div>

    <div v-else class="settings-content">
      <div class="tab-nav">
        <button
          v-for="(tab, index) in tabs"
          :key="tab.id"
          @click="activeTab = tab.id"
          class="tab-btn"
          :class="{ active: activeTab === tab.id }"
          :style="{ animationDelay: `${index * 80}ms` }"
        >
          {{ t(tab.label) }}
        </button>
      </div>

      <div class="tab-content">
        <transition name="tab-fade" mode="out-in">
          <GeneralSettings v-if="activeTab === 'general'" key="general" />
          <MarketSettings v-else-if="activeTab === 'market'" key="market" />
          <AppearanceSettings v-else-if="activeTab === 'appearance'" key="appearance" />
          <AboutPage v-else-if="activeTab === 'about'" key="about" />
        </transition>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-lg);
  height: 200px;
  border-radius: var(--radius-xl);
  background: linear-gradient(135deg, rgba(255, 123, 123, 0.1), rgba(255, 123, 123, 0.05));
  border: 2px solid rgba(255, 123, 123, 0.3);
  color: var(--strawberry);
  padding: var(--spacing-xl);
  margin-top: var(--spacing-lg);
}

.error-icon {
  font-size: var(--text-3xl);
}

.error-text {
  font-size: var(--text-base);
  font-weight: 500;
  text-align: center;
}

.settings-content {
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-card);
  overflow: hidden;
  border: 2px solid var(--color-border-subtle);
  margin-top: var(--spacing-lg);
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.tab-nav {
  display: flex;
  gap: var(--spacing-xs);
  padding: var(--spacing-lg);
  border-bottom: 2px solid var(--color-border-subtle);
  background: linear-gradient(180deg, var(--color-surface) 0%, var(--color-surface-subtle) 100%);
}

.tab-btn {
  padding: var(--spacing-sm) var(--spacing-lg);
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text-secondary);
  background: transparent;
  border: 2px solid transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  position: relative;
  font-family: var(--font-body);
  animation: slideInUp 0.4s var(--ease-out) backwards;
}

@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.tab-btn:hover {
  color: var(--color-text-primary);
  background: var(--color-surface-subtle);
  border-color: var(--color-border);
}

.tab-btn.active {
  color: var(--peach-500);
  background: linear-gradient(135deg, var(--peach-100), var(--peach-50));
  border-color: var(--peach-200);
  box-shadow: var(--shadow-soft);
}

.tab-btn.active:hover {
  background: linear-gradient(135deg, var(--peach-200), var(--peach-100));
}

.tab-content {
  padding: var(--spacing-xl);
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

/* Tab 内容过渡动画 */
.tab-fade-enter-active,
.tab-fade-leave-active {
  transition: all 0.25s var(--ease-out);
}

.tab-fade-enter-from {
  opacity: 0;
  transform: translateX(10px);
}

.tab-fade-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}
</style>
