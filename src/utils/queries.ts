import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query";
import type { NetList } from "../types/savedData";

import { exists, readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

// params for readTextFile/exists
const netsFileReadSettings = ["nets.json", { dir: BaseDirectory.AppData }] as const;

const queryClient = useQueryClient();

// loads the data file containing list of networks
function useNetList() {
	return useQuery<NetList>("netList", async () => {
		if (await exists(...netsFileReadSettings)) {
			const file = await readTextFile(...netsFileReadSettings);
			return JSON.parse(file);
		}
		return [];
	})
}

function useCreateNet() {
	return useMutation(async (name: string) => {
		let data: NetList = (await exists(...netsFileReadSettings)) ?
			JSON.parse(await readTextFile(...netsFileReadSettings)) :
			[];

		data = [...data, {
			name,
			id: crypto.randomUUID()
		}];

		return data;
	}, {
		onSuccess: () => queryClient.invalidateQueries("netList")
	})
}
