import { createApp } from "vue";
import App from "./App.vue";
import { createWebHistory, createRouter } from "vue-router";

import HomeView from "./pages/index.vue";
import CommandPalette from "./pages/command-palette.vue";
import Config from "./pages/config.vue";
import { createPinia } from "pinia";

const routes = [
  { path: "/", component: HomeView },
  { path: "/command-palette", component: CommandPalette },
  { path: "/config", component: Config },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const pinia = createPinia();

const app = createApp(App);
app.use(router);
app.use(pinia);
app.mount("#app");
