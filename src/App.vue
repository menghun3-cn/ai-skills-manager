<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useSettingsStore } from "./stores/settings";
import MainLayout from "./views/MainLayout.vue";
import { AlertTriangle, X } from "lucide-vue-next";

const settingsStore = useSettingsStore();
const showAdminWarning = ref(false);

onMounted(async () => {
  await settingsStore.loadSettings();
  settingsStore.applyTheme();
  
  // 检查管理员权限
  await checkAdminPrivileges();
});

async function checkAdminPrivileges() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const isAdmin = await invoke<boolean>("check_admin_privileges");
    
    if (!isAdmin) {
      showAdminWarning.value = true;
    }
  } catch (error) {
    console.error("Failed to check admin privileges:", error);
  }
}

function closeAdminWarning() {
  showAdminWarning.value = false;
}
</script>

<template>
  <div class="app-container">
    <!-- 管理员权限警告 -->
    <div v-if="showAdminWarning" class="admin-warning">
      <div class="warning-content">
        <AlertTriangle class="warning-icon" :stroke-width="1.5" />
        <div class="warning-text">
          <span class="warning-title">需要管理员权限</span>
          <span class="warning-desc">软件需要管理员权限才能正常创建软链接，请使用管理员身份重新运行</span>
        </div>
        <button @click="closeAdminWarning" class="warning-close" aria-label="关闭">
          <X class="h-4 w-4" :stroke-width="1.5" />
        </button>
      </div>
    </div>
    <MainLayout />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.admin-warning {
  flex-shrink: 0;
  background: linear-gradient(135deg, var(--strawberry-500) 0%, var(--strawberry-600) 100%);
  padding: var(--spacing-md) var(--spacing-lg);
}

.warning-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  max-width: 1200px;
  margin: 0 auto;
}

.warning-icon {
  width: 24px;
  height: 24px;
  color: white;
  flex-shrink: 0;
}

.warning-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.warning-title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: white;
}

.warning-desc {
  font-size: var(--text-xs);
  color: rgba(255, 255, 255, 0.9);
}

.warning-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: rgba(255, 255, 255, 0.2);
  border: none;
  border-radius: var(--radius-sm);
  color: white;
  cursor: pointer;
  transition: background var(--transition-fast) var(--ease-out);
  flex-shrink: 0;
}

.warning-close:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>

<style scoped>
</style>
