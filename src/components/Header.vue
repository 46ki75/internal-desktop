<template>
  <header :class="$style.header">
    <router-link
      v-for="definedRoute in definedRoutes"
      :to="definedRoute.to"
      :class="$style['navigation-icon']"
    >
      <ElmMdiIcon
        :d="definedRoute.d"
        size="1.75rem"
        :color="definedRoute.to === route.path ? '#6987b8' : undefined"
      />
    </router-link>
  </header>
</template>

<script setup lang="ts">
import { ElmMdiIcon } from "@elmethis/core";
import { mdiCog, mdiHome } from "@mdi/js";
import { useRoute } from "vue-router";

const route = useRoute();

export interface HeaderProps {}

withDefaults(defineProps<HeaderProps>(), {});

const definedRoutes: Array<{ to: string; d: string }> = [
  {
    to: "/",
    d: mdiHome,
  },
  {
    to: "/config",
    d: mdiCog,
  },
];
</script>

<style module lang="scss">
.header {
  margin: 0;
  padding: 0.5rem;
  background-color: rgba(#e5e6e8, 0.5);
  box-shadow: 0 0 0.25rem rgba(gray, 0.25);
  display: flex;
}

.navigation-icon {
  padding: 0.25rem;
  border-radius: 0.25rem;
  transition: background-color 200ms, opacity 100ms;
  cursor: pointer;

  &:hover {
    background-color: rgba(#788191, 0.1);
  }

  &:active {
    opacity: 0.8;
  }
}
</style>
