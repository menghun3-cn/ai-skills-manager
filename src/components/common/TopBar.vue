<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import { useI18n } from "vue-i18n";
import { ChevronRight, User, Home } from "lucide-vue-next";

const { t } = useI18n();
const route = useRoute();

const breadcrumbs = computed(() => {
  const paths = route.path.split("/").filter(Boolean);
  const crumbs = [{ label: t("nav.home"), path: "/", icon: Home }];

  let currentPath = "";
  for (const path of paths) {
    currentPath += `/${path}`;
    const labelKey = `nav.${path}`;

    const defaultLabel = path.charAt(0).toUpperCase() + path.slice(1);
    crumbs.push({
      label: t(labelKey) !== labelKey ? t(labelKey) : defaultLabel,
      path: currentPath,
    });
  }

  return crumbs;
});

const isLast = (index: number) => index === breadcrumbs.value.length - 1;
</script>

<template>
  <header class="header">
    <nav class="breadcrumb" aria-label="Breadcrumb">
      <template v-for="(crumb, index) in breadcrumbs" :key="crumb.path">
        <ChevronRight
          v-if="index > 0"
          class="breadcrumb-separator"
          :stroke-width="1.5"
        />
        <router-link
          v-if="!isLast(index)"
          :to="crumb.path"
          class="breadcrumb-link"
        >
          <component v-if="crumb.icon" :is="crumb.icon" class="breadcrumb-icon" :stroke-width="1.5" />
          <span>{{ crumb.label }}</span>
        </router-link>
        <span
          v-else
          class="breadcrumb-current"
        >
          <component v-if="crumb.icon" :is="crumb.icon" class="breadcrumb-icon" :stroke-width="1.5" />
          <span>{{ crumb.label }}</span>
        </span>
      </template>
    </nav>

    <div class="user-info">
      <div class="user-avatar">
        <User class="h-4 w-4" :stroke-width="1.5" />
      </div>
      <span class="user-name">{{ t("user.admin") }}</span>
    </div>
  </header>
</template>

<style scoped>
.header {
  height: var(--header-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-xl);
  background: linear-gradient(180deg, var(--color-surface) 0%, var(--color-surface-subtle) 100%);
  border-bottom: 2px solid var(--color-border-subtle);
  box-shadow: var(--shadow-soft);
  position: sticky;
  top: 0;
  z-index: 50;
  font-family: var(--font-body);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--text-sm);
}

.breadcrumb-separator {
  width: 14px;
  height: 14px;
  color: var(--color-text-tertiary);
  opacity: 0.6;
}

.breadcrumb-link {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  color: var(--color-text-secondary);
  text-decoration: none;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast) var(--ease-out);
}

.breadcrumb-link:hover {
  color: var(--peach-500);
  background: var(--peach-50);
}

.breadcrumb-current {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-weight: 600;
  color: var(--color-text-primary);
  padding: var(--spacing-xs) var(--spacing-sm);
  background: linear-gradient(135deg, var(--peach-100), var(--peach-50));
  border-radius: var(--radius-md);
  border: 1px solid var(--peach-200);
}

.breadcrumb-icon {
  width: 14px;
  height: 14px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--color-surface);
  border-radius: var(--radius-full);
  border: 2px solid var(--color-border);
  transition: all var(--transition-fast) var(--ease-out);
}

.user-info:hover {
  border-color: var(--peach-200);
  box-shadow: var(--shadow-soft);
}

.user-avatar {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  border-radius: var(--radius-full);
  color: white;
  box-shadow: var(--shadow-soft);
}

.user-name {
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-primary);
}
</style>
