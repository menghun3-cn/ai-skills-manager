import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Skill } from "@/types";

export const useSkillsStore = defineStore("skills", () => {
  const skills = ref<Skill[]>([]);
  const loading = ref(false);
  const searchQuery = ref("");
  const filterType = ref<"all" | "global" | "project">("all");
  const showDisabled = ref(false);
  const selectedSkills = ref<Set<string>>(new Set());

  const filteredSkills = computed(() => {
    let result = skills.value;

    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      result = result.filter(
        (skill) =>
          skill.name.toLowerCase().includes(query) ||
          skill.description?.toLowerCase().includes(query)
      );
    }

    if (filterType.value === "global") {
      result = result.filter((skill) => skill.bindings.global);
    } else if (filterType.value === "project") {
      result = result.filter((skill) => !skill.bindings.global && skill.bindings.projects.length > 0);
    }

    if (!showDisabled.value) {
      result = result.filter((skill) => skill.enabled);
    }

    return result;
  });

  async function loadSkills() {
    loading.value = true;
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<Skill[]>("get_skills");
      skills.value = result;
    } catch (error) {
      console.error("Failed to load skills:", error);
    } finally {
      loading.value = false;
    }
  }

  async function createSkill(name: string, description: string) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const newSkill = await invoke<Skill>("create_skill", { name, description });
      skills.value.push(newSkill);
      return newSkill;
    } catch (error) {
      console.error("Failed to create skill:", error);
      throw error;
    }
  }

  async function deleteSkill(name: string) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("delete_skill", { name });
      skills.value = skills.value.filter((s) => s.name !== name);
    } catch (error) {
      console.error("Failed to delete skill:", error);
      throw error;
    }
  }

  async function updateSkill(skill: Skill) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const updated = await invoke<Skill>("update_skill", { skill });
      const index = skills.value.findIndex((s) => s.name === updated.name);
      if (index !== -1) {
        skills.value[index] = updated;
      }
      return updated;
    } catch (error) {
      console.error("Failed to update skill:", error);
      throw error;
    }
  }

  async function syncSkill(skillName: string) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("sync_skill", { skillName });
    } catch (error) {
      console.error("Failed to sync skill:", error);
      throw error;
    }
  }

  async function syncAll() {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("sync_all");
    } catch (error) {
      console.error("Failed to sync all:", error);
      throw error;
    }
  }

  function toggleSkillSelection(skillName: string) {
    if (selectedSkills.value.has(skillName)) {
      selectedSkills.value.delete(skillName);
    } else {
      selectedSkills.value.add(skillName);
    }
  }

  function clearSelection() {
    selectedSkills.value.clear();
  }

  async function batchDelete() {
    const names = Array.from(selectedSkills.value);
    for (const name of names) {
      await deleteSkill(name);
    }
    clearSelection();
  }

  async function batchSync() {
    const names = Array.from(selectedSkills.value);
    for (const name of names) {
      await syncSkill(name);
    }
  }

  async function batchEnableToTools(skillNames: string[], toolIds: string[]): Promise<string | null> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("batch_sync_skills_to_tools", { skillNames, toolIds });
      return null;
    } catch (error) {
      const errorMsg = String(error);
      console.error("Failed to batch enable skills to tools:", error);
      
      if (errorMsg.includes("PERMISSION_DENIED") || errorMsg.includes("权限")) {
        return "创建软链接失败，权限不足。请使用管理员身份运行软件。";
      }
      return errorMsg;
    }
  }

  async function batchDisableFromTools(skillNames: string[], toolIds: string[]): Promise<string | null> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("batch_remove_skills_from_tools", { skillNames, toolIds });
      return null;
    } catch (error) {
      const errorMsg = String(error);
      console.error("Failed to batch disable skills from tools:", error);
      return errorMsg;
    }
  }

  return {
    skills,
    loading,
    searchQuery,
    filterType,
    showDisabled,
    selectedSkills,
    filteredSkills,
    loadSkills,
    createSkill,
    deleteSkill,
    updateSkill,
    syncSkill,
    syncAll,
    toggleSkillSelection,
    clearSelection,
    batchDelete,
    batchSync,
    batchEnableToTools,
    batchDisableFromTools,
  };
});
