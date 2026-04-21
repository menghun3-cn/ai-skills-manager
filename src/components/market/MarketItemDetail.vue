<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { MarketItem } from "@/types";
import { useMarketStore } from "@/stores/market";
import { useSkillsStore } from "@/stores/skills";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  X,
  Package,
  ExternalLink,
  Download,
  Check,
  User,
  Tag,
  Globe,
  Star,
  GitBranch,
} from "lucide-vue-next";

const props = defineProps<{
  item: MarketItem;
}>();

const emit = defineEmits<{
  close: [];
  install: [itemId: string];
}>();

const { t } = useI18n();
const marketStore = useMarketStore();
const skillsStore = useSkillsStore();

const isInstalled = computed(() => props.item.installed);
const isInstalling = computed(() => marketStore.installingId === props.item.id);

const itemUrl = computed(() => {
  if (props.item.repo) {
    return props.item.repo;
  }
  if (props.item.source === "awesome-claude-skills") {
    return `https://github.com/ComposioHQ/awesome-claude-skills`;
  }
  if (props.item.source === "skills.sh") {
    return `https://skills.sh`;
  }
  return null;
});

const sourceLabel = computed(() => {
  if (props.item.source === "awesome-claude-skills") return "Awesome";
  if (props.item.source === "skills.sh") return "Skills.sh";
  return props.item.source;
});

const sourceColor = computed(() => {
  if (props.item.source === "awesome-claude-skills") return "lavender";
  if (props.item.source === "skills.sh") return "honey";
  return "cream";
});

function formatDownloads(downloads?: number): string {
  if (!downloads) return "-";
  if (downloads >= 1000) {
    return (downloads / 1000).toFixed(1) + "k";
  }
  return downloads.toString();
}

async function openItemPage() {
  const url = itemUrl.value;
  if (url) {
    try {
      await openUrl(url);
    } catch (error) {
      console.error("Failed to open URL:", error);
      window.open(url, "_blank");
    }
  }
}

async function handleInstall() {
  if (isInstalling.value || isInstalled.value) return;
  emit("install", props.item.id);
}

function handleClose() {
  emit("close");
}
</script>

<template>
  <div class="market-detail-overlay" @click="handleClose">
    <div class="market-detail-panel" @click.stop>
      <!-- 头部 -->
      <div class="detail-header">
        <div class="header-main">
          <div class="item-icon-large">
            <Package class="icon-svg" :stroke-width="1.5" />
          </div>
          <div class="header-info">
            <h2 class="item-name">{{ item.name }}</h2>
            <div class="item-source">
              <span class="source-tag" :class="`tag-${sourceColor}`">
                {{ sourceLabel }}
              </span>
            </div>
          </div>
        </div>
        <button class="close-btn" @click="handleClose">
          <X class="close-icon" :stroke-width="2" />
        </button>
      </div>

      <!-- 描述 -->
      <div class="detail-section">
        <h3 class="section-title">{{ t("market.description") }}</h3>
        <p class="description-text">
          {{ item.description || t("market.noDescription") }}
        </p>
      </div>

      <!-- 元信息 -->
      <div class="detail-section">
        <h3 class="section-title">{{ t("market.info") }}</h3>
        <div class="info-grid">
          <div v-if="item.author" class="info-item">
            <User class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("market.author") }}:</span>
            <span class="info-value">{{ item.author }}</span>
          </div>
          <div v-if="item.skill_id" class="info-item">
            <Tag class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("market.skillId") }}:</span>
            <span class="info-value">{{ item.skill_id }}</span>
          </div>
          <div class="info-item">
            <Globe class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("market.source") }}:</span>
            <span class="info-value">{{ sourceLabel }}</span>
          </div>
          <div v-if="item.downloads" class="info-item">
            <Star class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("market.downloads") }}:</span>
            <span class="info-value">{{ formatDownloads(item.downloads) }}</span>
          </div>
          <div v-if="item.repo" class="info-item full-width">
            <GitBranch class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("market.repository") }}:</span>
            <a
              :href="item.repo"
              target="_blank"
              class="info-link"
              @click.prevent="openItemPage"
            >
              {{ item.repo }}
            </a>
          </div>
        </div>
      </div>

      <!-- 安装状态 -->
      <div class="detail-section">
        <h3 class="section-title">{{ t("market.status") }}</h3>
        <div class="status-display">
          <div v-if="isInstalled" class="status-badge installed">
            <Check class="status-icon" :stroke-width="2" />
            <span>{{ t("market.installed") }}</span>
          </div>
          <div v-else class="status-badge not-installed">
            <Download class="status-icon" :stroke-width="2" />
            <span>{{ t("market.notInstalled") }}</span>
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="detail-actions">
        <button
          v-if="itemUrl"
          class="action-btn btn-secondary"
          @click="openItemPage"
        >
          <ExternalLink class="btn-icon" :stroke-width="1.5" />
          {{ t("market.openPage") }}
        </button>
        <button
          v-if="isInstalled"
          class="action-btn btn-success"
          disabled
        >
          <Check class="btn-icon" :stroke-width="2" />
          {{ t("market.installed") }}
        </button>
        <button
          v-else-if="isInstalling"
          class="action-btn btn-installing"
          disabled
        >
          <span class="loading-spinner" />
          {{ t("market.installing") }}
        </button>
        <button
          v-else
          class="action-btn btn-primary"
          @click="handleInstall"
        >
          <Download class="btn-icon" :stroke-width="1.5" />
          {{ t("market.install") }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.market-detail-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(4px);
  z-index: 100;
  display: flex;
  justify-content: flex-end;
  animation: fadeIn 0.2s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.market-detail-panel {
  width: 420px;
  max-width: 100%;
  height: 100%;
  background: var(--color-surface);
  border-left: 1px solid var(--color-border);
  box-shadow: -4px 0 24px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  animation: slideIn 0.3s var(--ease-out);
  overflow: hidden;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
  }
  to {
    transform: translateX(0);
  }
}

/* 头部 */
.detail-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: var(--spacing-xl);
  border-bottom: 1px solid var(--color-border);
  background: linear-gradient(135deg, var(--peach-50), var(--lavender-50));
}

.header-main {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.item-icon-large {
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  border-radius: var(--radius-lg);
  color: white;
  flex-shrink: 0;
  box-shadow: var(--shadow-clay);
}

.icon-svg {
  width: 28px;
  height: 28px;
}

.header-info {
  flex: 1;
  min-width: 0;
}

.item-name {
  font-size: var(--text-xl);
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  font-family: var(--font-heading);
}

.item-source {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.source-tag {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 500;
}

.tag-lavender {
  background: linear-gradient(135deg, var(--lavender-100), var(--lavender-50));
  color: var(--lavender-500);
  border: 1px solid var(--lavender-200);
}

.tag-honey {
  background: linear-gradient(135deg, rgba(255, 214, 102, 0.2), rgba(255, 214, 102, 0.1));
  color: #D4A030;
  border: 1px solid rgba(255, 214, 102, 0.4);
}

.tag-cream {
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.close-btn:hover {
  background: var(--strawberry-50);
  border-color: var(--strawberry-200);
  color: var(--strawberry);
  transform: rotate(90deg);
}

.close-icon {
  width: 18px;
  height: 18px;
}

/* 内容区域 */
.detail-section {
  padding: var(--spacing-lg) var(--spacing-xl);
  border-bottom: 1px solid var(--color-border-subtle);
}

.section-title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin: 0 0 var(--spacing-md) 0;
}

.description-text {
  font-size: var(--text-sm);
  color: var(--color-text-primary);
  line-height: 1.6;
  margin: 0;
}

/* 信息网格 */
.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

.info-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--text-sm);
}

.info-item.full-width {
  grid-column: 1 / -1;
}

.info-icon {
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
  flex-shrink: 0;
}

.info-label {
  color: var(--color-text-tertiary);
}

.info-value {
  color: var(--color-text-primary);
  font-weight: 500;
}

.info-link {
  color: var(--lavender-500);
  text-decoration: none;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.info-link:hover {
  text-decoration: underline;
}

/* 状态显示 */
.status-display {
  display: flex;
  align-items: center;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 600;
}

.status-badge.installed {
  background: linear-gradient(135deg, var(--mint-100), var(--mint-50));
  color: var(--mint-600);
  border: 1px solid var(--mint-200);
}

.status-badge.not-installed {
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.status-icon {
  width: 18px;
  height: 18px;
}

/* 操作按钮 */
.detail-actions {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg) var(--spacing-xl);
  margin-top: auto;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface-subtle);
}

.action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 600;
  border: none;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.btn-secondary {
  background: var(--color-surface);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
}

.btn-secondary:hover {
  background: var(--color-surface-subtle);
  border-color: var(--color-border-strong);
  transform: translateY(-1px);
}

.btn-primary {
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  color: white;
  box-shadow: var(--shadow-clay);
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

.btn-success {
  background: linear-gradient(135deg, var(--mint-300), var(--mint-400));
  color: white;
  cursor: default;
  opacity: 0.9;
}

.btn-installing {
  background: linear-gradient(135deg, var(--honey), #FFB84D);
  color: white;
  cursor: default;
}

.btn-icon {
  width: 16px;
  height: 16px;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
