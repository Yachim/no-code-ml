import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query";
import type { NetList } from "../types/savedData";

import { exists, readTextFile, BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";

// params for readTextFile/exists
const netsFileReadSettings = ["nets.json", { dir: BaseDirectory.AppData }] as const;
const netsFileWriteSettings = (content: string) =>
	[netsFileReadSettings[0], content, netsFileReadSettings[1]] as const;

// loads the data file containing list of networks
export function useNets() {
	return useQuery<NetList>("netList", async () => {
		if (await exists(...netsFileReadSettings)) {
			const file = await readTextFile(...netsFileReadSettings);
			return JSON.parse(file);
		}
		return [];
	})
}

// creates a new network and returns info about it
export function useCreateNet() {
	const client = useQueryClient();

	return useMutation(async () => {
		let data: NetList = (await exists(...netsFileReadSettings)) ?
			JSON.parse(await readTextFile(...netsFileReadSettings)) :
			[];

		const newNet = {
			name: "Unnamed network",
			id: crypto.randomUUID()
		}

		data = [...data, newNet];

		await writeTextFile(...netsFileWriteSettings(JSON.stringify(data)))

		return newNet;
	}, {
		onSuccess: () => client.invalidateQueries("netList")
	})
}
