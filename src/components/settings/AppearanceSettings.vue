<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
import { Sun, Moon, Monitor, Check, Globe } from "lucide-vue-next";

const { t, locale } = useI18n();
const settingsStore = useSettingsStore();

function handleThemeChange(theme: "light" | "dark" | "system") {
  settingsStore.setTheme(theme);
}

function handleLanguageChange(lang: string) {
  locale.value = lang;
  settingsStore.setLanguage(lang);
}

const themes = [
  { id: "light", label: "settings.themeLight", icon: Sun, desc: "settings.themeLightDesc" },
  { id: "dark", label: "settings.themeDark", icon: Moon, desc: "settings.themeDarkDesc" },
  { id: "system", label: "settings.themeSystem", icon: Monitor, desc: "settings.themeSystemDesc" },
] as const;

const languages = [
  { value: "zh-CN", label: "中文 (简体)", flag: "🇨🇳" },
  { value: "en", label: "English", flag: "🇺🇸" },
] as const;
</script>

<template>
  <div class="settings-container">
    <!-- 主题设置 -->
    <div class="setting-section">
      <div class="section-header">
        <label class="section-label">{{ t("settings.theme") }}</label>
        <p class="section-desc">{{ t("settings.themeDesc") }}</p>
      </div>
      <div class="theme-grid">
        <button
          v-for="theme in themes"
          :key="theme.id"
          @click="handleThemeChange(theme.id)"
          class="theme-card"
          :class="{ active: settingsStore.settings.theme === theme.id }"
        >
          <div class="theme-icon-wrapper">
            <component :is="theme.icon" class="theme-icon" :stroke-width="1.5" />
          </div>
          <div class="theme-info">
            <span class="theme-name">{{ t(theme.label) }}</span>
            <span class="theme-desc">{{ t(theme.desc) }}</span>
          </div>
          <div v-if="settingsStore.settings.theme === theme.id" class="check-badge">
            <Check class="check-icon" :stroke-width="3" />
          </div>
        </button>
      </div>
    </div>

    <!-- 语言设置 -->
    <div class="setting-section">
      <div class="section-header">
        <label class="section-label">{{ t("settings.language") }}</label>
        <p class="section-desc">{{ t("settings.languageDesc") }}</p>
      </div>
      <div class="language-grid">
        <button
          v-for="lang in languages"
          :key="lang.value"
          @click="handleLanguageChange(lang.value)"
          class="language-card"
          :class="{ active: settingsStore.settings.language === lang.value }"
        >
          <span class="language-flag">{{ lang.flag }}</span>
          <span class="language-name">{{ lang.label }}</span>
          <div v-if="settingsStore.settings.language === lang.value" class="check-badge">
            <Check class="check-icon" :stroke-width="3" />
          </div>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
  max-width: 600px;
}

.setting-section {
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  padding: var(--spacing-xl);
  border: 2px solid var(--color-border-subtle);
  box-shadow: var(--shadow-soft);
}

.section-header {
  margin-bottom: var(--spacing-lg);
}

.section-label {
  display: block;
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--spacing-xs);
  font-family: var(--font-heading);
}

.section-desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
}

/* 主题卡片网格 */
.theme-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--spacing-md);
}

.theme-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  position: relative;
}

.theme-card:hover {
  background: var(--peach-50);
  border-color: var(--peach-200);
  transform: translateY(-2px);
}

.theme-card.active {
  background: linear-gradient(135deg, var(--peach-50), var(--peach-100));
  border-color: var(--peach-300);
  box-shadow: var(--shadow-clay);
}

.theme-icon-wrapper {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-soft);
  transition: all var(--transition-fast) var(--ease-out);
}

.theme-card:hover .theme-icon-wrapper {
  transform: scale(1.05);
}

.theme-card.active .theme-icon-wrapper {
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  color: white;
}

.theme-icon {
  width: 24px;
  height: 24px;
  color: var(--color-text-secondary);
  transition: color var(--transition-fast) var(--ease-out);
}

.theme-card.active .theme-icon {
  color: white;
}

.theme-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.theme-name {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-primary);
  font-family: var(--font-body);
}

.theme-desc {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.theme-card.active .theme-name {
  color: var(--peach-500);
}

.check-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--mint), #6BC49A);
  border-radius: var(--radius-full);
  box-shadow: var(--shadow-soft);
  animation: popIn 0.3s var(--ease-elastic);
}

@keyframes popIn {
  0% { transform: scale(0); }
  70% { transform: scale(1.2); }
  100% { transform: scale(1); }
}

.check-icon {
  width: 14px;
  height: 14px;
  color: white;
}

/* 语言卡片 */
.language-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-md);
}

.language-card {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  position: relative;
}

.language-card:hover {
  background: var(--peach-50);
  border-color: var(--peach-200);
  transform: translateX(4px);
}

.language-card.active {
  background: linear-gradient(135deg, var(--peach-50), var(--peach-100));
  border-color: var(--peach-300);
  box-shadow: var(--shadow-clay);
}

.language-flag {
  font-size: var(--text-2xl);
  line-height: 1;
}

.language-name {
  flex: 1;
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text-primary);
  font-family: var(--font-body);
}

.language-card.active .language-name {
  color: var(--peach-500);
  font-weight: 600;
}

.language-card .check-badge {
  position: static;
  width: 20px;
  height: 20px;
}

.language-card .check-icon {
  width: 12px;
  height: 12px;
}

/* 响应式 */
@media (max-width: 480px) {
  .theme-grid {
    grid-template-columns: 1fr;
  }

  .theme-card {
    flex-direction: row;
    justify-content: flex-start;
  }

  .theme-info {
    align-items: flex-start;
  }

  .language-grid {
    grid-template-columns: 1fr;
  }
}
</style>
