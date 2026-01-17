import createClient from "openapi-fetch";
import type { paths } from "./schema";

import { fetch } from "@tauri-apps/plugin-http";

export const openApiClient = createClient<paths>({
  fetch: fetch.bind(fetch),
});
