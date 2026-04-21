<script setup lang="ts">
import { type Component } from "vue";

defineProps<{
  title: string;
  icon?: Component;
  statistics?: Array<{
    label: string;
    value: string | number;
    highlight?: boolean;
  }>;
}>();

const iconSize = 52;
const iconInnerSize = 26;
</script>

<template>
  <div class="page-header-container">
    <div class="page-header-content">
      <div v-if="icon" class="page-header-icon">
        <component :is="icon" :style="{ width: iconInnerSize + 'px', height: iconInnerSize + 'px' }" :stroke-width="1.5" />
      </div>
      <div class="page-header-text">
        <h1 class="page-header-title">{{ title }}</h1>
        <div v-if="statistics && statistics.length > 0" class="page-header-stats">
          <template v-for="(stat, index) in statistics" :key="index">
            <span class="stat-item">
              {{ stat.label }}
              <span :class="['stat-value', { 'stat-highlight': stat.highlight }]">{{ stat.value }}</span>
            </span>
            <span v-if="index < statistics.length - 1" class="stat-separator">·</span>
          </template>
        </div>
      </div>
    </div>
    <div class="page-header-actions">
      <slot name="actions" />
    </div>
  </div>
</template>

<style scoped>
.page-header-container {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: var(--spacing-xl);
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-card);
  margin-bottom: var(--spacing-xl);
  transition: all var(--transition-fast) var(--ease-out);
  border: 2px solid var(--color-border-subtle);
}

.page-header-container:hover {
  box-shadow: var(--shadow-card-hover);
  transform: translateY(-2px);
}

.page-header-content {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-lg);
}

.page-header-icon {
  width: v-bind('iconSize + "px"');
  height: v-bind('iconSize + "px"');
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  border-radius: var(--radius-lg);
  color: white;
  flex-shrink: 0;
  box-shadow: var(--shadow-clay);
  transition: all var(--transition-fast) var(--ease-out);
}

.page-header-icon:hover {
  transform: scale(1.05) rotate(-5deg);
  box-shadow: var(--shadow-card-hover);
}

.page-header-text {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.page-header-title {
  font-family: var(--font-display);
  font-size: var(--text-2xl);
  font-weight: 400;
  color: var(--color-text-primary);
  letter-spacing: -0.02em;
  margin: 0;
}

.page-header-stats {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--spacing-sm);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.stat-value {
  font-weight: 600;
  color: var(--color-text-primary);
  background: var(--color-surface-subtle);
  padding: 2px 8px;
  border-radius: var(--radius-full);
  min-width: 24px;
  text-align: center;
}

.stat-value.stat-highlight {
  background: linear-gradient(135deg, var(--peach-100), var(--peach-50));
  color: var(--peach-600);
}

.stat-separator {
  color: var(--color-text-tertiary);
  opacity: 0.5;
}

.page-header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  flex-wrap: wrap;
}

.page-header-actions :deep(.btn) {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-lg);
  border-radius: var(--radius-md);
  font-size: var(--text-base);
  font-weight: 500;
  transition: all var(--transition-fast) var(--ease-out);
  border: 2px solid transparent;
  cursor: pointer;
  font-family: var(--font-body);
}

.page-header-actions :deep(.btn-primary) {
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  color: white;
  box-shadow: var(--shadow-clay);
}

.page-header-actions :deep(.btn-primary:hover) {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

.page-header-actions :deep(.btn-primary:active) {
  transform: scale(0.96);
  box-shadow: var(--shadow-pressed);
}

.page-header-actions :deep(.btn-secondary) {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  border-color: var(--color-border);
}

.page-header-actions :deep(.btn-secondary:hover) {
  background: var(--peach-50);
  border-color: var(--peach-200);
  color: var(--peach-500);
}

.page-header-actions :deep(.btn-ghost) {
  background: transparent;
  color: var(--color-text-secondary);
}

.page-header-actions :deep(.btn-ghost:hover) {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
}
</style>
