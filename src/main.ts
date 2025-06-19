import { createApp } from "vue";
import App from "./App.vue";

import { createWebHistory, createRouter } from "vue-router";

import HomeView from "./pages/index.vue";
import CommandPalette from "./pages/command-palette.vue";

const routes = [
  { path: "/", component: HomeView },
  { path: "/command-palette", component: CommandPalette },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const app = createApp(App);
app.use(router);
app.mount("#app");
