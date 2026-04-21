<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { useI18n } from "vue-i18n";
import type { Tool } from "@/types";
import { useToolsStore } from "@/stores/tools";
import { FolderOpen, Settings2, X, Wrench, AlertCircle, Search, Link, Link2Off, ScanLine, Scan } from "lucide-vue-next";

const props = defineProps<{
  tool: Tool;
}>();

const { t } = useI18n();
const toolsStore = useToolsStore();

const showSkillsDialog = ref(false);
const loading = ref(false);
const errorMessage = ref<string | null>(null);
const searchQuery = ref("");
const showOnlyEnabled = ref(false);
const processing = ref(false);



// 已启用的技能列表
const enabledSkills = computed(() => {
  return toolsStore.installedSkills.filter(skill =>
    toolsStore.isSkillEnabled(props.tool.id, skill.name)
  );
});

// 未启用的技能列表
const disabledSkills = computed(() => {
  return toolsStore.installedSkills.filter(skill =>
    !toolsStore.isSkillEnabled(props.tool.id, skill.name)
  );
});

// 过滤后的已启用技能
const filteredEnabledSkills = computed(() => {
  if (!searchQuery.value) return enabledSkills.value;
  const query = searchQuery.value.toLowerCase();
  return enabledSkills.value.filter(skill =>
    skill.name.toLowerCase().includes(query) ||
    (skill.description?.toLowerCase().includes(query) ?? false)
  );
});

// 过滤后的未启用技能
const filteredDisabledSkills = computed(() => {
  if (!searchQuery.value) return disabledSkills.value;
  const query = searchQuery.value.toLowerCase();
  return disabledSkills.value.filter(skill =>
    skill.name.toLowerCase().includes(query) ||
    (skill.description?.toLowerCase().includes(query) ?? false)
  );
});

// 监听对话框显示，加载技能列表
watch(showSkillsDialog, async (show) => {
  if (show) {
    loading.value = true;
    errorMessage.value = null;
    await toolsStore.loadInstalledSkills();
    await toolsStore.loadToolEnabledSkills(props.tool.id);
    loading.value = false;
  }
});

async function handleToggle() {
  await toolsStore.toggleTool(props.tool.id);
}

async function handleOpenConfig() {
  const error = await toolsStore.openPath(props.tool.configPath);
  if (error) {
    errorMessage.value = error;
    setTimeout(() => {
      errorMessage.value = null;
    }, 3000);
  }
}

async function handleOpenSkills() {
  const error = await toolsStore.openPath(props.tool.skillsPath);
  if (error) {
    errorMessage.value = error;
    setTimeout(() => {
      errorMessage.value = null;
    }, 3000);
  }
}

async function handleToggleSkill(skillName: string) {
  errorMessage.value = null;
  processing.value = true;
  const isEnabled = toolsStore.isSkillEnabled(props.tool.id, skillName);

  if (isEnabled) {
    // 停用技能
    const error = await toolsStore.disableSkillForTool(props.tool.id, skillName);
    if (error) {
      errorMessage.value = error;
    }
  } else {
    // 启用技能
    const error = await toolsStore.enableSkillForTool(props.tool.id, skillName);
    if (error) {
      errorMessage.value = error;
    }
  }
  processing.value = false;
}

// 一键启用所有技能
async function enableAllSkills() {
  errorMessage.value = null;
  processing.value = true;

  for (const skill of disabledSkills.value) {
    const error = await toolsStore.enableSkillForTool(props.tool.id, skill.name);
    if (error) {
      errorMessage.value = error;
      break;
    }
  }
  processing.value = false;
}

// 一键停用所有技能
async function disableAllSkills() {
  errorMessage.value = null;
  processing.value = true;

  for (const skill of enabledSkills.value) {
    const error = await toolsStore.disableSkillForTool(props.tool.id, skill.name);
    if (error) {
      errorMessage.value = error;
      break;
    }
  }
  processing.value = false;
}
</script>

<template>
  <div class="card">
    <div class="card-header">
      <div class="tool-icon">
        <Wrench class="h-5 w-5" :stroke-width="1.5" />
      </div>
      <div class="tool-info">
        <h3 class="tool-title">{{ tool.name }}</h3>
        <div class="tool-meta">
          <p class="tool-id">ID: {{ tool.id }}</p>
          <span
            class="detect-badge"
            :class="tool.detected ? 'detected' : 'not-detected'"
          >
            <component :is="tool.detected ? ScanLine : Scan" class="detect-icon" :stroke-width="1.5" />
            {{ tool.detected ? '已检测到' : '未检测到' }}
          </span>
        </div>
      </div>
      <button
        @click="handleToggle"
        class="toggle-switch"
        :class="tool.enabled ? 'enabled' : 'disabled'"
        :aria-label="tool.enabled ? t('tools.disable') : t('tools.enable')"
      >
        <span class="toggle-knob" />
      </button>
    </div>

    <div class="path-list">
      <div class="path-item">
        <span class="path-label">{{ t("tools.configPath") }}</span>
        <div class="path-value">
          <span class="path-text">{{ tool.configPath }}</span>
          <button
            @click="handleOpenConfig"
            class="path-btn"
            :aria-label="t('tools.open')"
          >
            <FolderOpen class="h-4 w-4" :stroke-width="1.5" />
          </button>
        </div>
      </div>
      <div class="path-item">
        <span class="path-label">{{ t("tools.skillsPath") }}</span>
        <div class="path-value">
          <span class="path-text">{{ tool.skillsPath }}</span>
          <button
            @click="handleOpenSkills"
            class="path-btn"
            :aria-label="t('tools.open')"
          >
            <FolderOpen class="h-4 w-4" :stroke-width="1.5" />
          </button>
        </div>
      </div>
    </div>

    <div class="card-footer">
      <span
        class="badge"
        :class="tool.enabled ? 'badge-success' : 'badge-default'"
      >
        {{ tool.enabled ? t("tools.enabled") : t("tools.disabled") }}
      </span>
      <button
        @click="showSkillsDialog = true"
        class="manage-btn"
      >
        <Settings2 class="h-4 w-4" :stroke-width="1.5" />
        {{ t("tools.manage") }}
      </button>
    </div>

    <div v-if="showSkillsDialog" class="dialog-overlay">
      <div class="dialog">
        <div class="dialog-header">
          <h3 class="dialog-title">
            {{ t("tools.toolSkills") }}
          </h3>
          <button
            @click="showSkillsDialog = false"
            class="dialog-close"
            :aria-label="t('tools.close')"
          >
            <X class="h-5 w-5" :stroke-width="1.5" />
          </button>
        </div>
        <p class="dialog-desc">
          {{ t("tools.toolSkillsDesc") }}
        </p>
        
        <!-- 错误提示 -->
        <div v-if="errorMessage" class="error-alert">
          <AlertCircle class="h-5 w-5 flex-shrink-0" :stroke-width="1.5" />
          <span class="error-text">{{ errorMessage }}</span>
        </div>

        <!-- 工具未启用提示 -->
        <div v-if="!tool.enabled" class="warning-alert">
          <AlertCircle class="h-5 w-5 flex-shrink-0" :stroke-width="1.5" />
          <span class="warning-text">工具未启用，请先启用工具才能管理技能</span>
        </div>

        <div class="dialog-content">
          <!-- 加载状态 -->
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner">
              <div class="spinner-dot"></div>
              <div class="spinner-dot"></div>
              <div class="spinner-dot"></div>
            </div>
            <span class="loading-text">加载中...</span>
          </div>

          <!-- 空状态 -->
          <div v-else-if="toolsStore.installedSkills.length === 0" class="empty-state">
            <p class="empty-text">暂无已安装的技能</p>
          </div>

          <!-- 技能管理 -->
          <div v-else class="skills-manager">
            <!-- 搜索和批量操作栏 -->
            <div class="skills-filter-bar">
              <div class="search-box">
                <Search class="search-icon" :stroke-width="1.5" />
                <input
                  v-model="searchQuery"
                  type="text"
                  placeholder="搜索技能..."
                  class="search-input"
                />
              </div>
              <!-- 批量操作按钮 -->
              <div class="batch-actions">
                <button
                  @click="enableAllSkills"
                  class="text-btn"
                  :disabled="processing || disabledSkills.length === 0 || !tool.enabled"
                >
                  <Link class="btn-icon-small" :stroke-width="1.5" />
                  全部开启
                </button>
                <button
                  @click="disableAllSkills"
                  class="text-btn text-btn-danger"
                  :disabled="processing || enabledSkills.length === 0 || !tool.enabled"
                >
                  <Link2Off class="btn-icon-small" :stroke-width="1.5" />
                  全部关闭
                </button>
              </div>
            </div>

            <!-- 技能卡片网格 -->
            <div class="skills-grid">
              <!-- 已启用的技能 -->
              <div
                v-for="skill in filteredEnabledSkills"
                :key="skill.name"
                class="skill-card"
                :class="{ processing, 'skill-disabled': !tool.enabled, 'skill-enabled': true }"
              >
                <div class="skill-card-info">
                  <span class="skill-card-name">{{ skill.name }}</span>
                  <span v-if="skill.description" class="skill-card-desc">{{ skill.description }}</span>
                </div>
                <label class="switch">
                  <input
                    type="checkbox"
                    :checked="true"
                    @change="handleToggleSkill(skill.name)"
                    :disabled="processing || !tool.enabled"
                  />
                  <span class="switch-slider"></span>
                </label>
              </div>

              <!-- 未启用的技能 -->
              <div
                v-for="skill in filteredDisabledSkills"
                :key="skill.name"
                class="skill-card"
                :class="{ processing, 'skill-disabled': !tool.enabled }"
              >
                <div class="skill-card-info">
                  <span class="skill-card-name">{{ skill.name }}</span>
                  <span v-if="skill.description" class="skill-card-desc">{{ skill.description }}</span>
                </div>
                <label class="switch">
                  <input
                    type="checkbox"
                    :checked="false"
                    @change="handleToggleSkill(skill.name)"
                    :disabled="processing || !tool.enabled"
                  />
                  <span class="switch-slider"></span>
                </label>
              </div>

              <!-- 空状态 -->
              <div v-if="enabledSkills.length === 0 && disabledSkills.length === 0" class="skills-empty">
                暂无技能
              </div>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button
            @click="showSkillsDialog = false"
            class="btn btn-secondary"
          >
            {{ t("tools.close") }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  padding: var(--spacing-md);
  transition: all var(--transition-fast) var(--ease-out);
}

.card:hover {
  border-color: var(--color-border-strong);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 16px;
}

.tool-icon {
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-primary-subtle), var(--color-accent-subtle));
  border-radius: var(--radius-lg);
  color: var(--color-primary);
  flex-shrink: 0;
}

.tool-info {
  flex: 1;
  min-width: 0;
}

.tool-title {
  font-size: var(--text-base);
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.tool-id {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin: 0;
}

.tool-meta {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-top: 4px;
  flex-wrap: wrap;
}

.detect-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  white-space: nowrap;
}

.detect-badge.detected {
  background: var(--grass-100);
  color: var(--grass-600);
}

.detect-badge.not-detected {
  background: var(--color-surface-subtle);
  color: var(--color-text-tertiary);
}

.detect-icon {
  width: 12px;
  height: 12px;
}

.toggle-switch {
  position: relative;
  width: 44px;
  height: 24px;
  border-radius: 12px;
  border: none;
  cursor: pointer;
  transition: background-color var(--transition-fast) var(--ease-out);
  flex-shrink: 0;
}

.toggle-switch.enabled {
  background: var(--color-success);
}

.toggle-switch.disabled {
  background: var(--color-border);
}

.toggle-knob {
  position: absolute;
  top: 2px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  transition: left var(--transition-fast) var(--ease-out);
}

.toggle-switch.enabled .toggle-knob {
  left: 22px;
}

.toggle-switch.disabled .toggle-knob {
  left: 2px;
}

.path-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
}

.path-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.path-label {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

.path-value {
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-text {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.path-btn {
  padding: 4px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--color-primary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.path-btn:hover {
  background: var(--color-primary-subtle);
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 16px;
  border-top: 1px solid var(--color-border-subtle);
}

.manage-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-primary);
  background: transparent;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.manage-btn:hover {
  background: var(--color-primary-subtle);
}

.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
}

.dialog {
  width: 900px;
  min-width: 900px;
  max-width: 95vw;
  max-height: 80vh;
  background: var(--color-surface-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  padding: var(--spacing-xl);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.dialog-title {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.dialog-close {
  padding: 4px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.dialog-close:hover {
  color: var(--color-text-primary);
  background: var(--color-border-subtle);
}

.dialog-desc {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0 0 16px 0;
}

.dialog-content {
  margin-bottom: 16px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
}

.error-alert {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  margin-bottom: 16px;
  background: var(--color-error-subtle, #fee2e2);
  border: 1px solid var(--color-error, #ef4444);
  border-radius: var(--radius-md);
  color: var(--color-error, #dc2626);
}

.error-text {
  font-size: var(--text-sm);
}

.warning-alert {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  margin-bottom: 16px;
  background: var(--color-warning-subtle, #fef3c7);
  border: 1px solid var(--color-warning, #f59e0b);
  border-radius: var(--radius-md);
  color: var(--color-warning, #d97706);
}

.warning-text {
  font-size: var(--text-sm);
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px;
  gap: 12px;
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

.spinner-dot:nth-child(1) {
  animation-delay: -0.32s;
}

.spinner-dot:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

.loading-text {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 32px;
}

.empty-text {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.skills-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.skills-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: var(--color-surface-subtle);
  border-radius: var(--radius-md);
}

.skills-count {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.skills-enabled {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
}

.skills-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.skill-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast) var(--ease-out);
}

.skill-item:hover {
  border-color: var(--color-border-strong);
}

.skill-item.skill-disabled {
  opacity: 0.6;
}

.skill-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  min-width: 0;
}

.skill-name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}

.skill-desc {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.skill-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 4px;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.skill-toggle.enabled {
  color: var(--color-success, #22c55e);
}

.skill-toggle.disabled {
  color: var(--color-text-tertiary);
}

.skill-toggle:hover:not(:disabled) {
  background: var(--color-surface-subtle);
}

.skill-toggle:disabled {
  cursor: not-allowed;
}

/* 技能管理 */
.skills-manager {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

/* 技能筛选栏 */
.skills-filter-bar {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.skills-filter-bar .search-box {
  flex: 1;
  position: relative;
}

.skills-filter-bar .search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
}

.skills-filter-bar .search-input {
  width: 100%;
  padding: 8px 12px 8px 36px;
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  color: var(--color-text);
  transition: all var(--transition-fast) var(--ease-out);
}

.skills-filter-bar .search-input:focus {
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

/* 批量操作按钮 */
.batch-actions {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
  flex-shrink: 0;
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
  border-color: var(--color-border);
}

.text-btn-danger:hover:not(:disabled) {
  background: var(--strawberry-50);
  border-color: var(--strawberry);
}

.btn-icon-small {
  width: 14px;
  height: 14px;
}

/* 技能卡片网格 - 横向两列 */
.skills-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-lg);
  min-height: 200px;
  max-height: 400px;
  overflow-y: auto;
  padding-right: var(--spacing-sm);
}

.skill-card {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  transition: all var(--transition-fast) var(--ease-out);
  min-height: 80px;
}

.skill-card:hover {
  border-color: var(--color-border-strong);
  box-shadow: var(--shadow-sm);
}

.skill-card.skill-enabled {
  border-left: 3px solid var(--color-success);
  background: linear-gradient(135deg, var(--color-surface) 0%, rgba(125, 212, 168, 0.08) 100%);
}

.skill-card:not(.skill-enabled) {
  border-left: 3px solid var(--color-border);
}

.skill-card.processing {
  opacity: 0.6;
  pointer-events: none;
}

.skill-card.skill-disabled {
  opacity: 0.5;
}

.skill-card-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.skill-card-name {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.skill-card-desc {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-height: 1.4;
}

.skills-empty {
  grid-column: span 2;
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
}

/* 开关按钮样式 - 加大并增强视觉效果 */
.switch {
  position: relative;
  display: inline-block;
  width: 56px;
  height: 32px;
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
  transition: all 0.3s ease;
  border-radius: 32px;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
}

.switch-slider:before {
  position: absolute;
  content: "";
  height: 26px;
  width: 26px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: all 0.3s ease;
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.switch input:checked + .switch-slider {
  background-color: var(--color-success);
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1), 0 0 10px rgba(125, 212, 168, 0.4);
}

.switch input:checked + .switch-slider:before {
  transform: translateX(24px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2), 0 0 8px rgba(255, 255, 255, 0.5);
}

.switch input:not(:checked) + .switch-slider {
  background-color: var(--strawberry-300);
}

.switch input:not(:checked) + .switch-slider:before {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.switch input:disabled + .switch-slider {
  opacity: 0.4;
  cursor: not-allowed;
  background-color: var(--color-border);
}

.switch input:disabled + .switch-slider:before {
  background-color: var(--color-surface-subtle);
}
</style>
