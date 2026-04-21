<script setup lang="ts">
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import type { MarketItem } from "@/types";
import { useMarketStore } from "@/stores/market";
import { useSkillsStore } from "@/stores/skills";
import { openUrl } from "@tauri-apps/plugin-opener";
import { Download, Check, X, Loader2, ExternalLink, AlertCircle, Package } from "lucide-vue-next";

const props = defineProps<{
  item: MarketItem;
}>();

const emit = defineEmits<{
  click: [item: MarketItem];
}>();

const { t } = useI18n();
const marketStore = useMarketStore();
const skillsStore = useSkillsStore();

const installing = ref(false);
const installError = ref<string | null>(null);
const installSuccess = ref(false);

const isInstalled = computed(() => props.item.installed);

// 获取项目 URL
const itemUrl = computed(() => {
  if (props.item.repo) {
    return props.item.repo;
  }
  // 根据 source 构建 URL
  if (props.item.source === "awesome-claude-skills") {
    return `https://github.com/ComposioHQ/awesome-claude-skills`;
  }
  if (props.item.source === "skills.sh") {
    return `https://skills.sh`;
  }
  return null;
});

async function handleInstall() {
  if (installing.value) return;
  installing.value = true;
  installError.value = null;
  try {
    await marketStore.installSkill(props.item.id);
    await skillsStore.loadSkills();
    installSuccess.value = true;
    setTimeout(() => {
      installSuccess.value = false;
    }, 2000);
  } catch (error) {
    installError.value = String(error);
  } finally {
    installing.value = false;
  }
}

function cancelInstall() {
  // 如果正在安装，可以取消（这里只是重置状态，实际取消需要更复杂的逻辑）
  if (installing.value) {
    installing.value = false;
    installError.value = null;
  }
}

async function openItemPage() {
  const url = itemUrl.value;
  if (url) {
    try {
      await openUrl(url);
    } catch (error) {
      console.error("Failed to open URL:", error);
      // 如果 Tauri API 失败，使用浏览器默认行为
      window.open(url, "_blank");
    }
  }
}

function formatDownloads(downloads?: number): string {
  if (!downloads) return "-";
  if (downloads >= 1000) {
    return (downloads / 1000).toFixed(1) + "k";
  }
  return downloads.toString();
}

function getSourceLabel(source: string): string {
  if (source === "awesome-claude-skills") return "Awesome";
  if (source === "skills.sh") return "Skills.sh";
  return source;
}

function getSourceColor(source: string): string {
  if (source === "awesome-claude-skills") return "lavender";
  if (source === "skills.sh") return "honey";
  return "cream";
}
</script>

<template>
  <div class="market-card" @click="emit('click', item)">
    <div class="card-glow"></div>
    <div class="card-content">
      <!-- 头部区域 -->
      <div class="card-header">
        <div class="header-main">
          <div class="item-icon">
            <Package class="icon-svg" :stroke-width="1.5" />
          </div>
          <div class="item-info">
            <h3 class="item-title" :title="item.name">{{ item.name }}</h3>
            <p class="item-description" :title="item.description">
              {{ item.description || t("market.noDescription") }}
            </p>
          </div>
        </div>
        <div class="header-actions">
          <!-- 安装成功状态 -->
          <button
            v-if="installSuccess"
            class="action-btn btn-success"
            disabled
            @click.stop
          >
            <Check class="btn-icon success-bounce" :stroke-width="2.5" />
            <span>{{ t("market.installSuccess") }}</span>
          </button>
          <!-- 已安装状态 -->
          <button
            v-else-if="isInstalled"
            class="action-btn btn-installed"
            disabled
            @click.stop
          >
            <Check class="btn-icon" :stroke-width="2.5" />
            <span>{{ t("market.installed") }}</span>
          </button>
          <!-- 安装中状态 -->
          <button
            v-else-if="installing"
            @click.stop="cancelInstall"
            class="action-btn btn-installing"
          >
            <Loader2 class="btn-icon animate-spin-slow" :stroke-width="1.5" />
            <span>{{ t("market.installing") }}</span>
          </button>
          <!-- 未安装状态 - 点击直接安装 -->
          <button
            v-else
            @click.stop="handleInstall"
            class="action-btn btn-install"
          >
            <Download class="btn-icon" :stroke-width="1.5" />
            <span>{{ t("market.install") }}</span>
          </button>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="installError" class="error-banner" @click.stop>
        <AlertCircle class="error-icon" :stroke-width="2" />
        <span class="error-text">{{ installError }}</span>
        <button @click.stop="installError = null" class="error-close">
          <X class="error-close-icon" :stroke-width="2" />
        </button>
      </div>

      <!-- 元信息标签 -->
      <div class="card-footer">
        <button
          v-if="itemUrl"
          @click.stop="openItemPage"
          class="meta-tag source-tag" :class="`tag-${getSourceColor(item.source)}`"
          :title="t('market.openPage')"
        >
          <ExternalLink class="tag-icon" :stroke-width="2" />
          {{ getSourceLabel(item.source) }}
        </button>
        <span v-else class="meta-tag" :class="`tag-${getSourceColor(item.source)}`">
          <ExternalLink class="tag-icon" :stroke-width="2" />
          {{ getSourceLabel(item.source) }}
        </span>
        <span v-if="item.author" class="meta-tag tag-cream">
          {{ t("market.author") }}: {{ item.author }}
        </span>
        <span v-if="item.downloads" class="meta-tag tag-mint">
          <Download class="tag-icon" :stroke-width="2" />
          {{ formatDownloads(item.downloads) }}
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ========== 卡片基础样式 ========== */
.market-card {
  position: relative;
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  overflow: hidden;
  transition: all var(--transition-normal) var(--ease-elastic);
  border: 2px solid var(--color-border-subtle);
}

.market-card:hover {
  transform: translateY(-4px) scale(1.01);
  box-shadow: var(--shadow-card-hover);
  border-color: var(--peach-200);
}

.card-glow {
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(255, 155, 173, 0.05), rgba(184, 169, 232, 0.05));
  opacity: 0;
  transition: opacity var(--transition-normal) var(--ease-out);
  pointer-events: none;
}

.market-card:hover .card-glow {
  opacity: 1;
}

.card-content {
  position: relative;
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

/* ========== 头部区域 ========== */
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--spacing-md);
}

.header-main {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md);
  flex: 1;
  min-width: 0;
}

.item-icon {
  width: 52px;
  height: 52px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--lavender-300), var(--lavender-400));
  border-radius: var(--radius-lg);
  color: white;
  flex-shrink: 0;
  box-shadow: var(--shadow-clay);
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-3px); }
}

.icon-svg {
  width: 26px;
  height: 26px;
}

.item-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.item-title {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: var(--font-heading);
}

.item-description {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-height: 1.5;
}

/* ========== 操作按钮区域 ========== */
.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 600;
  border: none;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  font-family: var(--font-body);
}

.btn-link {
  padding: var(--spacing-sm);
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border: 2px solid var(--color-border);
}

.btn-link:hover {
  background: var(--lavender-50);
  border-color: var(--lavender-200);
  color: var(--lavender-500);
  transform: translateY(-2px);
}

.btn-install {
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  color: white;
  box-shadow: var(--shadow-clay);
}

.btn-install:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

.btn-install:active:not(:disabled) {
  transform: scale(0.96);
  box-shadow: var(--shadow-pressed);
}

.btn-installing {
  background: linear-gradient(135deg, var(--honey), #FFB84D);
  color: white;
  box-shadow: var(--shadow-clay);
  cursor: pointer;
}

.btn-installing:hover {
  background: linear-gradient(135deg, var(--strawberry), #FF9999);
  transform: translateY(-2px);
}

.btn-installed {
  background: linear-gradient(135deg, var(--mint), #6BC49A);
  color: white;
  cursor: default;
  opacity: 0.9;
}

.btn-success {
  background: linear-gradient(135deg, var(--mint), #6BC49A);
  color: white;
  cursor: default;
}

.success-bounce {
  animation: successPop 0.5s var(--ease-elastic);
}

@keyframes successPop {
  0% { transform: scale(0); }
  50% { transform: scale(1.3); }
  100% { transform: scale(1); }
}

.btn-icon {
  width: 18px;
  height: 18px;
}

/* ========== 错误横幅 ========== */
.error-banner {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: linear-gradient(135deg, rgba(255, 123, 123, 0.1), rgba(255, 123, 123, 0.05));
  border: 1px solid rgba(255, 123, 123, 0.3);
  border-radius: var(--radius-md);
  animation: slideIn 0.3s var(--ease-out);
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.error-icon {
  width: 16px;
  height: 16px;
  color: var(--strawberry);
  flex-shrink: 0;
}

.error-text {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--strawberry);
}

.error-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--strawberry);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.error-close:hover {
  background: rgba(255, 123, 123, 0.1);
}

.error-close-icon {
  width: 14px;
  height: 14px;
}

/* ========== 元信息标签 ========== */
.card-footer {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-xs);
  padding-top: var(--spacing-sm);
  border-top: 1px solid var(--color-border-subtle);
}

.meta-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 500;
  transition: all var(--transition-fast) var(--ease-out);
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

.tag-mint {
  background: linear-gradient(135deg, rgba(125, 212, 168, 0.2), rgba(125, 212, 168, 0.1));
  color: #4A9B7A;
  border: 1px solid rgba(125, 212, 168, 0.4);
}

.tag-icon {
  width: 12px;
  height: 12px;
}

/* 来源标签按钮样式 */
.source-tag {
  cursor: pointer;
  border: none;
  font-family: inherit;
}

.source-tag:hover {
  transform: translateY(-1px);
  filter: brightness(1.05);
}

/* ========== 加载动画 ========== */
.animate-spin-slow {
  animation: spin 1.2s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
