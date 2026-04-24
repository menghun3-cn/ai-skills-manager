import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { Settings } from "@/types";

const defaultSettings: Settings = {
  editor: "monaco",
  autoSync: true,
  cleanupOnDisable: false,
  syncNotifications: true,
  refreshInterval: 30,
  theme: "system",
  fontFamily: "system-ui",
  fontSize: 14,
  language: "zh-CN",
  enabledDataSources: ["awesome-claude-skills", "skills-sh"],
  proxyUrl: undefined,
};

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings>({ ...defaultSettings });
  const configPath = ref<string>("");
  const githubToken = ref<string>("");

  const theme = computed(() => settings.value.theme);
  const language = computed(() => settings.value.language);

  async function loadSettings() {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<{ settings: Settings; githubToken?: string }>("get_config");
      settings.value = { ...defaultSettings, ...result.settings };
      githubToken.value = result.githubToken || "";
      const pathResult = await invoke<string>("get_app_dir_path");
      configPath.value = pathResult;
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  }

  async function saveSettings(newSettings: Partial<Settings>) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      settings.value = { ...settings.value, ...newSettings };
      await invoke("update_config", { config: settings.value });
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  }

  function applyTheme() {
    const themeValue = settings.value.theme;
    if (themeValue === "system") {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      document.documentElement.setAttribute("data-theme", prefersDark ? "dark" : "light");
    } else {
      document.documentElement.setAttribute("data-theme", themeValue);
    }
  }

  function setTheme(themeValue: "light" | "dark" | "system") {
    settings.value.theme = themeValue;
    applyTheme();
    saveSettings({ theme: themeValue });
  }

  function setLanguage(lang: string) {
    settings.value.language = lang;
    saveSettings({ language: lang });
  }

  async function saveGithubToken(token: string | undefined) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      githubToken.value = token || "";
      await invoke("update_github_token", { token });
    } catch (error) {
      console.error("Failed to save GitHub token:", error);
    }
  }

  return {
    settings,
    configPath,
    githubToken,
    theme,
    language,
    loadSettings,
    saveSettings,
    applyTheme,
    setTheme,
    setLanguage,
    saveGithubToken,
  };
});
