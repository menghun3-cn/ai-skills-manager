export interface Settings {
  editor: "monaco" | "system";
  autoSync: boolean;
  cleanupOnDisable: boolean;
  syncNotifications: boolean;
  refreshInterval: number;
  theme: "light" | "dark" | "system";
  fontFamily: string;
  fontSize: number;
  language: string;
  githubToken?: string;
  enabledDataSources?: string[];
}

export interface SkillBinding {
  global: boolean;
  projects: string[];
}

export interface Skill {
  name: string;
  description?: string;
  version?: string;
  author?: string;
  bindings: SkillBinding;
  enabledTools: string[];
  enabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface Tool {
  id: string;
  name: string;
  enabled: boolean;
  detected: boolean;
  configPath: string;
  skillsPath: string;
  custom: boolean;
  enabledTools?: string[];
}

export interface MarketItem {
  id: string;
  name: string;
  description?: string;
  author?: string;
  source: "awesome-claude-skills" | "skills.sh";
  downloads?: number;
  installed?: boolean;
  repo?: string;
  skillId?: string;
}

export interface SyncResult {
  success: boolean;
  skillName: string;
  error?: string;
}

export interface SkillFile {
  name: string;
  path: string;
  size: number;
  modified: string;
}
