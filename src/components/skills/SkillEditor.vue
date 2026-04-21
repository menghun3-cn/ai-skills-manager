<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import * as monaco from "monaco-editor";

const props = defineProps<{
  skillName: string;
}>();

const emit = defineEmits<{
  close: [];
}>();

const { t } = useI18n();

const editorContainer = ref<HTMLElement | null>(null);
let editor: monaco.editor.IStandaloneCodeEditor | null = null;
const files = ref<{ name: string; path: string }[]>([]);
const currentFile = ref<string | null>(null);
const fileContent = ref("");

async function loadSkillFiles() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const result = await invoke<{ name: string; path: string }[]>("get_skill_files", { skillName: props.skillName });
    files.value = result;
    if (result.length > 0) {
      await selectFile(result[0].path);
    }
  } catch (error) {
    console.error("Failed to load skill files:", error);
  }
}

async function selectFile(path: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const content = await invoke<string>("read_skill_file", { path });
    currentFile.value = path;
    fileContent.value = content;
    if (editor) {
      editor.setValue(content);
    }
  } catch (error) {
    console.error("Failed to read file:", error);
  }
}

async function saveFile() {
  if (!currentFile.value || !editor) return;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("write_skill_file", {
      path: currentFile.value,
      content: editor.getValue(),
    });
  } catch (error) {
    console.error("Failed to save file:", error);
  }
}

function handleClose() {
  emit("close");
}

onMounted(async () => {
  await loadSkillFiles();

  if (editorContainer.value) {
    editor = monaco.editor.create(editorContainer.value, {
      value: fileContent.value,
      language: "typescript",
      theme: document.documentElement.getAttribute("data-theme") === "dark" ? "vs-dark" : "vs",
      automaticLayout: true,
      minimap: { enabled: true },
      fontSize: 13,
      lineNumbers: "on",
      roundedSelection: true,
      scrollBeyondLastLine: false,
      wordWrap: "on",
    });

    editor.onDidChangeModelContent(() => {
      fileContent.value = editor?.getValue() || "";
    });
  }
});

onUnmounted(() => {
  editor?.dispose();
});

watch(fileContent, (newContent) => {
  if (editor && editor.getValue() !== newContent) {
    editor.setValue(newContent);
  }
});
</script>

<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="flex h-[80vh] w-[90vw] flex-col rounded-xl bg-[var(--color-background)] shadow-xl">
      <header class="flex items-center justify-between border-b border-[var(--color-border)] px-6 py-4">
        <h2 class="text-lg font-semibold text-[var(--color-text-primary)]">
          {{ skillName }}
        </h2>
        <div class="flex items-center gap-2">
          <button
            @click="saveFile"
            class="rounded-lg bg-[var(--color-primary)] px-4 py-2 text-sm font-medium text-white hover:bg-[var(--color-primary-hover)]"
          >
            {{ t("common.save") }}
          </button>
          <button
            @click="handleClose"
            class="rounded-lg border border-[var(--color-border)] px-4 py-2 text-sm text-[var(--color-text-secondary)] hover:bg-[var(--color-surface)]"
          >
            {{ t("common.close") }}
          </button>
        </div>
      </header>

      <div class="flex flex-1 overflow-hidden">
        <aside class="w-48 border-r border-[var(--color-border)] overflow-y-auto p-2">
          <button
            v-for="file in files"
            :key="file.path"
            @click="selectFile(file.path)"
            class="mb-1 w-full rounded-lg px-3 py-2 text-left text-sm"
            :class="
              currentFile === file.path
                ? 'bg-[var(--color-primary)] text-white'
                : 'text-[var(--color-text-secondary)] hover:bg-[var(--color-surface)]'
            "
          >
            {{ file.name }}
          </button>
        </aside>

        <div ref="editorContainer" class="flex-1"></div>
      </div>
    </div>
  </div>
</template>
