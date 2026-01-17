import createClient from "openapi-fetch";
import type { paths } from "./schema";

import { fetch } from "@tauri-apps/plugin-http";

export const openApiClient = createClient<paths>({
  baseUrl: "https://internal.ikuma.cloud",
  fetch: fetch.bind(fetch),
});
