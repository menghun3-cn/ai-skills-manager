<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useSettingsStore } from "@/stores/settings";
import { useMarketStore } from "@/stores/market";
import { Key, Database, Clock, Trash2, Check, Github, Globe, ExternalLink } from "lucide-vue-next";

const { t } = useI18n();
const settingsStore = useSettingsStore();
const marketStore = useMarketStore();

const githubToken = ref("");
const showTokenSaved = ref(false);

onMounted(() => {
  githubToken.value = settingsStore.settings.githubToken ?? "";
});

function updateGithubToken() {
  settingsStore.saveSettings({ githubToken: githubToken.value });
  showTokenSaved.value = true;
  setTimeout(() => {
    showTokenSaved.value = false;
  }, 2000);
}

async function clearCache() {
  await marketStore.clearCache();
}

const dataSources = [
  {
    id: "awesome-claude-skills",
    name: t("settings.awesomeClaudeSkills"),
    url: t("settings.awesomeClaudeSkillsUrl"),
    icon: Github,
    color: "lavender"
  },
  {
    id: "skills-sh",
    name: t("settings.skillsSh"),
    url: t("settings.skillsShUrl"),
    icon: Globe,
    color: "honey"
  }
];

function isSourceEnabled(sourceId: string): boolean {
  return settingsStore.settings.enabledDataSources?.includes(sourceId) ?? true;
}

function toggleSource(sourceId: string, enabled: boolean) {
  const currentSources = settingsStore.settings.enabledDataSources ?? ["awesome-claude-skills", "skills-sh"];
  let newSources: string[];
  
  if (enabled) {
    newSources = [...new Set([...currentSources, sourceId])];
  } else {
    newSources = currentSources.filter(id => id !== sourceId);
  }
  
  settingsStore.saveSettings({ enabledDataSources: newSources });
}

const intervals = [
  { value: 15, label: "15" },
  { value: 30, label: "30" },
  { value: 60, label: "60" },
  { value: 120, label: "120" }
];
</script>

<template>
  <div class="settings-container">
    <!-- GitHub Token -->
    <div class="setting-card">
      <div class="setting-header">
        <div class="header-icon token-icon">
          <Key class="icon-svg" :stroke-width="1.5" />
        </div>
        <div class="header-content">
          <label class="setting-label">{{ t("settings.githubToken") }}</label>
          <p class="setting-desc">{{ t("settings.githubTokenDesc") }}</p>
        </div>
      </div>
      <div class="input-group">
        <div class="input-wrapper">
          <input
            v-model="githubToken"
            type="password"
            :placeholder="t('settings.githubTokenPlaceholder')"
            class="custom-input"
          />
          <div v-if="showTokenSaved" class="save-indicator">
            <Check class="save-icon" :stroke-width="3" />
          </div>
        </div>
        <button
          @click="updateGithubToken"
          class="action-btn btn-primary"
        >
          {{ t("common.save") }}
        </button>
      </div>
    </div>

    <!-- 数据源 -->
    <div class="setting-card">
      <div class="setting-header">
        <div class="header-icon source-icon">
          <Database class="icon-svg" :stroke-width="1.5" />
        </div>
        <div class="header-content">
          <label class="setting-label">{{ t("settings.dataSources") }}</label>
          <p class="setting-desc">{{ t("settings.dataSourcesDesc") }}</p>
        </div>
      </div>
      <div class="source-list">
        <label
          v-for="source in dataSources"
          :key="source.id"
          class="source-item"
        >
          <input
            type="checkbox"
            :checked="isSourceEnabled(source.id)"
            @change="(e) => toggleSource(source.id, (e.target as HTMLInputElement).checked)"
            class="custom-checkbox"
          />
          <div class="source-content">
            <div class="source-icon-wrapper" :class="`bg-${source.color}`">
              <component :is="source.icon" class="source-icon" :stroke-width="1.5" />
            </div>
            <div class="source-info">
              <span class="source-name">{{ source.name }}</span>
              <a
                :href="source.url"
                target="_blank"
                class="source-url"
                @click.stop
              >
                {{ source.url }}
                <ExternalLink class="link-icon" :stroke-width="2" />
              </a>
            </div>
          </div>
        </label>
      </div>
    </div>

    <!-- 缓存策略 -->
    <div class="setting-card">
      <div class="setting-header">
        <div class="header-icon cache-icon">
          <Clock class="icon-svg" :stroke-width="1.5" />
        </div>
        <div class="header-content">
          <label class="setting-label">{{ t("settings.cacheStrategy") }}</label>
          <p class="setting-desc">{{ t("settings.cacheStrategyDesc") }}</p>
        </div>
      </div>
      <div class="cache-controls">
        <div class="interval-control">
          <span class="control-label">{{ t("settings.refreshInterval") }}</span>
          <div class="interval-select-wrapper">
            <select
              :value="settingsStore.settings.refreshInterval"
              @change="settingsStore.saveSettings({ refreshInterval: Number(($event.target as HTMLSelectElement).value) })"
              class="interval-select"
            >
              <option v-for="interval in intervals" :key="interval.value" :value="interval.value">
                {{ interval.label }}
              </option>
            </select>
            <span class="interval-unit">{{ t("settings.minutes") }}</span>
          </div>
        </div>
        <button
          @click="clearCache"
          class="action-btn btn-danger"
        >
          <Trash2 class="btn-icon" :stroke-width="1.5" />
          {{ t("settings.clearCache") }}
        </button>
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
  border-radius: var(--radius-xl);
  padding: var(--spacing-xl);
  border: 2px solid var(--color-border-subtle);
  box-shadow: var(--shadow-soft);
  transition: all var(--transition-fast) var(--ease-out);
}

.setting-card:hover {
  border-color: var(--peach-200);
  box-shadow: var(--shadow-card);
}

.setting-header {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-lg);
}

.header-icon {
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.token-icon {
  background: linear-gradient(135deg, var(--lavender-300), var(--lavender-400));
  color: white;
}

.source-icon {
  background: linear-gradient(135deg, var(--mint), #6BC49A);
  color: white;
}

.cache-icon {
  background: linear-gradient(135deg, var(--honey), #FFB84D);
  color: white;
}

.icon-svg {
  width: 22px;
  height: 22px;
}

.header-content {
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
  padding-right: 40px;
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
  border-color: var(--lavender-300);
  box-shadow: 0 0 0 4px rgba(184, 169, 232, 0.15);
}

.save-indicator {
  position: absolute;
  right: var(--spacing-sm);
  top: 50%;
  transform: translateY(-50%);
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--mint), #6BC49A);
  border-radius: var(--radius-full);
  animation: popIn 0.3s var(--ease-elastic);
}

.save-icon {
  width: 14px;
  height: 14px;
  color: white;
}

@keyframes popIn {
  0% { transform: translateY(-50%) scale(0); }
  70% { transform: translateY(-50%) scale(1.2); }
  100% { transform: translateY(-50%) scale(1); }
}

/* 按钮 */
.action-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 600;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  font-family: var(--font-body);
  white-space: nowrap;
}

.btn-primary {
  background: linear-gradient(135deg, var(--lavender-300), var(--lavender-400));
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

.btn-danger {
  background: linear-gradient(135deg, var(--strawberry), #FF9999);
  color: white;
  box-shadow: var(--shadow-clay);
}

.btn-danger:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

.btn-danger:active {
  transform: scale(0.96);
  box-shadow: var(--shadow-pressed);
}

.btn-icon {
  width: 16px;
  height: 16px;
}

/* 数据源列表 */
.source-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.source-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.source-item:hover {
  border-color: var(--peach-200);
  background: var(--peach-50);
}

.custom-checkbox {
  width: 20px;
  height: 20px;
  margin-top: 2px;
  accent-color: var(--peach-400);
  cursor: pointer;
}

.source-content {
  flex: 1;
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md);
}

.source-icon-wrapper {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.bg-lavender {
  background: linear-gradient(135deg, var(--lavender-300), var(--lavender-400));
  color: white;
}

.bg-honey {
  background: linear-gradient(135deg, var(--honey), #FFB84D);
  color: white;
}

.source-icon {
  width: 20px;
  height: 20px;
}

.source-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.source-name {
  font-size: var(--text-base);
  font-weight: 600;
  color: var(--color-text-primary);
  font-family: var(--font-body);
}

.source-url {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: var(--text-xs);
  color: var(--lavender-500);
  text-decoration: none;
  transition: color var(--transition-fast) var(--ease-out);
}

.source-url:hover {
  color: var(--peach-500);
  text-decoration: underline;
}

.link-icon {
  width: 12px;
  height: 12px;
}

/* 缓存控制 */
.cache-controls {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-lg);
  flex-wrap: wrap;
}

.interval-control {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.control-label {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.interval-select-wrapper {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.interval-select {
  padding: var(--spacing-sm) var(--spacing-md);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  font-family: var(--font-body);
  cursor: pointer;
  min-width: 70px;
  text-align: center;
  transition: all var(--transition-fast) var(--ease-out);
}

.interval-select:hover {
  border-color: var(--cream-400);
}

.interval-select:focus {
  outline: none;
  border-color: var(--honey);
  box-shadow: 0 0 0 4px rgba(255, 214, 102, 0.15);
}

.interval-unit {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}
</style>
