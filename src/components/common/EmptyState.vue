<script setup lang="ts">
import { Package, Search, Wifi, AlertCircle } from "lucide-vue-next";

defineProps<{
  title: string;
  description?: string;
  icon?: string;
}>();

function getIcon(icon?: string) {
  switch (icon) {
    case "search":
      return Search;
    case "wifi":
      return Wifi;
    case "alert":
      return AlertCircle;
    default:
      return Package;
  }
}
</script>

<template>
  <div class="empty-state">
    <div class="empty-icon">
      <component :is="getIcon(icon)" class="empty-icon-inner" :stroke-width="1.2" />
    </div>
    <h3 class="empty-title">{{ title }}</h3>
    <p v-if="description" class="empty-description">{{ description }}</p>
    <div class="empty-actions">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  padding: var(--spacing-xl);
  text-align: center;
}

.empty-icon {
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-border-subtle), var(--color-border));
  border-radius: var(--radius-xl);
  margin-bottom: var(--spacing-md);
}

.empty-icon-inner {
  width: 32px;
  height: 32px;
  color: var(--color-text-tertiary);
}

.empty-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-primary);
  margin: 0 0 4px 0;
}

.empty-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  max-width: 320px;
  line-height: 1.5;
  margin: 0;
}

.empty-actions {
  margin-top: var(--spacing-md);
}
</style>
