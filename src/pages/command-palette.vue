<template>
  <div :class="$style['command-palette']">
    <ElmCommandPalette
      :commands="commands"
      :on-command-invoked="closeCommandPalette"
    />
  </div>
</template>

<script setup lang="ts">
import { ElmCommandPalette } from "@elmethis/command-palette";
import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";
import { useWindowFocus } from "@vueuse/core";
import { watch } from "vue";

export interface CommandPaletteProps {}

withDefaults(defineProps<CommandPaletteProps>(), {});

const closeCommandPalette = async () => {
  await invoke("close_command_palette");
};

const wundowFocus = useWindowFocus();

watch(wundowFocus, async (f) => {
  if (!f) await invoke("close_command_palette");
});

const openUrl = (url: string) => () => {
  openPath(url);
};

const commands = [
  {
    id: "7e3a53b9-c486-4b67-8216-5686517b99b7",
    label: "GitHub",
    description: "https://github.com",
    icon: "https://github.githubassets.com/favicons/favicon.svg",
    onInvoke: openUrl("https://github.com"),
  },
  {
    id: "0e02fda3-0460-4bd5-839f-9f0d251ce83e",
    label: "VueUse",
    icon: "https://vueuse.org/favicon.svg",
    onInvoke: openUrl("https://vueuse.org"),
  },
  {
    id: "3589e1b1-ddc5-4f7e-97cd-79c1d76df288",
    label: "Feedly",
    icon: "https://feedly.com/feedly-32.png",
    onInvoke: openUrl("https://feedly.com"),
  },
  {
    id: "2209085f-9e63-49bc-abd5-8659c7261814",
    label: "GitLab",
    description: "https://about.gitlab.com",
    icon: "https://about.gitlab.com/images/ico/favicon.ico",
    onInvoke: openUrl("https://about.gitlab.com/"),
  },
  {
    id: "4d74f753-4727-46cd-9e0f-a9a6d7a6d1fc",
    label: "Fuse.js",
    description: "https://www.fusejs.io",
    icon: "https://www.fusejs.io/icons/favicon-32x32.png",
    onInvoke: openUrl("https://www.fusejs.io"),
  },
];
</script>

<style module lang="scss">
.command-palette {
  width: 100%;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
