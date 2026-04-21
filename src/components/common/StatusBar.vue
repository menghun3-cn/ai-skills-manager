<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useToolsStore } from "@/stores/tools";
import { useSettingsStore } from "@/stores/settings";
import { Circle, CheckCircle2 } from "lucide-vue-next";

const { t } = useI18n();
const toolsStore = useToolsStore();
const settingsStore = useSettingsStore();

const enabledCount = computed(() => toolsStore.enabledCount);
const storagePath = computed(() => settingsStore.configPath || ".ai-skills-manager");
</script>

<template>
  <footer class="status-bar">
    <div class="status-left">
      <span class="status-item status-success">
        <CheckCircle2 class="status-icon" :stroke-width="2" />
        {{ t("status.synced") }}
      </span>
      <span class="status-separator">·</span>
      <span class="status-item">{{ t("status.enabledTools", { count: enabledCount }) }}</span>
    </div>
    <div class="status-right">
      <span class="status-item status-path">
        <span class="path-label">{{ t("status.storagePath") }}:</span>
        <span class="path-value">{{ storagePath }}</span>
      </span>
    </div>
  </footer>
</template>

<style scoped>
.status-bar {
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-lg);
  background: linear-gradient(180deg, var(--color-surface-subtle) 0%, var(--color-surface) 100%);
  border-top: 2px solid var(--color-border-subtle);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  flex-shrink: 0;
  font-family: var(--font-body);
}

.status-left,
.status-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast) var(--ease-out);
}

.status-item:hover {
  background: var(--color-surface-subtle);
}

.status-success {
  color: var(--mint);
  font-weight: 500;
}

.status-icon {
  width: 14px;
  height: 14px;
}

.status-separator {
  color: var(--color-border-strong);
  font-weight: 600;
}

.status-path {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.path-label {
  color: var(--color-text-tertiary);
}

.path-value {
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  background: var(--color-surface-subtle);
  padding: 2px 6px;
  border-radius: var(--radius-xs);
  border: 1px solid var(--color-border);
}
</style>
