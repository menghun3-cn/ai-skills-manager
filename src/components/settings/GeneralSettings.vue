<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
import { FolderOpen, Check } from "lucide-vue-next";

const { t } = useI18n();
const settingsStore = useSettingsStore();

const storagePath = computed(() => {
  if (settingsStore.configPath) {
    return `${settingsStore.configPath}/skills`;
  }
  return "~/.ai-skills-manager/skills";
});

function openStoragePath() {
  window.open(`file://${storagePath.value}`);
}

function updateAutoSync(value: boolean) {
  settingsStore.saveSettings({ autoSync: value });
}

function updateCleanupOnDisable(value: boolean) {
  settingsStore.saveSettings({ cleanupOnDisable: value });
}

function updateSyncNotification(value: boolean) {
  settingsStore.saveSettings({ syncNotifications: value });
}

function updateProxyUrl(value: string) {
  settingsStore.saveSettings({ proxyUrl: value || undefined });
}

function updateGithubToken(value: string) {
  settingsStore.saveGithubToken(value || undefined);
}
</script>

<template>
  <div class="settings-container">
    <!-- 存储目录 -->
    <div class="setting-card">
      <div class="setting-header">
        <label class="setting-label">{{ t("settings.storageDir") }}</label>
        <p class="setting-desc">{{ t("settings.storageDirFixed") }}</p>
      </div>
      <div class="input-group">
        <div class="input-wrapper">
          <input
            :value="storagePath"
            type="text"
            readonly
            class="custom-input"
          />
        </div>
        <button
          @click="openStoragePath"
          class="action-btn btn-secondary"
        >
          <FolderOpen class="btn-icon" :stroke-width="1.5" />
          {{ t("common.open") }}
        </button>
      </div>
    </div>

    <!-- 默认编辑器 -->
    <div class="setting-card">
      <div class="setting-header">
        <label class="setting-label">{{ t("settings.defaultEditor") }}</label>
        <p class="setting-desc">{{ t("settings.editorDesc") }}</p>
      </div>
      <div class="select-wrapper">
        <select
          :value="settingsStore.settings.editor"
          @change="settingsStore.saveSettings({ editor: ($event.target as HTMLSelectElement).value as any })"
          class="custom-select"
        >
          <option value="monaco">{{ t("settings.monacoEditor") }}</option>
          <option value="system">{{ t("settings.systemEditor") }}</option>
        </select>
      </div>
    </div>

    <!-- 自动同步 -->
    <div class="setting-card setting-row">
      <div class="setting-info">
        <label class="setting-label">{{ t("settings.autoSync") }}</label>
        <p class="setting-desc">{{ t("settings.autoSyncDesc") }}</p>
      </div>
      <button
        @click="updateAutoSync(!settingsStore.settings.autoSync)"
        class="toggle-switch"
        :class="{ active: settingsStore.settings.autoSync }"
        :aria-label="settingsStore.settings.autoSync ? t('settings.disable') : t('settings.enable')"
      >
        <span class="toggle-slider">
          <Check v-if="settingsStore.settings.autoSync" class="check-icon" :stroke-width="3" />
        </span>
      </button>
    </div>

    <!-- 禁用时清理 -->
    <div class="setting-card setting-row">
      <div class="setting-info">
        <label class="setting-label">{{ t("settings.cleanupOnDisable") }}</label>
        <p class="setting-desc">{{ t("settings.cleanupOnDisableDesc") }}</p>
      </div>
      <button
        @click="updateCleanupOnDisable(!settingsStore.settings.cleanupOnDisable)"
        class="toggle-switch"
        :class="{ active: settingsStore.settings.cleanupOnDisable }"
        :aria-label="settingsStore.settings.cleanupOnDisable ? t('settings.disable') : t('settings.enable')"
      >
        <span class="toggle-slider">
          <Check v-if="settingsStore.settings.cleanupOnDisable" class="check-icon" :stroke-width="3" />
        </span>
      </button>
    </div>

    <!-- 同步通知 -->
    <div class="setting-card setting-row">
      <div class="setting-info">
        <label class="setting-label">{{ t("settings.syncNotification") }}</label>
        <p class="setting-desc">{{ t("settings.syncNotificationDesc") }}</p>
      </div>
      <button
        @click="updateSyncNotification(!settingsStore.settings.syncNotifications)"
        class="toggle-switch"
        :class="{ active: settingsStore.settings.syncNotifications }"
        :aria-label="settingsStore.settings.syncNotifications ? t('settings.disable') : t('settings.enable')"
      >
        <span class="toggle-slider">
          <Check v-if="settingsStore.settings.syncNotifications" class="check-icon" :stroke-width="3" />
        </span>
      </button>
    </div>

    <!-- 代理设置 -->
    <div class="setting-card">
      <div class="setting-header">
        <label class="setting-label">{{ t("settings.proxyUrl") }}</label>
        <p class="setting-desc">{{ t("settings.proxyUrlDesc") }}</p>
      </div>
      <div class="input-group">
        <div class="input-wrapper">
          <input
            :value="settingsStore.settings.proxyUrl || ''"
            @input="updateProxyUrl(($event.target as HTMLInputElement).value)"
            type="text"
            :placeholder="t('settings.proxyUrlPlaceholder')"
            class="custom-input"
          />
        </div>
      </div>
    </div>

    <!-- GitHub Token 设置 -->
    <div class="setting-card">
      <div class="setting-header">
        <label class="setting-label">{{ t("settings.githubToken") }}</label>
        <p class="setting-desc">{{ t("settings.githubTokenDesc") }}</p>
      </div>
      <div class="input-group">
        <div class="input-wrapper">
          <input
            :value="settingsStore.githubToken || ''"
            @input="updateGithubToken(($event.target as HTMLInputElement).value)"
            type="password"
            :placeholder="t('settings.githubTokenPlaceholder')"
            class="custom-input"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
  max-width: 600px;
}

.setting-card {
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  border: 2px solid var(--color-border-subtle);
  box-shadow: var(--shadow-soft);
  transition: all var(--transition-fast) var(--ease-out);
}

.setting-card:hover {
  border-color: var(--peach-200);
  box-shadow: var(--shadow-card);
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-lg);
}

.setting-header {
  margin-bottom: var(--spacing-md);
}

.setting-info {
  flex: 1;
}

.setting-label {
  display: block;
  font-size: var(--text-base);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--spacing-xs);
  font-family: var(--font-heading);
}

.setting-desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* 输入框组 */
.input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.input-wrapper {
  flex: 1;
  position: relative;
}

.custom-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  font-size: var(--text-sm);
  font-family: var(--font-mono);
  transition: all var(--transition-fast) var(--ease-out);
}

.custom-input:hover {
  border-color: var(--cream-400);
}

.custom-input:focus {
  outline: none;
  border-color: var(--peach-300);
  box-shadow: 0 0 0 4px rgba(255, 155, 173, 0.15);
}

/* 选择器 */
.select-wrapper {
  position: relative;
}

.select-wrapper::after {
  content: '';
  position: absolute;
  right: var(--spacing-md);
  top: 50%;
  transform: translateY(-50%);
  width: 0;
  height: 0;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: 5px solid var(--color-text-tertiary);
  pointer-events: none;
}

.custom-select {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-xl) var(--spacing-sm) var(--spacing-md);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  font-family: var(--font-body);
  cursor: pointer;
  appearance: none;
  transition: all var(--transition-fast) var(--ease-out);
}

.custom-select:hover {
  border-color: var(--cream-400);
}

.custom-select:focus {
  outline: none;
  border-color: var(--peach-300);
  box-shadow: 0 0 0 4px rgba(255, 155, 173, 0.15);
}

/* 按钮 */
.action-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 600;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  font-family: var(--font-body);
  white-space: nowrap;
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

.btn-icon {
  width: 16px;
  height: 16px;
}

/* 开关切换 */
.toggle-switch {
  position: relative;
  width: 52px;
  height: 28px;
  border-radius: var(--radius-full);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border);
  cursor: pointer;
  transition: all var(--transition-normal) var(--ease-elastic);
  flex-shrink: 0;
}

.toggle-switch.active {
  background: linear-gradient(135deg, var(--mint), #6BC49A);
  border-color: transparent;
}

.toggle-slider {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: var(--radius-full);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
  transition: transform var(--transition-normal) var(--ease-elastic);
  display: flex;
  align-items: center;
  justify-content: center;
}

.toggle-switch.active .toggle-slider {
  transform: translateX(24px);
}

.check-icon {
  width: 12px;
  height: 12px;
  color: var(--mint);
}

.toggle-switch:hover {
  border-color: var(--cream-400);
}

.toggle-switch.active:hover {
  box-shadow: 0 0 0 4px rgba(125, 212, 168, 0.2);
}
</style>
