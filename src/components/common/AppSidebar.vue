<script setup lang="ts">
import { ref, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import {
  Package,
  Wrench,
  Store,
  Settings,
  Sparkles,
  ChevronLeft,
  ChevronRight,
} from "lucide-vue-next";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();

const isCollapsed = ref(false);

const navItems = computed(() => [
  { path: "/skills", label: t("nav.skills"), icon: Package },
  { path: "/tools", label: t("nav.tools"), icon: Wrench },
  { path: "/market", label: t("nav.market"), icon: Store },
  { path: "/settings", label: t("nav.settings"), icon: Settings },
]);

function isActive(path: string) {
  return route.path === path;
}

function navigate(path: string) {
  router.push(path);
}

function toggleCollapse() {
  isCollapsed.value = !isCollapsed.value;
}
</script>

<template>
  <aside
    class="sidebar"
    :class="{ collapsed: isCollapsed }"
  >
    <header class="sidebar-header">
      <div
        class="sidebar-logo"
        :class="{ 'justify-center': isCollapsed }"
      >
        <div class="logo-icon">
          <Sparkles class="h-5 w-5" :stroke-width="1.5" />
        </div>
        <div v-if="!isCollapsed" class="logo-text">
          <span class="logo-title">AI Skills</span>
          <span class="logo-subtitle">Manager</span>
        </div>
      </div>

      <button
        v-if="!isCollapsed"
        @click="toggleCollapse"
        class="collapse-btn"
        :title="t('sidebar.collapse')"
      >
        <ChevronLeft class="h-4 w-4" :stroke-width="1.5" />
      </button>
      <button
        v-else
        @click="toggleCollapse"
        class="collapse-btn center"
        :title="t('sidebar.expand')"
      >
        <ChevronRight class="h-4 w-4" :stroke-width="1.5" />
      </button>
    </header>

    <nav class="sidebar-nav">
      <div class="nav-items">
        <button
          v-for="(item, index) in navItems"
          :key="item.path"
          @click="navigate(item.path)"
          class="nav-item"
          :class="{ active: isActive(item.path) }"
          :title="isCollapsed ? item.label : undefined"
          :style="{ animationDelay: `${index * 60}ms` }"
        >
          <div v-if="isActive(item.path)" class="active-indicator" />
          <component
            :is="item.icon"
            class="nav-icon"
            :stroke-width="1.5"
          />
          <span v-if="!isCollapsed" class="nav-label">{{ item.label }}</span>
        </button>
      </div>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width-expanded);
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  border-right: 2px solid var(--color-border-subtle);
  transition: all var(--transition-slow) var(--ease-spring);
  flex-shrink: 0;
  overflow: hidden;
  box-shadow: var(--shadow-soft);
}

.sidebar.collapsed {
  width: var(--sidebar-width-collapsed);
}

.sidebar-header {
  height: var(--header-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-lg);
  border-bottom: 2px solid var(--color-border-subtle);
  flex-shrink: 0;
  background: linear-gradient(180deg, var(--color-surface) 0%, var(--color-surface-subtle) 100%);
}

.sidebar-logo {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  overflow: hidden;
}

.logo-icon {
  width: 40px;
  height: 40px;
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

.logo-icon:hover {
  transform: scale(1.05) rotate(-5deg);
  box-shadow: var(--shadow-card-hover);
}

.logo-text {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  animation: fadeIn 0.3s var(--ease-out);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateX(-10px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.logo-title {
  font-family: var(--font-display);
  font-size: var(--text-lg);
  font-weight: 400;
  color: var(--color-text-primary);
  white-space: nowrap;
  letter-spacing: -0.02em;
}

.logo-subtitle {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  white-space: nowrap;
  font-weight: 500;
}

.collapse-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  color: var(--color-text-tertiary);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  flex-shrink: 0;
}

.collapse-btn:hover {
  background: var(--peach-100);
  color: var(--peach-500);
  border-color: var(--peach-200);
  transform: scale(1.1);
}

.collapse-btn:active {
  transform: scale(0.95);
}

.collapse-btn.center {
  margin: 0 auto;
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: var(--spacing-xl) 0;
}

.nav-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  padding: 0 var(--spacing-md);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  height: 44px;
  padding: 0 var(--spacing-md);
  border-radius: var(--radius-md);
  background: transparent;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  position: relative;
  color: var(--color-text-secondary);
  text-align: left;
  width: 100%;
  animation: slideIn 0.4s var(--ease-out) backwards;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.collapsed .nav-item {
  justify-content: center;
  padding: 0;
  width: 44px;
  height: 44px;
  margin: 0 auto;
}

.nav-item:hover {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  border-color: var(--color-border);
  transform: translateX(4px);
}

.collapsed .nav-item:hover {
  transform: scale(1.1);
}

.nav-item.active {
  background: linear-gradient(135deg, var(--peach-100), var(--peach-50));
  color: var(--peach-500);
  border-color: var(--peach-200);
  box-shadow: var(--shadow-soft);
}

.nav-item.active:hover {
  background: linear-gradient(135deg, var(--peach-200), var(--peach-100));
}

.active-indicator {
  position: absolute;
  left: -2px;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 20px;
  background: linear-gradient(180deg, var(--peach-300), var(--peach-400));
  border-radius: 0 var(--radius-xs) var(--radius-xs) 0;
}

.nav-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  transition: transform var(--transition-fast) var(--ease-out);
}

.nav-item:hover .nav-icon {
  transform: scale(1.15) rotate(-5deg);
}

.nav-item.active .nav-icon {
  transform: scale(1.1);
}

.nav-label {
  font-size: var(--text-base);
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: var(--font-body);
}

.nav-item.active .nav-label {
  font-weight: 600;
  color: var(--peach-600);
}
</style>
