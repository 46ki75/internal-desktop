import { ElmCommandPaletteProps } from "@elmethis/command-palette";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";
import { ref, watch } from "vue";

interface Bookmark {
  id: string;
  name?: string;
  url?: string;
  favicon?: string;
  tag: {
    id: String;
    name: string;
    color: String;
  };
  nsfw: boolean;
  favorite: boolean;
  notion_url: String;
}

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
        description: b.url,
        keywords: ["any"],
        icon: b.favicon,
        tag: {
          name: "URL",
          color: "blue",
        },
        onInvoke: b.url ? openUrlWrapper(b.url) : () => {},
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
      this.bookmarkList = await invoke<Bookmark[]>("fetch_bookmark_list");
      this.key = this.key + 1;
    },
  },
});
