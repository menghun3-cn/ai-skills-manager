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
const step = ref<"input" | "select">("input");

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
    error.value = String(e);
  } finally {
    discovering.value = false;
  }
}

async function handleImport() {
  if (selectedSkills.value.size === 0) return;

  loading.value = true;
  error.value = null;

  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const importedSkills = await invoke<string[]>("import_skills_from_github_repo", {
      owner: currentOwner.value,
      repo: currentRepo.value,
      selectedPaths: Array.from(selectedSkills.value),
    });
    console.log("Successfully imported skills:", importedSkills);
    emit("imported");
    await skillsStore.loadSkills();
    handleClose();
  } catch (e: any) {
    console.error("Failed to import skills from GitHub:", e);
    error.value = String(e);
  } finally {
    loading.value = false;
  }
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
      <template v-else>
        <div class="dialog-header mb-6 flex items-center gap-4" style="margin-bottom: 1.5rem;">
          <div
            class="dialog-icon flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-[var(--color-primary)]/10"
          >
            <FileText class="h-6 w-6 text-[var(--color-primary)]" />
          </div>
          <div class="dialog-title">
            <h2 class="text-xl font-semibold text-[var(--color-text-primary)]">
              选择要导入的 Skills
            </h2>
            <p class="mt-1 text-sm text-[var(--color-text-secondary)]">
              发现 {{ discoveredSkills.length }} 个 skills，选择要导入的项
            </p>
          </div>
        </div>

        <div class="dialog-body space-y-4" style="margin-bottom: 2rem;">
          <!-- 全选/取消全选 -->
          <div class="flex items-center justify-between mb-4">
            <button
              @click="toggleAll"
              class="text-sm text-[var(--color-primary)] hover:underline"
            >
              {{ selectedSkills.size === discoveredSkills.length ? '取消全选' : '全选' }}
            </button>
            <span class="text-sm text-[var(--color-text-secondary)]">
              已选择 {{ selectedSkills.size }} / {{ discoveredSkills.length }}
            </span>
          </div>

          <!-- Skills 列表 -->
          <div class="skills-list space-y-2 max-h-80 overflow-y-auto">
            <div
              v-for="skill in discoveredSkills"
              :key="skill.path"
              @click="toggleSkill(skill.path)"
              class="skill-item flex items-start gap-3 p-4 rounded-lg border-2 cursor-pointer transition-all"
              :class="{
                'border-[var(--color-primary)] bg-[var(--color-primary)]/5': selectedSkills.has(skill.path),
                'border-[var(--color-border)] hover:border-[var(--color-primary)]/50': !selectedSkills.has(skill.path)
              }"
            >
              <div class="checkbox mt-0.5">
                <div
                  class="w-5 h-5 rounded border-2 flex items-center justify-center transition-colors"
                  :class="{
                    'bg-[var(--color-primary)] border-[var(--color-primary)]': selectedSkills.has(skill.path),
                    'border-[var(--color-border)]': !selectedSkills.has(skill.path)
                  }"
                >
                  <Check
                    v-if="selectedSkills.has(skill.path)"
                    class="w-3 h-3 text-white"
                    :stroke-width="3"
                  />
                </div>
              </div>
              <div class="skill-info flex-1">
                <h3 class="font-medium text-[var(--color-text-primary)]">
                  {{ skill.name }}
                </h3>
                <p class="text-xs text-[var(--color-text-secondary)] mt-1">
                  {{ skill.path }}
                </p>
                <p
                  v-if="skill.description"
                  class="text-sm text-[var(--color-text-secondary)] mt-1 line-clamp-2"
                >
                  {{ skill.description }}
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

        <div class="dialog-footer mt-8 flex justify-between gap-4" style="margin-top: 2rem; display: flex; justify-content: space-between; gap: 1rem;">
          <button
            @click="goBack"
            class="btn btn-secondary dialog-btn"
            :disabled="loading"
            style="padding: 0.75rem 1.5rem; border: 2px solid var(--color-border); border-radius: var(--radius-md); background: var(--color-surface); cursor: pointer;"
          >
            返回
          </button>
          <div class="flex gap-4">
            <button
              @click="handleClose"
              class="btn btn-secondary dialog-btn"
              :disabled="loading"
              style="padding: 0.75rem 1.5rem; border: 2px solid var(--color-border); border-radius: var(--radius-md); background: var(--color-surface); cursor: pointer;"
            >
              <X class="h-4 w-4" :stroke-width="1.5" />
              {{ t("common.cancel") }}
            </button>
            <button
              @click="handleImport"
              :disabled="selectedSkills.size === 0 || loading"
              class="btn btn-primary dialog-btn"
              style="padding: 0.75rem 1.5rem; border-radius: var(--radius-md); background: linear-gradient(135deg, var(--peach-300), var(--peach-400)); color: white; cursor: pointer; border: none;"
            >
              <Loader2
                v-if="loading"
                class="h-4 w-4 animate-spin"
                :stroke-width="1.5"
              />
              <Github v-else class="h-4 w-4" :stroke-width="1.5" />
              {{ loading ? t("common.loading") : `导入 ${selectedSkills.size} 个 Skills` }}
            </button>
          </div>
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
