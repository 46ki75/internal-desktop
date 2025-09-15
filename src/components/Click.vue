<template>
  <div :class="$style.box">
    <ElmProgress weight="4px" :max="10" :value="progress" />
    <ElmButton block @click="enabled = !enabled">
      <ElmMdiIcon
        :d="enabled ? mdiLightningBolt : mdiSleep"
        :color="enabled ? '#59b57c' : '#c56565'"
        size="1.25rem"
      />
      <ElmInlineText
        :text="enabled ? 'Active' : 'Idle (Click to Activate)'"
        :color="enabled ? '#59b57c' : '#c56565'"
      />
    </ElmButton>
  </div>
</template>

<script setup lang="ts">
import {
  ElmButton,
  ElmProgress,
  ElmInlineText,
  ElmMdiIcon,
} from "@elmethis/core";
import { invoke } from "@tauri-apps/api/core";
import { onUnmounted, ref, watch } from "vue";
import { mdiLightningBolt, mdiSleep } from "@mdi/js";

export interface ClickProps {}

withDefaults(defineProps<ClickProps>(), {});

const enabled = ref<boolean>(false);
const progress = ref<number>(0);
const id = ref<number | null>(null);

watch(enabled, (v) => {
  if (v) {
    id.value = window.setInterval(async () => {
      progress.value = progress.value + 1;
      if (progress.value >= 10) {
        progress.value = 0;
        await invoke("click");
      }
    }, 1000);
  } else {
    if (id.value) clearInterval(id.value);
    progress.value = 0;
  }
});

onUnmounted(() => {
  if (id.value) clearInterval(id.value);
});
</script>

<style module lang="scss">
.box {
  display: grid;
  grid-template-rows: 1fr 2fr;
  margin-block: 1rem;
}
</style>
