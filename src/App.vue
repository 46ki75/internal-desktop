<template>
  <RouterView />
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { register } from "@tauri-apps/plugin-global-shortcut";
import { onMounted } from "vue";

onMounted(async () => {
  await register("CommandOrControl+Shift+Space", async () => {
    await invoke("open_command_palette");
  });

  await register("Esc", async () => {
    await invoke("close_command_palette");
  });
});
</script>

<style lang="scss">
html,
body {
  font-family: sans-serif;
  margin: 0;
  padding: 0;
}

#app {
  margin: 0;
  padding: 0;
}
</style>
