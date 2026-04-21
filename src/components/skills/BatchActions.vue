<script setup lang="ts">
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import { useSkillsStore } from "@/stores/skills";
import ConfirmDialog from "@/components/common/ConfirmDialog.vue";
import BatchEnableDialog from "./BatchEnableDialog.vue";
import { RefreshCw, Trash2, X, Link, Link2Off } from "lucide-vue-next";

const { t } = useI18n();
const skillsStore = useSkillsStore();

const showDeleteDialog = ref(false);
const showEnableDialog = ref(false);
const showDisableDialog = ref(false);
const errorMessage = ref<string | null>(null);

const selectedSkillNames = computed(() => {
  return Array.from(skillsStore.selectedSkills);
});

async function handleBatchSync() {
  await skillsStore.batchSync();
}

function handleBatchDelete() {
  showDeleteDialog.value = true;
}

async function confirmBatchDelete() {
  showDeleteDialog.value = false;
  await skillsStore.batchDelete();
}

function handleClearSelection() {
  skillsStore.clearSelection();
}

function handleBatchEnable() {
  errorMessage.value = null;
  showEnableDialog.value = true;
}

function handleBatchDisable() {
  errorMessage.value = null;
  showDisableDialog.value = true;
}

async function confirmBatchEnable(toolIds: string[]) {
  showEnableDialog.value = false;
  if (toolIds.length === 0) return;

  const error = await skillsStore.batchEnableToTools(selectedSkillNames.value, toolIds);
  if (error) {
    errorMessage.value = error;
  } else {
    // 操作成功，清空选择
    skillsStore.clearSelection();
  }
}

async function confirmBatchDisable(toolIds: string[]) {
  showDisableDialog.value = false;
  if (toolIds.length === 0) return;

  const error = await skillsStore.batchDisableFromTools(selectedSkillNames.value, toolIds);
  if (error) {
    errorMessage.value = error;
  } else {
    // 操作成功，清空选择
    skillsStore.clearSelection();
  }
}
</script>

<template>
  <div class="mb-4 flex flex-col gap-3 rounded-lg border border-[var(--color-border)] bg-[var(--color-surface)] p-3">
    <div class="flex items-center justify-between">
      <span class="text-sm text-[var(--color-text-secondary)]">
        {{ skillsStore.selectedSkills.size }} {{ t("skills.title") }} {{ t("skills.selected") || "selected" }}
      </span>
      <div class="flex items-center gap-2">
        <button
          @click="handleBatchSync"
          :disabled="skillsStore.selectedSkills.size === 0"
          class="btn btn-secondary"
        >
          <RefreshCw class="h-4 w-4" :stroke-width="1.5" />
          {{ t("skills.refresh") }}
        </button>
        <button
          @click="handleBatchDelete"
          :disabled="skillsStore.selectedSkills.size === 0"
          class="btn btn-danger"
        >
          <Trash2 class="h-4 w-4" :stroke-width="1.5" />
          {{ t("skills.delete") }}
        </button>
        <button
          @click="handleClearSelection"
          class="btn btn-ghost"
        >
          <X class="h-4 w-4" :stroke-width="1.5" />
          {{ t("common.cancel") }}
        </button>
      </div>
    </div>

    <!-- 软链接批量操作 -->
    <div class="flex items-center gap-2 border-t border-[var(--color-border-subtle)] pt-3">
      <span class="text-xs text-[var(--color-text-tertiary)]">{{ t("skills.symlinkActions") }}:</span>
      <button
        @click="handleBatchEnable"
        :disabled="skillsStore.selectedSkills.size === 0"
        class="btn btn-primary btn-sm"
      >
        <Link class="h-4 w-4" :stroke-width="1.5" />
        {{ t("skills.batchEnableToTools") }}
      </button>
      <button
        @click="handleBatchDisable"
        :disabled="skillsStore.selectedSkills.size === 0"
        class="btn btn-secondary btn-sm"
      >
        <Link2Off class="h-4 w-4" :stroke-width="1.5" />
        {{ t("skills.batchDisableFromTools") }}
      </button>
    </div>

    <!-- 错误提示 -->
    <div v-if="errorMessage" class="error-alert">
      <span class="error-text">{{ errorMessage }}</span>
      <button @click="errorMessage = null" class="error-close">
        <X class="h-4 w-4" :stroke-width="1.5" />
      </button>
    </div>

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :show="showDeleteDialog"
      :title="t('skills.confirmDelete')"
      :message="t('skills.confirmDeleteDesc')"
      type="danger"
      @confirm="confirmBatchDelete"
      @cancel="showDeleteDialog = false"
    />

    <!-- 批量启用对话框 -->
    <BatchEnableDialog
      :show="showEnableDialog"
      :skill-names="selectedSkillNames"
      mode="enable"
      @confirm="confirmBatchEnable"
      @cancel="showEnableDialog = false"
    />

    <!-- 批量停用对话框 -->
    <BatchEnableDialog
      :show="showDisableDialog"
      :skill-names="selectedSkillNames"
      mode="disable"
      @confirm="confirmBatchDisable"
      @cancel="showDisableDialog = false"
    />
  </div>
</template>

<style scoped>
.error-alert {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--strawberry-50);
  border: 2px solid var(--strawberry-200);
  border-radius: var(--radius-md);
  color: var(--strawberry);
}

.error-text {
  font-size: var(--text-sm);
  flex: 1;
}

.error-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: none;
  border: none;
  color: var(--strawberry);
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: background var(--transition-fast) var(--ease-out);
}

.error-close:hover {
  background: var(--strawberry-100);
}

.btn-sm {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--text-sm);
}
</style>
