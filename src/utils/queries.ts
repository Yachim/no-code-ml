import { useQuery } from "@sveltestack/svelte-query";
import type { NetList } from "../types/savedData";

import { exists, readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

// loads the data file containing list of networks
function useNetList() {
	return useQuery<NetList>("nets", async () => {
		const fileSettings = ["nets.json", { dir: BaseDirectory.AppData }] as const;

		if (await exists(...fileSettings)) {
			const file = await readTextFile(...fileSettings)
			return JSON.parse(file)
		}
		return []
	})
}
