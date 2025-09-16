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
  </div>
</template>

<script setup lang="ts">
import { ElmButton, ElmHeading, ElmTextField } from "@elmethis/core";
import { useConfigStore } from "../store/configStore";
import { onMounted, ref } from "vue";

export interface ConfigProps {}

withDefaults(defineProps<ConfigProps>(), {});

const configStore = useConfigStore();
const notionApiKey = ref<string>();
const notionBookmarkDataSourceId = ref<string>();

onMounted(async () => {
  notionApiKey.value = (await configStore.get("notionApiKey")) as string;
  notionBookmarkDataSourceId.value = (await configStore.get(
    "notionBookmarkDataSourceId"
  )) as string;
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
</script>

<style module lang="scss">
.container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
