import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Tool, Skill } from "@/types";

export const useToolsStore = defineStore("tools", () => {
  const tools = ref<Tool[]>([]);
  const loading = ref(false);
  const searchQuery = ref("");
  const installedSkills = ref<Skill[]>([]);
  const toolEnabledSkills = ref<Map<string, Set<string>>>(new Map());

  const enabledTools = computed(() => tools.value.filter((t) => t.enabled));
  const enabledCount = computed(() => enabledTools.value.length);

  // 只显示已检测到的工具
  const detectedTools = computed(() => tools.value.filter((t) => t.detected));

  // 已检测且已启用的工具（用于技能管理页面）
  const detectedAndEnabledTools = computed(() =>
    tools.value.filter((t) => t.detected && t.enabled)
  );

  // 过滤后的工具（用于工具页面，显示所有工具）
  const filteredTools = computed(() => {
    let result = tools.value;
    if (searchQuery.value.trim()) {
      const query = searchQuery.value.toLowerCase();
      result = result.filter((tool) =>
        tool.name.toLowerCase().includes(query)
      );
    }
    return result;
  });

  async function detectTools() {
    loading.value = true;
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<Tool[]>("detect_tools");
      tools.value = result;
    } catch (error) {
      console.error("Failed to detect tools:", error);
    } finally {
      loading.value = false;
    }
  }

  // 加载工具列表（别名，用于与其他 store 保持一致）
  async function loadTools() {
    return detectTools();
  }

  async function addCustomTool(path: string) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const newTool = await invoke<Tool>("add_custom_tool", { path });
      tools.value.push(newTool);
      return newTool;
    } catch (error) {
      console.error("Failed to add custom tool:", error);
      throw error;
    }
  }

  async function toggleTool(id: string) {
    const tool = tools.value.find((t) => t.id === id);
    if (tool) {
      tool.enabled = !tool.enabled;
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("update_tool", { tool });
      } catch (error) {
        tool.enabled = !tool.enabled;
        console.error("Failed to toggle tool:", error);
        throw error;
      }
    }
  }

  async function enableAll() {
    for (const tool of tools.value) {
      if (!tool.enabled) {
        await toggleTool(tool.id);
      }
    }
  }

  async function disableAll() {
    for (const tool of tools.value) {
      if (tool.enabled) {
        await toggleTool(tool.id);
      }
    }
  }

  async function openPath(path: string): Promise<string | null> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("open_path", { path });
      return null;
    } catch (error) {
      const errorMsg = String(error);
      console.error("Failed to open path:", error);
      
      // 检查是否是路径不存在错误
      if (errorMsg.includes("PATH_NOT_FOUND")) {
        return "目录不存在";
      }
      return errorMsg;
    }
  }

  // 加载已安装的技能列表
  async function loadInstalledSkills() {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<Skill[]>("get_installed_skills");
      console.log("Loaded installed skills:", result);
      installedSkills.value = result;
    } catch (error) {
      console.error("Failed to load installed skills:", error);
    }
  }

  // 加载指定工具的已启用技能
  async function loadToolEnabledSkills(toolId: string) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<string[]>("get_tool_enabled_skills", { toolId });
      toolEnabledSkills.value.set(toolId, new Set(result));
    } catch (error) {
      console.error("Failed to load tool enabled skills:", error);
    }
  }

  // 检查技能是否已启用（同时检查原始名称和转换后的名称）
  function isSkillEnabled(toolId: string, skillName: string): boolean {
    const enabledSkills = toolEnabledSkills.value.get(toolId);
    if (!enabledSkills) return false;

    // 检查原始名称
    if (enabledSkills.has(skillName)) return true;

    // 检查转换后的名称（空格替换为连字符并转为小写，与安装时一致）
    const normalizedName = skillName.replace(/ /g, '-').toLowerCase();
    if (enabledSkills.has(normalizedName)) return true;

    // 反过来检查：如果存储的是转换后的名称，但传入的是原始名称
    for (const enabledSkill of enabledSkills) {
      const enabledNormalized = enabledSkill.replace(/ /g, '-').toLowerCase();
      if (enabledNormalized === normalizedName) return true;
    }

    // 额外检查：忽略大小写和连字符/下划线差异
    for (const enabledSkill of enabledSkills) {
      const enabledClean = enabledSkill.toLowerCase().replace(/[-_]/g, '');
      const skillClean = skillName.toLowerCase().replace(/[-_]/g, '');
      if (enabledClean === skillClean) return true;
    }

    return false;
  }

  // 为工具启用技能
  async function enableSkillForTool(toolId: string, skillName: string): Promise<string | null> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("enable_skill_for_tool", { toolId, skillName });

      // 成功后更新本地状态
      if (!toolEnabledSkills.value.has(toolId)) {
        toolEnabledSkills.value.set(toolId, new Set());
      }
      toolEnabledSkills.value.get(toolId)?.add(skillName);

      return null;
    } catch (error) {
      const errorMsg = String(error);
      console.error("Failed to enable skill for tool:", error);

      // 检查是否是权限错误
      if (errorMsg.includes("PERMISSION_DENIED")) {
        return "创建软链接失败，权限不足。请使用管理员身份运行软件。";
      }
      return errorMsg;
    }
  }

  // 为工具停用技能
  async function disableSkillForTool(toolId: string, skillName: string): Promise<string | null> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("disable_skill_for_tool", { toolId, skillName });

      // 成功后更新本地状态
      toolEnabledSkills.value.get(toolId)?.delete(skillName);

      return null;
    } catch (error) {
      const errorMsg = String(error);
      console.error("Failed to disable skill for tool:", error);
      return errorMsg;
    }
  }

  return {
    tools,
    loading,
    searchQuery,
    installedSkills,
    toolEnabledSkills,
    enabledTools,
    enabledCount,
    detectedTools,
    detectedAndEnabledTools,
    filteredTools,
    detectTools,
    loadTools,
    addCustomTool,
    toggleTool,
    enableAll,
    disableAll,
    openPath,
    loadInstalledSkills,
    loadToolEnabledSkills,
    isSkillEnabled,
    enableSkillForTool,
    disableSkillForTool,
  };
});
