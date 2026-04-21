<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { AlertTriangle, X } from "lucide-vue-next";

const props = defineProps<{
  show: boolean;
  title: string;
  message: string;
  type?: "warning" | "danger" | "info";
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();

const { t } = useI18n();

function handleConfirm() {
  emit("confirm");
}

function handleCancel() {
  emit("cancel");
}

function handleOverlayClick() {
  emit("cancel");
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="show" class="dialog-overlay" @click="handleOverlayClick">
        <div class="dialog-panel" @click.stop>
          <!-- 头部 -->
          <div class="dialog-header" :class="type">
            <div class="header-icon">
              <AlertTriangle class="icon-svg" :stroke-width="2" />
            </div>
            <h3 class="dialog-title">{{ title }}</h3>
            <button class="close-btn" @click="handleCancel">
              <X class="close-icon" :stroke-width="2" />
            </button>
          </div>

          <!-- 内容 -->
          <div class="dialog-content">
            <p class="dialog-message">{{ message }}</p>
          </div>

          <!-- 按钮 -->
          <div class="dialog-actions">
            <button class="btn btn-secondary" @click="handleCancel">
              {{ t("common.cancel") }}
            </button>
            <button class="btn btn-danger" @click="handleConfirm">
              {{ t("common.confirm") }}
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
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-lg);
}

.dialog-panel {
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-card-hover);
  border: 2px solid var(--color-border);
  width: 100%;
  max-width: 420px;
  overflow: hidden;
  animation: dialogPop 0.3s var(--ease-elastic);
}

@keyframes dialogPop {
  0% {
    opacity: 0;
    transform: scale(0.9) translateY(20px);
  }
  100% {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

/* 头部 */
.dialog-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--color-border-subtle);
  background: linear-gradient(135deg, var(--strawberry-50), var(--strawberry-100));
}

.dialog-header.danger {
  background: linear-gradient(135deg, var(--strawberry-50), var(--strawberry-100));
}

.dialog-header.warning {
  background: linear-gradient(135deg, var(--honey-50), var(--honey-100));
}

.dialog-header.info {
  background: linear-gradient(135deg, var(--lavender-50), var(--lavender-100));
}

.header-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--strawberry-200);
  border-radius: var(--radius-lg);
  color: var(--strawberry);
  flex-shrink: 0;
}

.icon-svg {
  width: 22px;
  height: 22px;
}

.dialog-title {
  flex: 1;
  font-size: var(--text-lg);
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
  font-family: var(--font-heading);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
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
}

.close-icon {
  width: 16px;
  height: 16px;
}

/* 内容 */
.dialog-content {
  padding: var(--spacing-xl);
}

.dialog-message {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  line-height: 1.6;
  margin: 0;
}

/* 按钮 */
.dialog-actions {
  display: flex;
  gap: var(--spacing-md);
  padding: var(--spacing-lg) var(--spacing-xl);
  border-top: 1px solid var(--color-border-subtle);
  background: var(--color-surface-subtle);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  flex: 1;
  padding: var(--spacing-sm) var(--spacing-lg);
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
}

.btn-danger {
  background: linear-gradient(135deg, var(--strawberry-300), var(--strawberry-400));
  color: white;
  box-shadow: var(--shadow-clay);
}

.btn-danger:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

/* 过渡动画 */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: all 0.2s var(--ease-out);
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

.dialog-fade-enter-from .dialog-panel,
.dialog-fade-leave-to .dialog-panel {
  transform: scale(0.95) translateY(10px);
}
</style>
