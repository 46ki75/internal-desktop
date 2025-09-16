<template>
  <CronManager />
  <Header v-if="route.path !== '/command-palette'" />
  <div :class="$style.routes">
    <router-view v-slot="{ Component, route }">
      <transition name="fade" mode="out-in">
        <component :is="Component" :key="route.path" />
      </transition>
    </router-view>
  </div>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import { invoke } from "@tauri-apps/api/core";
import { register } from "@tauri-apps/plugin-global-shortcut";
import { onMounted, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useRoute } from "vue-router";
import CronManager from "./components/CronManager.vue";

const route = useRoute();

onMounted(async () => {
  await register("CommandOrControl+Shift+Space", async () => {
    await invoke("open_command_palette");
  });

  await register("Esc", async () => {
    await invoke("close_command_palette");
  });

  getCurrentWindow().onCloseRequested(async (_event) => {
    await getCurrentWindow().hide();
  });
});

watch(route, (route) => {
  if (route.path === "/command-palette") {
    document.body.style.overflowY = "hidden";
  } else {
    document.body.style.overflowY = "auto";
  }
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

.fade-enter-active,
.fade-leave-active {
  transition: opacity 150ms, transform 100ms;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateX(-8px);
}
</style>

<style lang="scss" module>
.routes {
  padding: 1rem;
}
</style>
