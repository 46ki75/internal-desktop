<template>
  <div :class="$style['command-palette']">
    <ElmCommandPalette
      v-if="windowFocus"
      :key="bookmarkStore.key"
      :commands="bookmarkStore.commands"
      :on-command-invoked="closeCommandPalette"
    />
  </div>
</template>

<script setup lang="ts">
import { ElmCommandPalette } from "@elmethis/command-palette";
import { invoke } from "@tauri-apps/api/core";
import { useWindowFocus } from "@vueuse/core";
import { watch } from "vue";
import { useBookmarkStore } from "../store/bookmarkStore";

export interface CommandPaletteProps {}

withDefaults(defineProps<CommandPaletteProps>(), {});

const bookmarkStore = useBookmarkStore();

const closeCommandPalette = async () => {
  await invoke("close_command_palette");
};

const windowFocus = useWindowFocus();

watch(windowFocus, async (f) => {
  if (f) {
    // mounted
  } else {
    // unmounted
    await invoke("close_command_palette");
  }
});
</script>

<style module lang="scss">
.command-palette {
  width: 100%;
  height: 100vh;
  display: flex;
  justify-content: center;
  opacity: 0.95;
}
</style>
