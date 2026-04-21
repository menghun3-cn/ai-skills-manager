<script setup lang="ts">
import { ref, watch } from "vue";
import { useRoute } from "vue-router";
import AppSidebar from "@/components/common/AppSidebar.vue";
import TopBar from "@/components/common/TopBar.vue";
import StatusBar from "@/components/common/StatusBar.vue";

const route = useRoute();
const routerViewKey = ref(route.fullPath);

watch(
  () => route.fullPath,
  (newPath) => {
    routerViewKey.value = newPath;
  }
);
</script>

<template>
  <div class="layout">
    <AppSidebar />
    <div class="main-area">
      <TopBar />
      <main class="content-area">
        <router-view v-if="routerViewKey" :key="routerViewKey" v-slot="{ Component, route: currentRoute }">
          <transition name="page-transition" mode="out-in">
            <component :is="Component" :key="currentRoute.fullPath" />
          </transition>
        </router-view>
      </main>
      <StatusBar />
    </div>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  background: var(--color-background);
  font-family: var(--font-body);
}

.main-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
  background: var(--color-background);
}

.content-area {
  flex: 1;
  overflow: auto;
  padding: var(--spacing-2xl);
  background: var(--color-background);
}

/* 页面过渡动画 - 软糯滑入效果 */
.page-transition-enter-active,
.page-transition-leave-active {
  transition: all 0.35s var(--ease-spring);
}

.page-transition-enter-from {
  opacity: 0;
  transform: translateX(20px) scale(0.98);
}

.page-transition-leave-to {
  opacity: 0;
  transform: translateX(-15px) scale(0.98);
}
</style>
