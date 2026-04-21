<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useToolsStore } from "@/stores/tools";
import { X, Check, AlertCircle, Wrench } from "lucide-vue-next";

const props = defineProps<{
  show: boolean;
  skillNames: string[];
  mode: "enable" | "disable";
}>();

const emit = defineEmits<{
  confirm: [toolIds: string[]];
  cancel: [];
}>();

const { t } = useI18n();
const toolsStore = useToolsStore();

const selectedTools = ref<Set<string>>(new Set());
const errorMessage = ref<string | null>(null);

const dialogTitle = computed(() => {
  if (props.mode === "enable") {
    return t("skills.batchEnableToTools");
  }
  return t("skills.batchDisableFromTools");
});

const dialogDescription = computed(() => {
  const count = props.skillNames.length;
  if (props.mode === "enable") {
    return t("skills.batchEnableDesc", { count });
  }
  return t("skills.batchDisableDesc", { count });
});

const confirmButtonText = computed(() => {
  if (props.mode === "enable") {
    return t("skills.enableToTools");
  }
  return t("skills.disableFromTools");
});

const enabledTools = computed(() => {
  return toolsStore.tools.filter((tool) => tool.enabled);
});

const canConfirm = computed(() => {
  return selectedTools.value.size > 0;
});

onMounted(() => {
  toolsStore.detectTools();
  selectedTools.value.clear();
  errorMessage.value = null;
});

function toggleTool(toolId: string) {
  if (selectedTools.value.has(toolId)) {
    selectedTools.value.delete(toolId);
  } else {
    selectedTools.value.add(toolId);
  }
}

function selectAll() {
  enabledTools.value.forEach((tool) => {
    selectedTools.value.add(tool.id);
  });
}

function clearSelection() {
  selectedTools.value.clear();
}

function handleConfirm() {
  if (selectedTools.value.size === 0) return;
  emit("confirm", Array.from(selectedTools.value));
}

function handleCancel() {
  emit("cancel");
}
</script>

<template>
  <div v-if="show" class="dialog-overlay" @click.self="handleCancel">
    <div class="dialog">
      <div class="dialog-header">
        <h3 class="dialog-title">{{ dialogTitle }}</h3>
        <button @click="handleCancel" class="dialog-close" :aria-label="t('common.close')">
          <X class="h-5 w-5" :stroke-width="1.5" />
        </button>
      </div>

      <p class="dialog-desc">{{ dialogDescription }}</p>

      <!-- 错误提示 -->
      <div v-if="errorMessage" class="error-alert">
        <AlertCircle class="h-5 w-5 flex-shrink-0" :stroke-width="1.5" />
        <span class="error-text">{{ errorMessage }}</span>
      </div>

      <!-- 工具列表 -->
      <div class="dialog-content">
        <div v-if="toolsStore.loading" class="loading-state">
          <div class="loading-spinner">
            <div class="spinner-dot"></div>
            <div class="spinner-dot"></div>
            <div class="spinner-dot"></div>
          </div>
          <span class="loading-text">{{ t("common.loading") }}</span>
        </div>

        <div v-else-if="enabledTools.length === 0" class="empty-state">
          <Wrench class="h-12 w-12 text-[var(--color-text-tertiary)]" :stroke-width="1.5" />
          <p class="empty-text">{{ t("skills.noEnabledTools") }}</p>
          <p class="empty-subtext">{{ t("skills.enableToolsFirst") }}</p>
        </div>

        <div v-else class="tools-section">
          <div class="tools-header">
            <span class="tools-count">{{ t("tools.enabled") }} ({{ enabledTools.length }})</span>
            <div class="tools-actions">
              <button @click="selectAll" class="text-btn">{{ t("common.selectAll") }}</button>
              <button @click="clearSelection" class="text-btn">{{ t("common.clear") }}</button>
            </div>
          </div>

          <div class="tools-list">
            <div
              v-for="tool in enabledTools"
              :key="tool.id"
              class="tool-item"
              :class="{ selected: selectedTools.has(tool.id) }"
              @click="toggleTool(tool.id)"
            >
              <div class="tool-checkbox">
                <Check v-if="selectedTools.has(tool.id)" class="h-4 w-4" :stroke-width="2" />
              </div>
              <div class="tool-info">
                <span class="tool-name">{{ tool.name }}</span>
                <span class="tool-path">{{ tool.skillsPath }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="dialog-footer">
        <span class="selection-info">
          {{ t("skills.selectedTools", { count: selectedTools.size }) }}
        </span>
        <div class="dialog-actions">
          <button @click="handleCancel" class="btn btn-secondary">
            {{ t("common.cancel") }}
          </button>
          <button
            @click="handleConfirm"
            :disabled="!canConfirm"
            class="btn"
            :class="mode === 'enable' ? 'btn-primary' : 'btn-danger'"
          >
            {{ confirmButtonText }}
          </button>
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
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  padding: var(--spacing-lg);
}

.dialog {
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  border: 2px solid var(--color-border);
  width: 100%;
  max-width: 520px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg) var(--spacing-xl);
  border-bottom: 2px solid var(--color-border);
}

.dialog-title {
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text);
  margin: 0;
}

.dialog-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: var(--color-surface-subtle);
  border: none;
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.dialog-close:hover {
  background: var(--strawberry-100);
  color: var(--strawberry);
}

.dialog-desc {
  padding: var(--spacing-md) var(--spacing-xl);
  margin: 0;
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  border-bottom: 1px solid var(--color-border-subtle);
}

.error-alert {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin: var(--spacing-md) var(--spacing-xl);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--strawberry-50);
  border: 2px solid var(--strawberry-200);
  border-radius: var(--radius-md);
  color: var(--strawberry);
}

.error-text {
  font-size: var(--text-sm);
}

.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md) var(--spacing-xl);
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-2xl);
  gap: var(--spacing-md);
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
  0%,
  80%,
  100% {
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
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-2xl);
  gap: var(--spacing-md);
  text-align: center;
}

.empty-text {
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text-secondary);
  margin: 0;
}

.empty-subtext {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  margin: 0;
}

.tools-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.tools-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) 0;
}

.tools-count {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-secondary);
}

.tools-actions {
  display: flex;
  gap: var(--spacing-md);
}

.text-btn {
  font-size: var(--text-sm);
  color: var(--color-primary);
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
  transition: color var(--transition-fast) var(--ease-out);
}

.text-btn:hover {
  color: var(--color-primary-hover);
  text-decoration: underline;
}

.tools-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  max-height: 300px;
  overflow-y: auto;
}

.tool-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md) var(--spacing-lg);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-item:hover {
  border-color: var(--color-border);
  background: var(--color-surface);
}

.tool-item.selected {
  border-color: var(--color-primary);
  background: var(--color-primary-50);
}

.tool-checkbox {
  width: 20px;
  height: 20px;
  border: 2px solid var(--color-border);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  background: var(--color-surface);
  color: var(--color-surface);
  transition: all var(--transition-fast) var(--ease-out);
}

.tool-item.selected .tool-checkbox {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-surface);
}

.tool-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
  flex: 1;
}

.tool-name {
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tool-path {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-lg) var(--spacing-xl);
  border-top: 2px solid var(--color-border);
  gap: var(--spacing-lg);
}

.selection-info {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.dialog-actions {
  display: flex;
  gap: var(--spacing-md);
}
</style>
