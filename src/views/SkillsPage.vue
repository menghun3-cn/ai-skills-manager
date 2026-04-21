<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useSkillsStore } from "@/stores/skills";
import type { Skill } from "@/types";
import SkillCard from "@/components/skills/SkillCard.vue";
import SkillEditor from "@/components/skills/SkillEditor.vue";
import SkillDetail from "@/components/skills/SkillDetail.vue";
import SkillManageDialog from "@/components/skills/SkillManageDialog.vue";
import CreateSkillDialog from "@/components/skills/CreateSkillDialog.vue";
import BatchActions from "@/components/skills/BatchActions.vue";

import EmptyState from "@/components/common/EmptyState.vue";
import PageHeader from "@/components/common/PageHeader.vue";
import { Plus, RefreshCw, LayoutGrid, Search, Package } from "lucide-vue-next";

const { t } = useI18n();
const skillsStore = useSkillsStore();

const showEditor = ref(false);
const editingSkill = ref<string | null>(null);
const showCreateDialog = ref(false);
const batchMode = ref(false);
const showDetail = ref(false);
const showManage = ref(false);
const selectedSkill = ref<Skill | null>(null);


const statistics = computed(() => {
  const globalCount = skillsStore.skills.filter(s => s.bindings.global).length;
  const projectCount = skillsStore.skills.filter(s => !s.bindings.global && s.bindings.projects.length > 0).length;
  return [
    { label: t("skills.stats.total"), value: skillsStore.skills.length, highlight: true },
    { label: t("skills.stats.global"), value: globalCount },
    { label: t("skills.stats.project"), value: projectCount },
  ];
});

onMounted(() => {
  skillsStore.loadSkills();
});

function handleEdit(skillName: string) {
  editingSkill.value = skillName;
  showEditor.value = true;
}

function handleShowDetail(skill: Skill) {
  if (batchMode.value) return;
  // 打开技能详情对话框
  selectedSkill.value = skill;
  showDetail.value = true;
}

function handleCloseDetail() {
  showDetail.value = false;
  selectedSkill.value = null;
}

function handleManage(skill: Skill) {
  if (batchMode.value) return;
  // 打开技能管理对话框（仅IDE管理）
  selectedSkill.value = skill;
  showManage.value = true;
}

function handleCloseManage() {
  showManage.value = false;
  selectedSkill.value = null;
}

async function handleDeleteSkill(skillName: string) {
  try {
    await skillsStore.deleteSkill(skillName);
    showDetail.value = false;
    selectedSkill.value = null;
  } catch (error) {
    console.error("Failed to delete skill:", error);
  }
}

function handleCloseEditor() {
  showEditor.value = false;
  editingSkill.value = null;
}

function handleCreate() {
  showCreateDialog.value = true;
}

async function handleRefresh() {
  await skillsStore.loadSkills();
}
</script>

<template>
  <div class="page">
    <PageHeader
      :title="t('skills.title')"
      :icon="Package"
      :statistics="statistics"
    >
      <template #actions>
        <button
          v-if="batchMode"
          @click="batchMode = false"
          class="btn btn-secondary"
        >
          {{ t("common.close") }}
        </button>
        <button
          v-else
          @click="batchMode = true"
          class="btn btn-secondary"
        >
          <LayoutGrid class="h-4 w-4" :stroke-width="1.5" />
          {{ t("skills.batch") }}
        </button>
        <button
          @click="handleRefresh"
          class="btn btn-secondary"
        >
          <RefreshCw class="h-4 w-4" :stroke-width="1.5" />
          {{ t("skills.refresh") }}
        </button>
        <button
          @click="handleCreate"
          class="btn btn-primary"
        >
          <Plus class="h-4 w-4" :stroke-width="1.5" />
          {{ t("skills.new") }}
        </button>
      </template>
    </PageHeader>

    <div class="toolbar">
      <div class="search-box">
        <Search class="search-icon" :stroke-width="1.5" />
        <input
          v-model="skillsStore.searchQuery"
          type="text"
          :placeholder="t('skills.search')"
          class="input search-input"
        />
      </div>
      <div class="filter-group">
        <button
          v-for="tab in ['all', 'global', 'project']"
          :key="tab"
          @click="skillsStore.filterType = tab as any"
          class="filter-btn"
          :class="{ active: skillsStore.filterType === tab }"
        >
          {{ t(`skills.${tab === 'all' ? 'all' : tab === 'global' ? 'global' : 'projectBound'}`) }}
        </button>
      </div>
      <label class="filter-checkbox">
        <input
          type="checkbox"
          v-model="skillsStore.showDisabled"
        />
        {{ t("skills.showDisabled") }}
      </label>
    </div>

    <BatchActions v-if="batchMode" />

    <div v-if="skillsStore.loading" class="loading-state">
      <div class="loading-spinner">
        <div class="spinner-dot"></div>
        <div class="spinner-dot"></div>
        <div class="spinner-dot"></div>
      </div>
      <span class="loading-text">{{ t("common.loading") }}</span>
    </div>

    <EmptyState
      v-else-if="skillsStore.filteredSkills.length === 0"
      :title="t('skills.noSkills')"
      :description="t('skills.noSkillsDesc')"
    />

    <div v-else class="card-grid">
      <SkillCard
        v-for="(skill, index) in skillsStore.filteredSkills"
        :key="skill.name"
        :skill="skill"
        :batch-mode="batchMode"
        :style="{ animationDelay: String(index * 50) + 'ms' }"
        class="animate-fade-in"
        @edit="handleEdit"
        @click="handleShowDetail(skill)"
        @manage="handleManage"
      />
    </div>

    <SkillEditor
      v-if="showEditor && editingSkill"
      :skill-name="editingSkill"
      @close="handleCloseEditor"
    />

    <CreateSkillDialog
      v-if="showCreateDialog"
      @close="showCreateDialog = false"
      @created="showCreateDialog = false"
    />

    <SkillDetail
      v-if="showDetail && selectedSkill"
      :skill="selectedSkill"
      @close="handleCloseDetail"
      @edit="handleEdit"
      @delete="handleDeleteSkill"
    />

    <SkillManageDialog
      v-if="showManage && selectedSkill"
      :skill="selectedSkill"
      :show="showManage"
      @close="handleCloseManage"
    />

  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-xl);
  flex-wrap: wrap;
  padding: var(--spacing-lg);
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);
  border: 2px solid var(--color-border-subtle);
}

.search-box {
  position: relative;
  width: 280px;
}

.search-icon {
  position: absolute;
  left: var(--spacing-md);
  top: 50%;
  transform: translateY(-50%);
  width: 18px;
  height: 18px;
  color: var(--color-text-tertiary);
  transition: color var(--transition-fast) var(--ease-out);
}

.search-box:focus-within .search-icon {
  color: var(--peach-400);
}

.search-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  padding-left: 40px;
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  transition: all var(--transition-fast) var(--ease-out);
}

.search-input:hover {
  border-color: var(--cream-400);
}

.search-input:focus {
  outline: none;
  border-color: var(--peach-300);
  box-shadow: 0 0 0 4px var(--peach-100);
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

.filter-group {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs);
  background: var(--color-surface-subtle);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-md);
}

.filter-btn {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--text-base);
  font-weight: 500;
  color: var(--color-text-secondary);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
  font-family: var(--font-body);
}

.filter-btn:hover {
  color: var(--color-text-primary);
  background: var(--color-surface);
}

.filter-btn.active {
  color: white;
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  box-shadow: var(--shadow-soft);
}

.filter-checkbox {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast) var(--ease-out);
}

.filter-checkbox:hover {
  background: var(--color-surface-subtle);
}

.filter-checkbox input {
  width: 18px;
  height: 18px;
  accent-color: var(--peach-400);
  cursor: pointer;
  border-radius: var(--radius-sm);
}

.loading-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-lg);
  padding: var(--spacing-5xl);
}

.loading-spinner {
  display: flex;
  gap: var(--spacing-sm);
}

.spinner-dot {
  width: 12px;
  height: 12px;
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  border-radius: var(--radius-full);
  animation: bounce 1.4s ease-in-out infinite both;
}

.spinner-dot:nth-child(1) {
  animation-delay: -0.32s;
}

.spinner-dot:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0.6);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

.loading-text {
  color: var(--color-text-secondary);
  font-size: var(--text-base);
  font-weight: 500;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--spacing-lg);
  flex: 1;
  align-items: start;
}

.btn {
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

.btn-primary {
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  color: white;
  box-shadow: var(--shadow-clay);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card-hover);
}

.btn-primary:active {
  transform: scale(0.96);
  box-shadow: var(--shadow-pressed);
}

.btn-secondary {
  background: var(--color-surface-subtle);
  color: var(--color-text-primary);
  border-color: var(--color-border);
}

.btn-secondary:hover {
  background: var(--peach-50);
  border-color: var(--peach-200);
  color: var(--peach-500);
}
</style>
