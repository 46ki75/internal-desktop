<template>
  <div :class="$style.container">
    <ElmHeading :level="2"> Notion API Key</ElmHeading>
    <ElmTextField
      v-model="notionApiKey"
      label="API Key"
      icon="key"
      is-password
      :loading="configStore.loading"
    />
    <ElmTextField
      v-model="notionBookmarkDataSourceId"
      label="Notion Bookmark Data Source ID"
      icon="link"
      :loading="configStore.loading"
    />

    <ElmButton block @click="handleSet" :loading="configStore.loading">
      Update
    </ElmButton>

    <div :class="$style['switch-container']">
      <ElmMdiIcon :d="mdiButtonPointer" size="1.5rem" />
      <ElmInlineText>Enable Autostart</ElmInlineText>
      <ElmSwitch
        v-model="isAutostartEnabled"
        :disabled="isAutostartEnabledLoading"
        color="#bfa056"
        size="1rem"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  ElmButton,
  ElmHeading,
  ElmInlineText,
  ElmMdiIcon,
  ElmSwitch,
  ElmTextField,
} from "@elmethis/vue";
import { useConfigStore } from "../store/configStore";
import { mdiButtonPointer } from "@mdi/js";
import { onMounted, ref, watch } from "vue";
import { enable, isEnabled, disable } from "@tauri-apps/plugin-autostart";

export interface ConfigProps {}

withDefaults(defineProps<ConfigProps>(), {});

const configStore = useConfigStore();
const notionApiKey = ref<string>();
const notionBookmarkDataSourceId = ref<string>();
const isAutostartEnabledLoading = ref<boolean>(true);
const isAutostartEnabled = ref<boolean>(false);

onMounted(async () => {
  notionApiKey.value = (await configStore.get("notionApiKey")) as string;
  notionBookmarkDataSourceId.value = (await configStore.get(
    "notionBookmarkDataSourceId"
  )) as string;

  // Autostart
  try {
    isAutostartEnabledLoading.value = true;
    isAutostartEnabled.value = await isEnabled();
  } finally {
    isAutostartEnabledLoading.value = false;
  }
});

const handleSet = async () => {
  if (notionApiKey.value) {
    await configStore.set({ key: "notionApiKey", value: notionApiKey.value });
  }
  if (notionBookmarkDataSourceId.value) {
    await configStore.set({
      key: "notionBookmarkDataSourceId",
      value: notionBookmarkDataSourceId.value,
    });
  }
};

watch(isAutostartEnabled, async (isAutostartEnabledValue) => {
  try {
    isAutostartEnabledLoading.value = true;
    if (isAutostartEnabledValue) {
      await enable();
      isAutostartEnabled.value = await isEnabled();
    } else {
      await disable();
      isAutostartEnabled.value = await isEnabled();
    }
  } finally {
    isAutostartEnabledLoading.value = false;
  }
});
</script>

<style module lang="scss">
.container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.switch-container {
  display: grid;
  grid-template-columns: 3rem 1fr 4rem;
  align-items: center;
  user-select: none;
}
</style>
