<template>
  <div></div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useBookmarkStore } from "../store/bookmarkStore";

export interface CronManagerProps {}

withDefaults(defineProps<CronManagerProps>(), {});

const bookmarkStore = useBookmarkStore();

const fechBookmarkId = ref<number>();

onMounted(async () => {
  await bookmarkStore.fetchBookmarkList();
  fechBookmarkId.value = window.setInterval(async () => {
    await bookmarkStore.fetchBookmarkList();
  }, 30 * 60 * 1000);
});

onUnmounted(() => {
  if (fechBookmarkId.value) {
    window.clearInterval(fechBookmarkId.value);
  }
});
</script>

<style module lang="scss"></style>
