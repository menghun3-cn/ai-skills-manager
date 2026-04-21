<script setup lang="ts">
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import type { Skill } from "@/types";
import { useSkillsStore } from "@/stores/skills";
import ConfirmDialog from "@/components/common/ConfirmDialog.vue";
import { Pencil, Trash2, CheckSquare, Cpu, Monitor, Globe, Folder, Settings2 } from "lucide-vue-next";

const props = defineProps<{
  skill: Skill;
  batchMode?: boolean;
}>();

const emit = defineEmits<{
  edit: [skillName: string];
  click: [];
  manage: [skill: Skill];
}>();

const { t } = useI18n();
const skillsStore = useSkillsStore();

const showDeleteDialog = ref(false);

const isSelected = computed(() => skillsStore.selectedSkills.has(props.skill.name));

// 计算启用的IDE数量
const enabledToolsCount = computed(() => props.skill.enabledTools?.length || 0);

// 获取启用的IDE名称列表（用于tooltip）
const enabledToolsNames = computed(() => {
  if (!props.skill.enabledTools || props.skill.enabledTools.length === 0) {
    return t("skills.noEnabledTools");
  }
  return props.skill.enabledTools.join(", ");
});

function toggleSelect() {
  if (props.batchMode) {
    skillsStore.toggleSkillSelection(props.skill.name);
  } else {
    emit("click");
  }
}

function handleEdit() {
  emit("edit", props.skill.name);
}

function handleDelete() {
  showDeleteDialog.value = true;
}

function handleManage() {
  emit("manage", props.skill);
}

async function confirmDelete() {
  showDeleteDialog.value = false;
  await skillsStore.deleteSkill(props.skill.name);
}
</script>

<template>
  <div
    class="card"
    :class="{ 'card-selected': isSelected }"
    @click="toggleSelect"
  >
    <div v-if="batchMode" class="select-indicator">
      <CheckSquare
        class="select-icon"
        :class="isSelected ? 'selected' : ''"
        :stroke-width="1.5"
      />
    </div>

    <!-- 管理技能按钮 - 仅在非批量模式显示 -->
    <div v-if="!batchMode" class="manage-indicator" @click.stop="handleManage">
      <Settings2
        class="manage-icon"
        :stroke-width="1.5"
      />
    </div>

    <div class="card-header">
      <div class="skill-icon">
        <Cpu class="h-5 w-5" :stroke-width="1.5" />
      </div>
      <div class="skill-info">
        <div class="skill-title-row">
          <h3 class="skill-title">{{ skill.name }}</h3>
          <span
            v-if="!skill.enabled"
            class="badge badge-warning"
          >
            {{ t("tools.disabled") }}
          </span>
        </div>
        <div class="skill-type">
          <Globe v-if="skill.bindings.global" class="type-icon" :stroke-width="1.5" />
          <Folder v-else class="type-icon" :stroke-width="1.5" />
          <span>{{ skill.bindings.global ? t("skills.global") : t("skills.projectBound") }}</span>
        </div>
      </div>
    </div>

    <p class="skill-description">
      {{ skill.description || t("skills.noDescription") }}
    </p>

    <!-- IDE 使用信息 -->
    <div class="ide-info" :title="enabledToolsNames">
      <div class="ide-count">
        <Monitor class="ide-icon" :stroke-width="1.5" />
        <span class="ide-count-text">
          {{ enabledToolsCount > 0 ? t("skills.enabledInTools", { count: enabledToolsCount }) : t("skills.notEnabledInTools") }}
        </span>
      </div>
      <div v-if="enabledToolsCount > 0" class="ide-names">
        {{ enabledToolsNames }}
      </div>
    </div>

    <div class="card-actions" @click.stop>
      <button
        @click="handleEdit"
        class="action-btn action-edit"
      >
        <Pencil class="action-icon" :stroke-width="1.5" />
        {{ t("skills.edit") }}
      </button>
      <button
        @click="handleDelete"
        class="action-btn action-delete"
      >
        <Trash2 class="action-icon" :stroke-width="1.5" />
        {{ t("skills.delete") }}
      </button>
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
.card {
  position: relative;
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  padding: var(--spacing-md);
  transition: all var(--transition-fast) var(--ease-out);
  cursor: pointer;
  height: fit-content;
}

.card:hover {
  border-color: var(--color-border-strong);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.card-selected {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-subtle);
}

.select-indicator {
  position: absolute;
  right: 12px;
  top: 12px;
}

.select-icon {
  width: 20px;
  height: 20px;
  color: var(--color-border);
  transition: color var(--transition-fast) var(--ease-out);
}

.select-icon.selected {
  color: var(--color-primary);
}

/* 管理技能按钮 */
.manage-indicator {
  position: absolute;
  right: 12px;
  top: 12px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  z-index: 10;
}

.manage-indicator:hover {
  background: var(--color-primary-subtle);
  border-color: var(--color-primary);
  transform: scale(1.05);
}

.manage-icon {
  width: 16px;
  height: 16px;
  color: var(--color-text-secondary);
  transition: color var(--transition-fast) var(--ease-out);
}

.manage-indicator:hover .manage-icon {
  color: var(--color-primary);
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.skill-icon {
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

.skill-info {
  flex: 1;
  min-width: 0;
}

.skill-title-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.skill-title {
  font-size: var(--text-base);
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  flex-shrink: 0;
}

.badge-warning {
  background: var(--strawberry-100);
  color: var(--strawberry);
}

.skill-type {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.type-icon {
  width: 14px;
  height: 14px;
}

.skill-description {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0 0 12px 0;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* IDE 信息样式 */
.ide-info {
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-md);
  margin-bottom: 12px;
}

.ide-count {
  display: flex;
  align-items: center;
  gap: 8px;
}

.ide-icon {
  width: 16px;
  height: 16px;
  color: var(--color-primary);
}

.ide-count-text {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text);
}

.ide-names {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  margin-top: 4px;
  padding-left: 24px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-actions {
  display: flex;
  gap: 8px;
  margin-top: auto;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  flex: 1;
  padding: 8px 12px;
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  border: 1px solid transparent;
}

.action-icon {
  width: 14px;
  height: 14px;
}

.action-edit {
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.action-edit:hover {
  background: var(--color-primary-subtle);
  color: var(--color-primary);
  border-color: var(--color-primary);
}

.action-delete {
  background: var(--color-surface-subtle);
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}

.action-delete:hover {
  background: var(--strawberry-50);
  color: var(--strawberry);
  border-color: var(--strawberry);
}
</style>
