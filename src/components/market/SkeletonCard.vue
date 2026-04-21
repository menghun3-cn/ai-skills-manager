<script setup lang="ts">
</script>

<template>
  <div class="skeleton-card">
    <div class="skeleton-glow"></div>
    <div class="skeleton-content">
      <!-- 头部区域 -->
      <div class="skeleton-header">
        <div class="skeleton-main">
          <div class="skeleton-icon animate-pulse"></div>
          <div class="skeleton-info">
            <div class="skeleton-title animate-pulse"></div>
            <div class="skeleton-desc animate-pulse"></div>
          </div>
        </div>
        <div class="skeleton-action animate-pulse"></div>
      </div>

      <!-- 元信息标签 -->
      <div class="skeleton-footer">
        <div class="skeleton-tag animate-pulse"></div>
        <div class="skeleton-tag animate-pulse"></div>
        <div class="skeleton-tag animate-pulse"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.skeleton-card {
  position: relative;
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  overflow: hidden;
  border: 2px solid var(--color-border-subtle);
}

.skeleton-glow {
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(255, 155, 173, 0.05), rgba(184, 169, 232, 0.05));
  opacity: 0;
  pointer-events: none;
}

.skeleton-content {
  position: relative;
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

/* 头部区域 */
.skeleton-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--spacing-md);
}

.skeleton-main {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-md);
  flex: 1;
  min-width: 0;
}

.skeleton-icon {
  width: 52px;
  height: 52px;
  border-radius: var(--radius-lg);
  background: var(--color-surface-subtle);
  flex-shrink: 0;
}

.skeleton-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.skeleton-title {
  height: 22px;
  width: 60%;
  border-radius: var(--radius-sm);
  background: var(--color-surface-subtle);
}

.skeleton-desc {
  height: 16px;
  width: 80%;
  border-radius: var(--radius-sm);
  background: var(--color-surface-subtle);
}

.skeleton-action {
  width: 80px;
  height: 36px;
  border-radius: var(--radius-md);
  background: var(--color-surface-subtle);
  flex-shrink: 0;
}

/* 元信息标签 */
.skeleton-footer {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-xs);
  padding-top: var(--spacing-sm);
  border-top: 1px solid var(--color-border-subtle);
}

.skeleton-tag {
  height: 24px;
  width: 60px;
  border-radius: var(--radius-full);
  background: var(--color-surface-subtle);
}

.skeleton-tag:nth-child(2) {
  width: 80px;
}

.skeleton-tag:nth-child(3) {
  width: 50px;
}

/* 脉冲动画 */
.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
