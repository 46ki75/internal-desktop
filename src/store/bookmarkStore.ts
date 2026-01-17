import { ElmCommandPaletteProps } from "@elmethis/vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";
import { ref, watch } from "vue";
import { openApiClient } from "../openapi/client";
import { useAuthStore } from "./authStore";
import { paths } from "../openapi/schema";

type Bookmark =
  paths["/api/v1/bookmark"]["get"]["responses"]["200"]["content"]["application/json"][number];

const openUrlWrapper = (url: string) => () => {
  openUrl(url);
};

export const useBookmarkStore = defineStore("bookmark", {
  state() {
    const bookmarkList = useLocalStorage<Bookmark[]>("bookmarkList", []);
    const commands = ref<ElmCommandPaletteProps["commands"]>([]);

    watch(bookmarkList, (bookmarkList) => {
      commands.value = bookmarkList.map((b) => ({
        id: b.id,
        label: b.name || "No Title",
        description: b.url ?? undefined,
        keywords: ["any"],
        icon: b.favicon ?? undefined,
        tag: {
          name: "URL",
          color: "blue",
        },
        onInvoke: b.url ? openUrlWrapper(b.url) : undefined,
      }));
    });

    return {
      key: 0,
      commands,
      bookmarkList,
    };
  },

  actions: {
    async fetchBookmarkList() {
      // this.bookmarkList = await invoke<Bookmark[]>("fetch_bookmark_list");

      try {
        const authStore = useAuthStore();
        await authStore.refreshAccessToken();
        const accessToken = authStore.accessToken;

        if (!accessToken) {
          throw new Error("No access token available.");
        }

        const response = await openApiClient.GET("/api/v1/bookmark", {
          params: { header: { Authorization: `Bearer ${accessToken}` } },
        });

        if (response.error) {
          throw new Error(response.error);
        }

        const bookmarks = response.data;

        if (!Array.isArray(bookmarks)) {
          throw new Error("Invalid bookmark data received.");
        }

        this.bookmarkList = bookmarks;

        this.key = this.key + 1;
      } catch (error) {
        console.error(error);
      }
    },
  },
});
