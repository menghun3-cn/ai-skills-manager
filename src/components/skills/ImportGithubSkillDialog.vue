<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useSkillsStore } from "@/stores/skills";
import { X, Loader2, Github, Check, FileText } from "lucide-vue-next";

interface DiscoveredSkill {
  name: string;
  path: string;
  description?: string;
}

const emit = defineEmits<{
  close: [];
  imported: [];
}>();

const { t } = useI18n();
const skillsStore = useSkillsStore();

const repoUrl = ref("");
const loading = ref(false);
const discovering = ref(false);
const error = ref<string | null>(null);
const discoveredSkills = ref<DiscoveredSkill[]>([]);
const selectedSkills = ref<Set<string>>(new Set());
const currentOwner = ref("");
const currentRepo = ref("");
const step = ref<"input" | "select" | "confirmOverwrite">("input");
const overwriteSkillName = ref<string | null>(null);

function parseGithubRepo(url: string): { owner: string; repo: string } | null {
  // 支持多种 GitHub URL 格式
  const patterns = [
    // https://github.com/owner/repo
    /https?:\/\/github\.com\/([^\/]+)\/([^\/]+)(?:\/.*)?$/,
    // github.com/owner/repo
    /github\.com\/([^\/]+)\/([^\/]+)(?:\/.*)?$/,
    // owner/repo
    /^([^\/]+)\/([^\/]+)$/,
  ];

  for (const pattern of patterns) {
    const match = url.trim().match(pattern);
    if (match) {
      return {
        owner: match[1],
        repo: match[2].replace(/\.git$/, ""), // 移除 .git 后缀
      };
    }
  }
  return null;
}

async function handleDiscover() {
  if (!repoUrl.value.trim()) return;

  const parsed = parseGithubRepo(repoUrl.value);
  if (!parsed) {
    error.value = t("skills.invalidGithubUrl");
    return;
  }

  discovering.value = true;
  error.value = null;
  discoveredSkills.value = [];

  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const skills = await invoke<DiscoveredSkill[]>("discover_skills_from_github_repo", {
      owner: parsed.owner,
      repo: parsed.repo,
    });
    
    discoveredSkills.value = skills;
    currentOwner.value = parsed.owner;
    currentRepo.value = parsed.repo;
    
    // 默认全选
    selectedSkills.value = new Set(skills.map(s => s.path));
    
    if (skills.length > 0) {
      step.value = "select";
    } else {
      error.value = "未在仓库中发现任何 skill";
    }
  } catch (e: any) {
    console.error("Failed to discover skills from GitHub:", e);
    // 提取错误信息 - Tauri 错误可能是字符串或对象
    if (typeof e === 'string') {
      error.value = e;
    } else if (e && typeof e === 'object') {
      error.value = e.message || e.error || JSON.stringify(e);
    } else {
      error.value = String(e);
    }
  } finally {
    discovering.value = false;
  }
}

async function handleImport(force = false) {
  if (selectedSkills.value.size === 0) return;

  loading.value = true;
  error.value = null;

  try {
    console.log("Starting import with:", {
      owner: currentOwner.value,
      repo: currentRepo.value,
      selectedPaths: Array.from(selectedSkills.value),
      force,
    });
    
    const { invoke } = await import("@tauri-apps/api/core");
    
    let importedSkills: string[];
    
    if (force) {
      // 强制覆盖导入
      importedSkills = await invoke<string[]>("import_skills_from_github_repo_force", {
        owner: currentOwner.value,
        repo: currentRepo.value,
        selectedPaths: Array.from(selectedSkills.value),
      });
    } else {
      importedSkills = await invoke<string[]>("import_skills_from_github_repo", {
        owner: currentOwner.value,
        repo: currentRepo.value,
        selectedPaths: Array.from(selectedSkills.value),
      });
    }
    
    console.log("Successfully imported skills:", importedSkills);
    emit("imported");
    await skillsStore.loadSkills();
    handleClose();
  } catch (e: any) {
    console.error("Failed to import skills from GitHub:", e);
    console.error("Error type:", typeof e);
    console.error("Error keys:", e && typeof e === 'object' ? Object.keys(e) : 'N/A');
    
    // 提取错误信息 - Tauri 错误可能是字符串或对象
    let errorMsg: string;
    if (typeof e === 'string') {
      errorMsg = e;
    } else if (e && typeof e === 'object') {
      errorMsg = e.message || e.error || JSON.stringify(e);
    } else {
      errorMsg = String(e);
    }
    
    console.error("Extracted error message:", errorMsg);
    
    // 检查是否是 skill 已存在的错误
    if (errorMsg.startsWith("SKILL_EXISTS:")) {
      const skillName = errorMsg.replace("SKILL_EXISTS:", "");
      overwriteSkillName.value = skillName;
      step.value = "confirmOverwrite";
      error.value = null;
    } else {
      error.value = errorMsg;
    }
  } finally {
    loading.value = false;
  }
}

function confirmOverwrite() {
  handleImport(true);
}

function cancelOverwrite() {
  overwriteSkillName.value = null;
  step.value = "select";
}

function toggleSkill(path: string) {
  if (selectedSkills.value.has(path)) {
    selectedSkills.value.delete(path);
  } else {
    selectedSkills.value.add(path);
  }
}

function toggleAll() {
  if (selectedSkills.value.size === discoveredSkills.value.length) {
    selectedSkills.value.clear();
  } else {
    selectedSkills.value = new Set(discoveredSkills.value.map(s => s.path));
  }
}

function handleClose() {
  if (!loading.value && !discovering.value) {
    emit("close");
  }
}

function goBack() {
  step.value = "input";
  discoveredSkills.value = [];
  selectedSkills.value.clear();
  error.value = null;
}
</script>

<template>
  <div
    class="github-import-dialog fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 backdrop-blur-sm"
    @click.self="handleClose"
  >
    <div
      class="dialog-content w-full max-w-2xl rounded-2xl bg-[var(--color-surface-elevated)] p-8 shadow-2xl max-h-[90vh] overflow-y-auto"
      style="padding: 2rem;"
    >
      <!-- 输入步骤 -->
      <template v-if="step === 'input'">
        <div class="dialog-header mb-6 flex items-center gap-4" style="margin-bottom: 1.5rem;">
          <div
            class="dialog-icon flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-[var(--color-primary)]/10"
          >
            <Github class="h-6 w-6 text-[var(--color-primary)]" />
          </div>
          <div class="dialog-title">
            <h2 class="text-xl font-semibold text-[var(--color-text-primary)]">
              {{ t("skills.importFromGithub") }}
            </h2>
            <p class="mt-1 text-sm text-[var(--color-text-secondary)]">
              {{ t("skills.importFromGithubDesc") }}
            </p>
          </div>
        </div>

        <div class="dialog-body space-y-5" style="margin-bottom: 2rem;">
          <div class="form-field">
            <label
              class="field-label mb-2 block text-sm font-medium text-[var(--color-text-primary)]"
              style="margin-bottom: 0.5rem; display: block;"
            >
              {{ t("skills.githubRepoUrl") }}
            </label>
            <input
              v-model="repoUrl"
              type="text"
              :placeholder="t('skills.githubRepoPlaceholder')"
              class="input field-input"
              :disabled="discovering"
              @keyup.enter="handleDiscover"
              style="width: 100%; padding: 0.75rem 1rem; border: 2px solid var(--color-border); border-radius: var(--radius-md); font-size: var(--text-base);"
            />
            <p class="field-hint mt-2 text-xs text-[var(--color-text-secondary)]" style="margin-top: 0.5rem;">
              {{ t("skills.githubUrlHint") }}
            </p>
          </div>

          <div
            v-if="error"
            class="error-message rounded-lg bg-red-500/10 p-4 text-sm text-red-500"
            style="padding: 1rem;"
          >
            {{ error }}
          </div>
        </div>

        <div class="dialog-footer mt-8 flex justify-end gap-4" style="margin-top: 2rem; display: flex; justify-content: flex-end; gap: 1rem;">
          <button
            @click="handleClose"
            class="btn btn-secondary dialog-btn"
            :disabled="discovering"
            style="padding: 0.75rem 1.5rem; border: 2px solid var(--color-border); border-radius: var(--radius-md); background: var(--color-surface); cursor: pointer;"
          >
            <X class="h-4 w-4" :stroke-width="1.5" />
            {{ t("common.cancel") }}
          </button>
          <button
            @click="handleDiscover"
            :disabled="!repoUrl.trim() || discovering"
            class="btn btn-primary dialog-btn"
            style="padding: 0.75rem 1.5rem; border-radius: var(--radius-md); background: linear-gradient(135deg, var(--peach-300), var(--peach-400)); color: white; cursor: pointer; border: none;"
          >
            <Loader2
              v-if="discovering"
              class="h-4 w-4 animate-spin"
              :stroke-width="1.5"
            />
            <Github v-else class="h-4 w-4" :stroke-width="1.5" />
            {{ discovering ? t("common.loading") : "发现 Skills" }}
          </button>
        </div>
      </template>

      <!-- 选择步骤 -->
      <template v-if="step === 'select'">
        <div class="select-header mb-6" style="margin-bottom: 1.5rem;">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
              <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-[var(--color-primary)]/10">
                <FileText class="h-5 w-5 text-[var(--color-primary)]" :stroke-width="1.5" />
              </div>
              <div>
                <h2 class="text-lg font-semibold text-[var(--color-text-primary)]">
                  选择要导入的 Skills
                </h2>
                <p class="text-sm text-[var(--color-text-secondary)]">
                  发现 {{ discoveredSkills.length }} 个 skills
                </p>
              </div>
            </div>
            <button
              @click="toggleAll"
              class="px-3 py-1.5 text-sm font-medium text-[var(--color-primary)] rounded-md hover:bg-[var(--color-primary)]/10 transition-colors"
            >
              {{ selectedSkills.size === discoveredSkills.length ? '取消全选' : '全选' }}
            </button>
          </div>
        </div>

        <div class="dialog-body" style="margin-bottom: 2rem;">
          <!-- Skills 列表 -->
          <div class="skills-list space-y-3 max-h-96 overflow-y-auto pr-2" style="scrollbar-width: thin; scrollbar-color: var(--color-border) transparent;">
            <div
              v-for="skill in discoveredSkills"
              :key="skill.path"
              @click="toggleSkill(skill.path)"
              class="skill-item group relative flex items-start gap-4 p-4 rounded-xl border-2 cursor-pointer transition-all duration-200"
              :class="{
                'border-[var(--color-primary)] bg-[var(--color-primary)]/5 shadow-sm': selectedSkills.has(skill.path),
                'border-[var(--color-border)] hover:border-[var(--color-primary)]/30 hover:shadow-sm': !selectedSkills.has(skill.path)
              }"
              role="checkbox"
              :aria-checked="selectedSkills.has(skill.path)"
            >
              <!-- 复选框 -->
              <div
                class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-md border-2 transition-all duration-200"
                :class="{
                  'border-[var(--color-primary)] bg-[var(--color-primary)]': selectedSkills.has(skill.path),
                  'border-[var(--color-border)] group-hover:border-[var(--color-primary)]/50': !selectedSkills.has(skill.path)
                }"
              >
                <Check
                  v-if="selectedSkills.has(skill.path)"
                  class="h-4 w-4 text-white"
                  :stroke-width="2.5"
                />
              </div>

              <!-- Skill 信息 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <h3 class="font-semibold text-[var(--color-text-primary)] truncate">
                    {{ skill.name }}
                  </h3>
                  <span v-if="selectedSkills.has(skill.path)" class="px-2 py-0.5 text-xs font-medium rounded-full bg-[var(--color-primary)]/10 text-[var(--color-primary)]">
                    已选择
                  </span>
                </div>
                <p class="text-xs text-[var(--color-text-secondary)] mb-2">
                  {{ skill.path }}
                </p>
                <p
                  v-if="skill.description"
                  class="text-sm text-[var(--color-text-secondary)] line-clamp-2 leading-relaxed"
                >
                  {{ skill.description }}
                </p>
              </div>

              <!-- 右侧图标 -->
              <div class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
                <svg class="h-5 w-5 text-[var(--color-text-secondary)]" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                </svg>
              </div>
            </div>
          </div>

          <!-- 底部统计和错误 -->
          <div class="mt-4 flex items-center justify-between">
            <span class="text-sm text-[var(--color-text-secondary)]">
              已选择 <span class="font-semibold text-[var(--color-primary)]">{{ selectedSkills.size }}</span> / {{ discoveredSkills.length }} 个
            </span>
            <div
              v-if="error"
              class="text-sm text-red-500"
            >
              {{ error }}
            </div>
          </div>
        </div>

        <div class="dialog-footer flex justify-end gap-3" style="display: flex; justify-content: flex-end; gap: 0.75rem;">
          <button
            @click="goBack"
            class="px-5 py-2.5 text-sm font-medium rounded-lg border-2 border-[var(--color-border)] bg-[var(--color-surface)] text-[var(--color-text-primary)] hover:bg-[var(--color-surface-hover)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="loading"
          >
            返回
          </button>
          <button
            @click="handleClose"
            class="px-5 py-2.5 text-sm font-medium rounded-lg border-2 border-[var(--color-border)] bg-[var(--color-surface)] text-[var(--color-text-primary)] hover:bg-[var(--color-surface-hover)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="loading"
          >
            <X class="inline-block h-4 w-4 mr-1.5 align-text-bottom" :stroke-width="1.5" />
            {{ t("common.cancel") }}
          </button>
          <button
            @click="handleImport"
            :disabled="selectedSkills.size === 0 || loading"
            class="px-5 py-2.5 text-sm font-medium rounded-lg bg-gradient-to-r from-[var(--peach-300)] to-[var(--peach-400)] text-white shadow-sm hover:shadow-md transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:shadow-sm"
          >
            <Loader2 v-if="loading" class="inline-block h-4 w-4 mr-1.5 animate-spin align-text-bottom" :stroke-width="1.5" />
            <Github v-else class="inline-block h-4 w-4 mr-1.5 align-text-bottom" :stroke-width="1.5" />
            {{ loading ? t("common.loading") : `导入 ${selectedSkills.size} 个 Skills` }}
          </button>
        </div>
      </template>

      <!-- 覆盖确认步骤 -->
      <template v-if="step === 'confirmOverwrite'">
        <div class="dialog-header mb-6 flex items-center gap-4" style="margin-bottom: 1.5rem;">
          <div
            class="dialog-icon flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-red-500/10"
          >
            <svg class="h-6 w-6 text-red-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
              <line x1="12" y1="9" x2="12" y2="13"></line>
              <line x1="12" y1="17" x2="12.01" y2="17"></line>
            </svg>
          </div>
          <div class="dialog-title">
            <h2 class="text-xl font-semibold text-[var(--color-text-primary)]">
              确认覆盖
            </h2>
            <p class="mt-1 text-sm text-[var(--color-text-secondary)]">
              此操作不可撤销，请确认是否继续
            </p>
          </div>
        </div>

        <div class="dialog-body space-y-4" style="margin-bottom: 2rem;">
          <div class="p-5 rounded-xl bg-red-50 dark:bg-red-900/10 border border-red-200 dark:border-red-800/30">
            <div class="flex items-start gap-3">
              <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-red-100 dark:bg-red-900/30">
                <svg class="h-4 w-4 text-red-600 dark:text-red-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
                  <line x1="12" y1="9" x2="12" y2="13"></line>
                  <line x1="12" y1="17" x2="12.01" y2="17"></line>
                </svg>
              </div>
              <div>
                <h3 class="font-semibold text-red-800 dark:text-red-300 mb-1">
                  即将覆盖 Skill
                </h3>
                <p class="text-sm text-red-700 dark:text-red-400">
                  <strong class="font-semibold">{{ overwriteSkillName }}</strong> 已经存在。覆盖将删除现有的 skill 及其所有文件，并重新从 GitHub 导入。
                </p>
              </div>
            </div>
          </div>

          <div
            v-if="error"
            class="error-message rounded-lg bg-red-500/10 p-4 text-sm text-red-500"
            style="padding: 1rem;"
          >
            {{ error }}
          </div>
        </div>

        <div class="dialog-footer flex justify-end gap-3" style="display: flex; justify-content: flex-end; gap: 0.75rem;">
          <button
            @click="cancelOverwrite"
            class="px-5 py-2.5 text-sm font-medium rounded-lg border-2 border-[var(--color-border)] bg-[var(--color-surface)] text-[var(--color-text-primary)] hover:bg-[var(--color-surface-hover)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="loading"
          >
            取消
          </button>
          <button
            @click="confirmOverwrite"
            :disabled="loading"
            class="px-5 py-2.5 text-sm font-medium rounded-lg bg-gradient-to-r from-red-500 to-red-600 text-white shadow-sm hover:shadow-md transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:shadow-sm"
          >
            <Loader2
              v-if="loading"
              class="inline-block h-4 w-4 mr-1.5 animate-spin align-text-bottom"
              :stroke-width="1.5"
            />
            <span v-else>确认覆盖</span>
          </button>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.skills-list {
  scrollbar-width: thin;
  scrollbar-color: var(--color-border) transparent;
}

.skills-list::-webkit-scrollbar {
  width: 6px;
}

.skills-list::-webkit-scrollbar-track {
  background: transparent;
}

.skills-list::-webkit-scrollbar-thumb {
  background-color: var(--color-border);
  border-radius: 3px;
}
</style>
