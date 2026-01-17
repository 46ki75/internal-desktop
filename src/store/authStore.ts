import { defineStore } from "pinia";
import { useConfigStore } from "./configStore";
import { z } from "zod";

const tokenResponseSchema = z.object({
  access_token: z.string(),
  token_type: z.string(),
  expires_in: z.number(),
});

export const useAuthStore = defineStore("auth", {
  state() {
    return {
      accessToken: null as string | null,
    };
  },
  actions: {
    async refreshAccessToken() {
      const configStore = useConfigStore();

      const clientId = await configStore.get("oauthClientId");
      const clientSecret = await configStore.get("oauthClientSecret");

      if (typeof clientId !== "string" || typeof clientSecret !== "string") {
        throw new Error(
          "OAuth client ID or secret is not configured properly.",
        );
      }

      const params = new URLSearchParams({
        grant_type: "client_credentials",
        client_id: clientId,
        client_secret: clientSecret,
      });

      const response = await fetch(
        "https://ap-northeast-1bmzkezekx.auth.ap-northeast-1.amazoncognito.com/oauth2/token",
        {
          method: "POST",
          headers: { "Content-Type": "application/x-www-form-urlencoded" },
          body: params.toString(),
        },
      );

      const body = await response.json();

      const parsed = tokenResponseSchema.safeParse(body);
      if (!parsed.success) {
        throw new Error("Failed to refresh access token.");
      }

      this.accessToken = parsed.data.access_token;
    },
  },
});
