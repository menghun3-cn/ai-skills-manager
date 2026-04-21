<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Skill, Tool } from "@/types";
import { useToolsStore } from "@/stores/tools";
import { useSkillsStore } from "@/stores/skills";
import { X, Search, Check, Monitor, Link, Link2Off } from "lucide-vue-next";

const props = defineProps<{
  show: boolean;
  skill: Skill;
}>();

const emit = defineEmits<{
  close: [];
}>();

const { t } = useI18n();
const toolsStore = useToolsStore();
const skillsStore = useSkillsStore();

const searchQuery = ref("");
const loading = ref(false);
const errorMessage = ref<string | null>(null);

// 加载工具列表
watch(() => props.show, async (show) => {
  if (show) {
    loading.value = true;
    await toolsStore.loadTools();
    loading.value = false;
  }
});

// 过滤工具
const filteredTools = computed(() => {
  if (!searchQuery.value) return toolsStore.enabledTools;
  const query = searchQuery.value.toLowerCase();
  return toolsStore.enabledTools.filter(tool => 
    tool.name.toLowerCase().includes(query) ||
    tool.id.toLowerCase().includes(query)
  );
});

// 检查技能是否已在工具中启用
function isSkillEnabled(toolName: string): boolean {
  return props.skill.enabledTools?.includes(toolName) || false;
}

// 切换技能在工具中的启用状态
async function toggleSkillForTool(tool: Tool) {
  errorMessage.value = null;
  loading.value = true;

  const isEnabled = isSkillEnabled(tool.name);

  try {
    if (isEnabled) {
      // 停用技能
      const error = await toolsStore.disableSkillForTool(tool.id, props.skill.name);
      if (error) {
        errorMessage.value = error;
      } else {
        // 更新本地技能数据
        const index = props.skill.enabledTools?.indexOf(tool.name);
        if (index !== undefined && index > -1) {
          props.skill.enabledTools?.splice(index, 1);
        }
      }
    } else {
      // 启用技能
      const error = await toolsStore.enableSkillForTool(tool.id, props.skill.name);
      if (error) {
        errorMessage.value = error;
      } else {
        // 更新本地技能数据
        if (!props.skill.enabledTools) {
          props.skill.enabledTools = [];
        }
        props.skill.enabledTools.push(tool.name);
      }
    }
    
    // 刷新技能列表以同步数据
    await skillsStore.loadSkills();
  } catch (error) {
    console.error("Failed to toggle skill:", error);
    errorMessage.value = String(error);
  } finally {
    loading.value = false;
  }
}

function close() {
  emit("close");
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="show" class="dialog-overlay" @click.self="close">
        <div class="dialog-container">
          <!-- 对话框头部 -->
          <div class="dialog-header">
            <div class="header-content">
              <div class="skill-icon">
                <Monitor class="h-5 w-5" :stroke-width="1.5" />
              </div>
              <div class="header-text">
                <h3 class="dialog-title">{{ t("skills.manageTools") }}</h3>
                <p class="dialog-subtitle">{{ skill.name }}</p>
              </div>
            </div>
            <button @click="close" class="close-btn" aria-label="关闭">
              <X class="h-5 w-5" :stroke-width="1.5" />
            </button>
          </div>

          <!-- 错误提示 -->
          <div v-if="errorMessage" class="error-alert">
            {{ errorMessage }}
          </div>

          <!-- 搜索框 -->
          <div class="search-section">
            <div class="search-box">
              <Search class="search-icon" :stroke-width="1.5" />
              <input
                v-model="searchQuery"
                type="text"
                :placeholder="t('tools.searchPlaceholder')"
                class="search-input"
              />
            </div>
          </div>

          <!-- 工具列表 -->
          <div class="tools-section">
            <div v-if="loading" class="loading-state">
              <div class="loading-spinner">
                <div class="spinner-dot"></div>
                <div class="spinner-dot"></div>
                <div class="spinner-dot"></div>
              </div>
              <span class="loading-text">{{ t("common.loading") }}</span>
            </div>

            <div v-else-if="toolsStore.enabledTools.length === 0" class="empty-state">
              <Monitor class="empty-icon" :stroke-width="1.5" />
              <p class="empty-title">{{ t("skills.noEnabledTools") }}</p>
              <p class="empty-desc">{{ t("skills.enableToolsFirst") }}</p>
            </div>

            <div v-else-if="filteredTools.length === 0" class="empty-state">
              <Search class="empty-icon" :stroke-width="1.5" />
              <p class="empty-title">{{ t("common.noResults") }}</p>
              <p class="empty-desc">{{ t("market.noResultsDesc") }}</p>
            </div>

            <div v-else class="tools-list">
              <div
                v-for="tool in filteredTools"
                :key="tool.id"
                class="tool-item"
                :class="{ enabled: isSkillEnabled(tool.name) }"
                @click="toggleSkillForTool(tool)"
              >
                <div class="tool-checkbox">
                  <Check v-if="isSkillEnabled(tool.name)" class="check-icon" :stroke-width="2" />
                </div>
                <div class="tool-info">
                  <span class="tool-name">{{ tool.name }}</span>
                  <span class="tool-id">{{ tool.id }}</span>
                </div>
                <div class="tool-status">
                  <Link v-if="isSkillEnabled(tool.name)" class="status-icon enabled" :stroke-width="1.5" />
                  <Link2Off v-else class="status-icon disabled" :stroke-width="1.5" />
                  <span class="status-text" :class="isSkillEnabled(tool.name) ? 'enabled' : 'disabled'">
                    {{ isSkillEnabled(tool.name) ? t("skills.enabled") : t("skills.disabled") }}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- 对话框底部 -->
          <div class="dialog-footer">
            <span class="footer-info">
              {{ t("skills.enabledInTools", { count: skill.enabledTools?.length || 0 }) }}
            </span>
            <button @click="close" class="btn btn-secondary">
              {{ t("common.close") }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: var(--spacing-lg);
}

.dialog-container {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  width: 100%;
  max-width: 560px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-xl);
  animation: dialog-enter 0.2s var(--ease-out);
}

@keyframes dialog-enter {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.header-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.skill-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-primary-subtle), var(--color-accent-subtle));
  border-radius: var(--radius-lg);
  color: var(--color-primary);
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.dialog-title {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
}

.dialog-subtitle {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.close-btn:hover {
  background: var(--color-surface-hover);
  color: var(--color-text);
}

.error-alert {
  margin: var(--spacing-md) var(--spacing-lg) 0;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--strawberry-50);
  border: 1px solid var(--strawberry-200);
  border-radius: var(--radius-md);
  color: var(--strawberry);
  font-size: var(--text-sm);
}

.search-section {
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.search-box {
  position: relative;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  width: 18px;
  height: 18px;
  color: var(--color-text-tertiary);
}

.search-input {
  width: 100%;
  padding: 10px 12px 10px 40px;
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  color: var(--color-text);
  transition: all var(--transition-fast) var(--ease-out);
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary);
  background: var(--color-surface);
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

.tools-section {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
  min-height: 200px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-md);
  padding: var(--spacing-xl);
}

.loading-spinner {
  display: flex;
  gap: 6px;
}

.spinner-dot {
  width: 8px;
  height: 8px;
  background: var(--color-primary);
  border-radius: 50%;
  animation: bounce 1.4s infinite ease-in-out both;
}

.spinner-dot:nth-child(1) { animation-delay: -0.32s; }
.spinner-dot:nth-child(2) { animation-delay: -0.16s; }

@keyframes bounce {
  0%, 80%, 100% { transform: scale(0); }
  40% { transform: scale(1); }
}

.loading-text {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xl);
  text-align: center;
}

.empty-icon {
  width: 48px;
  height: 48px;
  color: var(--color-text-tertiary);
}

.empty-title {
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text);
  margin: 0;
}

.empty-desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
}

.tools-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.tool-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md);
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-item:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-border-strong);
}

.tool-item.enabled {
  background: var(--color-primary-subtle);
  border-color: var(--color-primary);
}

.tool-checkbox {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-surface);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-sm);
  flex-shrink: 0;
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-item.enabled .tool-checkbox {
  background: var(--color-primary);
  border-color: var(--color-primary);
}

.check-icon {
  width: 14px;
  height: 14px;
  color: white;
}

.tool-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.tool-name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tool-id {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
}

.tool-status {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

.status-icon {
  width: 16px;
  height: 16px;
}

.status-icon.enabled {
  color: var(--color-primary);
}

.status-icon.disabled {
  color: var(--color-text-tertiary);
}

.status-text {
  font-size: var(--text-xs);
  font-weight: 500;
}

.status-text.enabled {
  color: var(--color-primary);
}

.status-text.disabled {
  color: var(--color-text-tertiary);
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--color-border);
  gap: var(--spacing-md);
}

.footer-info {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  border: 1px solid transparent;
}

.btn-secondary {
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.btn-secondary:hover {
  background: var(--color-surface-hover);
  color: var(--color-text);
  border-color: var(--color-border-strong);
}

/* Transition animations */
.dialog-enter-active,
.dialog-leave-active {
  transition: all 0.2s var(--ease-out);
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
