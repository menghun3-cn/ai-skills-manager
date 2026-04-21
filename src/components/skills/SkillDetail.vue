<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import type { Skill, SkillFile } from "@/types";
import { useSkillsStore } from "@/stores/skills";
import { invoke } from "@tauri-apps/api/core";
import ConfirmDialog from "@/components/common/ConfirmDialog.vue";
import {
  X,
  Cpu,
  Globe,
  Folder,
  FileText,
  Calendar,
  User,
  Tag,
  CheckCircle,
  Circle,
  RefreshCw,
  Trash2,
  Pencil,
} from "lucide-vue-next";

const props = defineProps<{
  skill: Skill;
}>();

const emit = defineEmits<{
  close: [];
  edit: [skillName: string];
  delete: [skillName: string];
}>();

const { t } = useI18n();
const skillsStore = useSkillsStore();

const skillFiles = ref<SkillFile[]>([]);
const loading = ref(false);
const showDeleteDialog = ref(false);

const bindingText = computed(() => {
  if (props.skill.bindings.global) {
    return t("skills.global");
  }
  if (props.skill.bindings.projects.length > 0) {
    return t("skills.projectBound");
  }
  return t("skills.noBinding");
});

const bindingIcon = computed(() => {
  if (props.skill.bindings.global) {
    return Globe;
  }
  return Folder;
});

const bindingDetail = computed(() => {
  if (props.skill.bindings.global) {
    return t("skills.globalDesc");
  }
  if (props.skill.bindings.projects.length > 0) {
    return props.skill.bindings.projects.join(", ");
  }
  return "";
});

const formattedDate = computed(() => {
  if (!props.skill.updatedAt) return "-";
  try {
    const date = new Date(props.skill.updatedAt);
    return date.toLocaleDateString();
  } catch {
    return props.skill.updatedAt;
  }
});

onMounted(async () => {
  await loadSkillFiles();
});

async function loadSkillFiles() {
  loading.value = true;
  try {
    const files = await invoke<SkillFile[]>("get_skill_files", {
      skillName: props.skill.name,
    });
    skillFiles.value = files;
  } catch (error) {
    console.error("Failed to load skill files:", error);
  } finally {
    loading.value = false;
  }
}

function handleEdit() {
  emit("edit", props.skill.name);
}

function handleDelete() {
  showDeleteDialog.value = true;
}

function confirmDelete() {
  showDeleteDialog.value = false;
  emit("delete", props.skill.name);
}

async function handleSync() {
  try {
    await skillsStore.syncSkill(props.skill.name);
    await loadSkillFiles();
  } catch (error) {
    console.error("Failed to sync skill:", error);
  }
}

function handleClose() {
  emit("close");
}
</script>

<template>
  <div class="skill-detail-overlay" @click="handleClose">
    <div class="skill-detail-panel" @click.stop>
      <!-- 头部 -->
      <div class="detail-header">
        <div class="header-main">
          <div class="skill-icon-large">
            <Cpu class="icon-svg" :stroke-width="1.5" />
          </div>
          <div class="header-info">
            <h2 class="skill-name">{{ skill.name }}</h2>
            <div class="skill-status">
              <component :is="bindingIcon" class="status-icon" :stroke-width="1.5" />
              <span>{{ bindingText }}</span>
              <span v-if="bindingDetail" class="status-detail">({{ bindingDetail }})</span>
            </div>
          </div>
        </div>
        <button class="close-btn" @click="handleClose">
          <X class="close-icon" :stroke-width="2" />
        </button>
      </div>

      <!-- 描述 -->
      <div class="detail-section">
        <h3 class="section-title">{{ t("skills.description") }}</h3>
        <p class="description-text">
          {{ skill.description || t("skills.noDescription") }}
        </p>
      </div>

      <!-- 元信息 -->
      <div class="detail-section">
        <h3 class="section-title">{{ t("skills.info") }}</h3>
        <div class="info-grid">
          <div class="info-item">
            <User class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("skills.author") }}:</span>
            <span class="info-value">{{ skill.author || "-" }}</span>
          </div>
          <div class="info-item">
            <Tag class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("skills.version") }}:</span>
            <span class="info-value">{{ skill.version || "-" }}</span>
          </div>
          <div class="info-item">
            <Calendar class="info-icon" :stroke-width="1.5" />
            <span class="info-label">{{ t("skills.updatedAt") }}:</span>
            <span class="info-value">{{ formattedDate }}</span>
          </div>
          <div class="info-item">
            <component
              :is="skill.enabled ? CheckCircle : Circle"
              class="info-icon"
              :class="skill.enabled ? 'enabled' : 'disabled'"
              :stroke-width="1.5"
            />
            <span class="info-label">{{ t("skills.status") }}:</span>
            <span class="info-value" :class="skill.enabled ? 'enabled' : 'disabled'">
              {{ skill.enabled ? t("skills.enabled") : t("skills.disabled") }}
            </span>
          </div>
        </div>
      </div>

      <!-- 文件列表 -->
      <div class="detail-section">
        <h3 class="section-title">
          {{ t("skills.files") }} ({{ skillFiles.length }})
        </h3>
        <div v-if="loading" class="loading-state">
          <RefreshCw class="loading-icon animate-spin" :stroke-width="1.5" />
          <span>{{ t("common.loading") }}</span>
        </div>
        <div v-else-if="skillFiles.length === 0" class="empty-state">
          {{ t("skills.noFiles") }}
        </div>
        <ul v-else class="files-list">
          <li v-for="file in skillFiles" :key="file.path" class="file-item">
            <FileText class="file-icon" :stroke-width="1.5" />
            <span class="file-name">{{ file.name }}</span>
          </li>
        </ul>
      </div>

      <!-- 操作按钮 -->
      <div class="detail-actions">
        <button class="action-btn btn-secondary" @click="handleSync">
          <RefreshCw class="btn-icon" :stroke-width="1.5" />
          {{ t("skills.sync") }}
        </button>
        <button class="action-btn btn-secondary" @click="handleEdit">
          <Pencil class="btn-icon" :stroke-width="1.5" />
          {{ t("skills.edit") }}
        </button>
        <button class="action-btn btn-danger" @click="handleDelete">
          <Trash2 class="btn-icon" :stroke-width="1.5" />
          {{ t("skills.delete") }}
        </button>
      </div>
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :show="showDeleteDialog"
      :title="t('skills.confirmDelete')"
      :message="t('skills.confirmDeleteDesc')"
      type="danger"
      @confirm="confirmDelete"
      @cancel="showDeleteDialog = false"
    />
  </div>
</template>

<style scoped>
.skill-detail-overlay {
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

.skill-detail-panel {
  width: 480px;
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
  background: linear-gradient(135deg, var(--lavender-50), var(--peach-50));
}

.header-main {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.skill-icon-large {
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--lavender-300), var(--lavender-400));
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

.skill-name {
  font-size: var(--text-xl);
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-xs) 0;
  font-family: var(--font-heading);
}

.skill-status {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.status-icon {
  width: 14px;
  height: 14px;
  color: var(--lavender-400);
}

.status-detail {
  color: var(--color-text-tertiary);
  font-size: var(--text-xs);
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
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.section-icon {
  width: 16px;
  height: 16px;
}

.description-text {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  line-height: 1.6;
  margin: 0;
}

/* 信息网格 */
.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--spacing-md);
}

.info-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--text-sm);
}

.info-icon {
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
  flex-shrink: 0;
}

.info-icon.enabled {
  color: var(--grass-500);
}

.info-icon.disabled {
  color: var(--color-text-tertiary);
}

.info-label {
  color: var(--color-text-secondary);
  flex-shrink: 0;
}

.info-value {
  color: var(--color-text-primary);
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.info-value.enabled {
  color: var(--grass-500);
}

.info-value.disabled {
  color: var(--color-text-tertiary);
}

.search-box {
  position: relative;
  margin-bottom: var(--spacing-md);
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

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

.tools-group {
  margin-bottom: var(--spacing-md);
}

.tools-group-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-sm);
  padding: 0 var(--spacing-xs);
}

.group-icon {
  width: 14px;
  height: 14px;
}

.group-icon.enabled {
  color: var(--grass-500);
}

.group-icon.disabled {
  color: var(--color-text-tertiary);
}

.group-title {
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
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
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-item:hover:not(.processing) {
  background: var(--color-surface-hover);
  border-color: var(--color-border-strong);
}

.tool-item.processing {
  opacity: 0.6;
  cursor: not-allowed;
}

.tool-checkbox {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-surface);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-sm);
  flex-shrink: 0;
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-checkbox.enabled {
  background: var(--grass-500);
  border-color: var(--grass-500);
}

.check-icon {
  width: 12px;
  height: 12px;
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

.tool-status-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.tool-status-icon.enabled {
  color: var(--grass-500);
}

.tool-status-icon.disabled {
  color: var(--color-text-tertiary);
}

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

/* 文件列表 */
.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg);
  color: var(--color-text-secondary);
}

.loading-icon {
  width: 20px;
  height: 20px;
  color: var(--color-primary);
}

.animate-spin {
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

.empty-state {
  text-align: center;
  padding: var(--spacing-lg);
  color: var(--color-text-tertiary);
  font-size: var(--text-sm);
}

.files-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.file-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: var(--color-surface-subtle);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.file-icon {
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
}

.file-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 操作按钮 */
.detail-actions {
  display: flex;
  gap: var(--spacing-sm);
  padding: var(--spacing-lg) var(--spacing-xl);
  border-top: 1px solid var(--color-border);
  background: var(--color-surface);
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  border: 1px solid transparent;
}

.btn-icon {
  width: 16px;
  height: 16px;
}

.btn-secondary {
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.btn-secondary:hover {
  background: var(--color-surface-hover);
  color: var(--color-text-primary);
  border-color: var(--color-border-strong);
}

.btn-danger {
  background: var(--strawberry-50);
  color: var(--strawberry);
  border-color: var(--strawberry-200);
}

.btn-danger:hover {
  background: var(--strawberry-100);
  border-color: var(--strawberry);
}

</style>
