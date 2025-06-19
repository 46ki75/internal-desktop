<template>
  <main>
    <button @click="open">New Window</button>
    <button @click="close">Close Window</button>
  </main>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const open = async () => {
  console.log("clicked");
  await invoke("open_command_palette");
};

const close = async () => {
  const result = await invoke("close_command_palette");
  console.log(result);
};
</script>
