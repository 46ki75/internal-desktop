import { defineStore } from "pinia";
import { load } from "@tauri-apps/plugin-store";

export const useConfigStore = defineStore("config", {
  state() {
    return {
      loading: false,
    };
  },
  actions: {
    async set({ key, value }: { key: string; value: string }) {
      this.loading = true;
      const store = await load("store.json");
      await store.set(key, value);
      await store.save();
      await store.close();
      this.loading = false;
    },

    async get(key: string) {
      this.loading = true;
      const store = await load("store.json");
      const resultValue = await store.get(key);
      await store.close();
      this.loading = false;
      return resultValue;
    },
  },
});
