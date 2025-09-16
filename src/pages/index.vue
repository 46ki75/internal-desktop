<template>
  <div>
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
import Click from "../components/Click.vue";
import { ElmInlineText } from "@elmethis/core";
</script>

<style module lang="scss"></style>
