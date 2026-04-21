<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Skill } from "@/types";
import { useSkillsStore } from "@/stores/skills";
import { useToolsStore } from "@/stores/tools";
import {
  X,
  Monitor,
  Link,
  Link2Off,
  CheckCircle,
  Circle,
  Search,
  AlertCircle,
} from "lucide-vue-next";

const props = defineProps<{
  skill: Skill;
  show: boolean;
}>();

const emit = defineEmits<{
  close: [];
}>();

const { t } = useI18n();
const skillsStore = useSkillsStore();
const toolsStore = useToolsStore();

const searchQuery = ref("");
const errorMessage = ref<string | null>(null);
const processing = ref(false);
const showOnlyEnabled = ref(false);

// 加载工具列表
watch(() => props.show, async (show) => {
  if (show) {
    await toolsStore.loadTools();
  }
}, { immediate: true });

// 已启用的工具（从已检测且已启用的工具中筛选）
const enabledTools = computed(() => {
  return toolsStore.detectedAndEnabledTools.filter(tool =>
    props.skill.enabledTools?.includes(tool.name)
  );
});

// 未启用的工具（从已检测且已启用的工具中筛选）
const disabledTools = computed(() => {
  return toolsStore.detectedAndEnabledTools.filter(tool =>
    !props.skill.enabledTools?.includes(tool.name)
  );
});

// 过滤后的已启用工具
const filteredEnabledTools = computed(() => {
  if (!searchQuery.value) return enabledTools.value;
  const query = searchQuery.value.toLowerCase();
  return enabledTools.value.filter(tool =>
    tool.name.toLowerCase().includes(query) ||
    tool.id.toLowerCase().includes(query)
  );
});

// 过滤后的未启用工具
const filteredDisabledTools = computed(() => {
  if (!searchQuery.value) return disabledTools.value;
  const query = searchQuery.value.toLowerCase();
  return disabledTools.value.filter(tool =>
    tool.name.toLowerCase().includes(query) ||
    tool.id.toLowerCase().includes(query)
  );
});

// 检查技能是否已在工具中启用
function isSkillEnabled(toolName: string): boolean {
  return props.skill.enabledTools?.includes(toolName) || false;
}

// 切换技能在工具中的启用状态
async function toggleSkillForTool(tool: { id: string; name: string }) {
  errorMessage.value = null;
  processing.value = true;

  const isEnabled = isSkillEnabled(tool.name);

  try {
    if (isEnabled) {
      // 停用技能
      const error = await toolsStore.disableSkillForTool(tool.id, props.skill.name);
      if (error) {
        errorMessage.value = error;
      }
    } else {
      // 启用技能
      const error = await toolsStore.enableSkillForTool(tool.id, props.skill.name);
      if (error) {
        errorMessage.value = error;
      }
    }

    // 刷新技能列表以同步数据
    await skillsStore.loadSkills();
    await toolsStore.loadTools();
  } catch (error) {
    console.error("Failed to toggle skill:", error);
    errorMessage.value = String(error);
  } finally {
    processing.value = false;
  }
}

// 一键启用到所有工具
async function enableAllTools() {
  errorMessage.value = null;
  processing.value = true;

  try {
    for (const tool of disabledTools.value) {
      const error = await toolsStore.enableSkillForTool(tool.id, props.skill.name);
      if (error) {
        errorMessage.value = error;
        break;
      }
    }
    await skillsStore.loadSkills();
    await toolsStore.loadTools();
  } catch (error) {
    console.error("Failed to enable all:", error);
    errorMessage.value = String(error);
  } finally {
    processing.value = false;
  }
}

// 一键停用从所有工具
async function disableAllTools() {
  errorMessage.value = null;
  processing.value = true;

  try {
    for (const tool of enabledTools.value) {
      const error = await toolsStore.disableSkillForTool(tool.id, props.skill.name);
      if (error) {
        errorMessage.value = error;
        break;
      }
    }
    await skillsStore.loadSkills();
    await toolsStore.loadTools();
  } catch (error) {
    console.error("Failed to disable all:", error);
    errorMessage.value = String(error);
  } finally {
    processing.value = false;
  }
}

function handleClose() {
  emit("close");
}
</script>

<template>
  <div v-if="show" class="dialog-overlay" @click="handleClose">
    <div class="dialog" @click.stop>
      <!-- 头部 -->
      <div class="dialog-header">
        <div class="header-main">
          <Monitor class="header-icon" :stroke-width="1.5" />
          <div class="header-info">
            <h2 class="dialog-title">管理技能配置</h2>
            <p class="dialog-subtitle">{{ skill.name }}</p>
          </div>
        </div>
        <button class="close-btn" @click="handleClose">
          <X class="close-icon" :stroke-width="2" />
        </button>
      </div>

      <!-- 内容 -->
      <div class="dialog-content">
        <!-- 批量操作按钮 -->
        <div class="batch-actions">
          <button
            @click="enableAllTools"
            class="text-btn"
            :disabled="processing || disabledTools.length === 0"
          >
            <Link class="btn-icon-small" :stroke-width="1.5" />
            全部开启
          </button>
          <button
            @click="disableAllTools"
            class="text-btn text-btn-danger"
            :disabled="processing || enabledTools.length === 0"
          >
            <Link2Off class="btn-icon-small" :stroke-width="1.5" />
            全部关闭
          </button>
        </div>

        <!-- 错误提示 -->
        <div v-if="errorMessage" class="error-alert">
          <AlertCircle class="h-4 w-4 flex-shrink-0" :stroke-width="1.5" />
          <span class="error-text">{{ errorMessage }}</span>
        </div>

        <!-- 搜索框和筛选 -->
        <div class="tools-filter-bar">
          <div class="search-box">
            <Search class="search-icon" :stroke-width="1.5" />
            <input
              v-model="searchQuery"
              type="text"
              :placeholder="t('tools.searchPlaceholder')"
              class="search-input"
            />
          </div>
          <label class="filter-toggle">
            <input
              type="checkbox"
              v-model="showOnlyEnabled"
            />
            <span>仅看已启用</span>
          </label>
        </div>

        <!-- 两列工具列表 -->
        <div class="tools-two-column">
          <!-- 已启用的工具 -->
          <div v-if="!showOnlyEnabled || enabledTools.length > 0" class="tools-column">
            <div class="tools-column-header">
              <CheckCircle class="column-icon enabled" :stroke-width="1.5" />
              <span class="column-title">已启用 ({{ enabledTools.length }})</span>
            </div>
            <div class="tools-column-list">
              <div
                v-for="tool in filteredEnabledTools"
                :key="tool.id"
                class="tool-row"
                :class="{ processing }"
              >
                <div class="tool-row-info">
                  <span class="tool-row-name">{{ tool.name }}</span>
                  <span class="tool-row-id">{{ tool.id }}</span>
                </div>
                <label class="switch">
                  <input
                    type="checkbox"
                    :checked="true"
                    @change="toggleSkillForTool(tool)"
                    :disabled="processing"
                  />
                  <span class="switch-slider"></span>
                </label>
              </div>
              <div v-if="enabledTools.length === 0" class="column-empty">
                暂无已启用的工具
              </div>
            </div>
          </div>

          <!-- 未启用的工具 -->
          <div v-if="!showOnlyEnabled && disabledTools.length > 0" class="tools-column">
            <div class="tools-column-header">
              <Circle class="column-icon disabled" :stroke-width="1.5" />
              <span class="column-title">未启用 ({{ disabledTools.length }})</span>
            </div>
            <div class="tools-column-list">
              <div
                v-for="tool in filteredDisabledTools"
                :key="tool.id"
                class="tool-row"
                :class="{ processing }"
              >
                <div class="tool-row-info">
                  <span class="tool-row-name">{{ tool.name }}</span>
                  <span class="tool-row-id">{{ tool.id }}</span>
                </div>
                <label class="switch">
                  <input
                    type="checkbox"
                    :checked="false"
                    @change="toggleSkillForTool(tool)"
                    :disabled="processing"
                  />
                  <span class="switch-slider"></span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- 无工具提示 -->
        <div v-if="toolsStore.detectedAndEnabledTools.length === 0" class="empty-tools">
          <Monitor class="empty-icon" :stroke-width="1.5" />
          <p>没有已检测并启用的IDE工具</p>
          <p class="empty-desc">请先在工具页面检测并启用至少一个IDE</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
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

.dialog {
  width: 90%;
  max-width: 700px;
  max-height: 80vh;
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  display: flex;
  flex-direction: column;
  animation: slideIn 0.3s var(--ease-out);
  overflow: hidden;
}

@keyframes slideIn {
  from {
    transform: translateY(-20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

/* 头部 */
.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg) var(--spacing-xl);
  border-bottom: 1px solid var(--color-border);
  background: linear-gradient(135deg, var(--lavender-50), var(--peach-50));
}

.header-main {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.header-icon {
  width: 40px;
  height: 40px;
  color: var(--lavender-400);
}

.header-info {
  flex: 1;
}

.dialog-title {
  font-size: var(--text-lg);
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0;
}

.dialog-subtitle {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: var(--spacing-xs) 0 0 0;
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

/* 内容 */
.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-lg) var(--spacing-xl);
}

/* 批量操作按钮 */
.batch-actions {
  display: flex;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
  padding-bottom: var(--spacing-md);
  border-bottom: 1px solid var(--color-border-subtle);
}

.text-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 6px 12px;
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-primary);
  font-size: var(--text-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.text-btn:hover:not(:disabled) {
  background: var(--color-primary-subtle);
  border-color: var(--color-primary);
}

.text-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.text-btn-danger {
  color: var(--strawberry);
}

.text-btn-danger:hover:not(:disabled) {
  background: var(--strawberry-50);
  border-color: var(--strawberry);
}

.btn-icon-small {
  width: 14px;
  height: 14px;
}

/* 错误提示 */
.error-alert {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--strawberry-50);
  border: 1px solid var(--strawberry-200);
  border-radius: var(--radius-md);
  color: var(--strawberry);
  font-size: var(--text-sm);
  margin-bottom: var(--spacing-md);
}

.error-text {
  flex: 1;
}

/* 工具筛选栏 */
.tools-filter-bar {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.tools-filter-bar .search-box {
  flex: 1;
  position: relative;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
}

.search-input {
  width: 100%;
  padding: 8px 12px 8px 36px;
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

.filter-toggle {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  cursor: pointer;
  white-space: nowrap;
}

.filter-toggle input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

/* 两列工具列表 */
.tools-two-column {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-lg);
  min-height: 200px;
}

.tools-column {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.tools-column-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--color-surface-subtle);
  border-radius: var(--radius-md);
}

.column-icon {
  width: 16px;
  height: 16px;
}

.column-icon.enabled {
  color: var(--grass-500);
}

.column-icon.disabled {
  color: var(--color-text-tertiary);
}

.column-title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-secondary);
}

.tools-column-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  max-height: 300px;
  overflow-y: auto;
}

.tool-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-row:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-border-strong);
}

.tool-row.processing {
  opacity: 0.6;
  pointer-events: none;
}

.tool-row-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.tool-row-name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tool-row-id {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.column-empty {
  text-align: center;
  padding: var(--spacing-lg);
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
}

/* 开关按钮样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  flex-shrink: 0;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.switch-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--color-border-strong);
  transition: .3s;
  border-radius: 24px;
}

.switch-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: .3s;
  border-radius: 50%;
}

.switch input:checked + .switch-slider {
  background-color: var(--grass-500);
}

.switch input:checked + .switch-slider:before {
  transform: translateX(20px);
}

.switch input:disabled + .switch-slider {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 空状态 */
.empty-tools {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xl);
  text-align: center;
  color: var(--color-text-secondary);
}

.empty-icon {
  width: 40px;
  height: 40px;
  color: var(--color-text-tertiary);
}

.empty-desc {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}
</style>
