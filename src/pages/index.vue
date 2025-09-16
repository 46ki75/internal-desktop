<template>
  <div>
    <button @click="open">Open Command Palette Window</button>
    <button @click="close">Close Command Palette Window</button>
    <Click />

    <div>
      <ElmInlineText text="Launch command palette: " />
      <ElmInlineText text="Ctrl" kbd />
      <ElmInlineText text=" + " />
      <ElmInlineText text="Shift" kbd />
      <ElmInlineText text=" + " />
      <ElmInlineText text="Space" kbd />
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Click from "../components/Click.vue";
import { ElmInlineText } from "@elmethis/core";

const open = async () => {
  console.log("clicked");
  await invoke("open_command_palette");
};

const close = async () => {
  const result = await invoke("close_command_palette");
  console.log(result);
};
</script>

<style module lang="scss"></style>
