<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useSkillsStore } from "@/stores/skills";
import { X, Loader2 } from "lucide-vue-next";

const emit = defineEmits<{
  close: [];
  created: [];
}>();

const { t } = useI18n();
const skillsStore = useSkillsStore();

const name = ref("");
const description = ref("");
const loading = ref(false);

async function handleCreate() {
  if (!name.value.trim()) return;

  loading.value = true;
  try {
    await skillsStore.createSkill(name.value.trim(), description.value.trim());
    emit("created");
  } catch (error) {
    console.error("Failed to create skill:", error);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
    <div class="w-full max-w-md rounded-xl bg-[var(--color-surface-elevated)] p-6 shadow-xl">
      <h2 class="mb-4 text-lg font-semibold text-[var(--color-text-primary)]">
        {{ t("skills.createSkill") }}
      </h2>

      <div class="space-y-4">
        <div>
          <label class="mb-1 block text-sm font-medium text-[var(--color-text-primary)]">
            {{ t("skills.skillName") }}
          </label>
          <input
            v-model="name"
            type="text"
            :placeholder="t('skills.skillNamePlaceholder')"
            class="input"
          />
        </div>

        <div>
          <label class="mb-1 block text-sm font-medium text-[var(--color-text-primary)]">
            {{ t("skills.skillDescription") }}
          </label>
          <textarea
            v-model="description"
            :placeholder="t('skills.skillDescriptionPlaceholder')"
            rows="3"
            class="input resize-none"
          ></textarea>
        </div>
      </div>

      <div class="mt-6 flex justify-end gap-3">
        <button
          @click="emit('close')"
          class="btn btn-secondary"
          :disabled="loading"
        >
          <X class="h-4 w-4" :stroke-width="1.5" />
          {{ t("common.cancel") }}
        </button>
        <button
          @click="handleCreate"
          :disabled="!name.trim() || loading"
          class="btn btn-primary"
        >
          <Loader2 v-if="loading" class="h-4 w-4 animate-spin" :stroke-width="1.5" />
          {{ loading ? t("common.loading") : t("skills.create") }}
        </button>
      </div>
    </div>
  </div>
</template>
