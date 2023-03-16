import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query";
import type { Net, NetList } from "../types/savedData";

import { exists, readTextFile, BaseDirectory, writeTextFile, createDir } from "@tauri-apps/api/fs";
import type { NetworkModelType } from "../types/network";
import { currentNetId } from "./stores";

if (!await exists("nets", { dir: BaseDirectory.AppData }))
	await createDir("nets", { dir: BaseDirectory.AppData });

if (!await exists("trained_params", { dir: BaseDirectory.AppData }))
	await createDir("trained_params", { dir: BaseDirectory.AppData });

// loads the data file containing list of networks
export function useNets() {
	return useQuery<NetList>("netList", async () => {
		if (await exists("nets.json", { dir: BaseDirectory.AppData })) {
			const file = await readTextFile("nets.json", { dir: BaseDirectory.AppData });
			return JSON.parse(file);
		}
		return [];
	})
}

// creates a new network and returns info about it
export function useCreateNet() {
	const client = useQueryClient();

	return useMutation(async (modelType: NetworkModelType) => {
		let data: NetList = (await exists("nets.json", { dir: BaseDirectory.AppData })) ?
			JSON.parse(await readTextFile("nets.json", { dir: BaseDirectory.AppData })) :
			[];

		let id = crypto.randomUUID();

		// because I am scared
		while (data.findIndex((net) => net.id === id) !== -1) {
			id = crypto.randomUUID();
			console.log("1 in 2.71 quintillion moment happened.")
		}

		const newNet = {
			name: "Unnamed network",
			id,
			modelType
		}

		data = [...data, newNet];

		await writeTextFile(`nets/net_${newNet.id}.json`, JSON.stringify(newNet), { dir: BaseDirectory.AppData });
		await writeTextFile(`trained_params/net_${newNet.id}.json`, JSON.stringify({ id: newNet.id }), { dir: BaseDirectory.AppData });
		await writeTextFile("nets.json", JSON.stringify(data), { dir: BaseDirectory.AppData });

		return newNet;
	}, {
		onSuccess: () => client.invalidateQueries("netList")
	})
}

export function useRenameNet() {
	const client = useQueryClient();

	return useMutation(async (data: { name: string, id: string }) => {
		const netList: NetList = JSON.parse(await readTextFile("nets.json", { dir: BaseDirectory.AppData }));

		const netIndex = netList.findIndex((net) => net.id === data.id);
		netList[netIndex].name = data.name;

		await writeTextFile("nets.json", JSON.stringify(netList), { dir: BaseDirectory.AppData });

		const net: Net = JSON.parse(await readTextFile(`nets/net_${data.id}.json`, { dir: BaseDirectory.AppData }));

		net.name = data.name;

		await writeTextFile(`nets/net_${data.id}.json`, JSON.stringify(net), { dir: BaseDirectory.AppData });
	}, {
		onSuccess: () => {
			client.invalidateQueries("netList")
			client.invalidateQueries("net")
		}
	})
}

export function useNet() {
	let selectedNetId = "";

	const queryOptions = {
		queryKey: ["net", selectedNetId],
		queryFn: async () => {
			const net: Net = JSON.parse(await readTextFile(`nets/net_${selectedNetId}.json`, { dir: BaseDirectory.AppData }));

			return net;
		},
		enabled: selectedNetId !== ""
	}

	const query = useQuery(queryOptions);

	currentNetId.subscribe((value) => {
		selectedNetId = value;
		query.setOptions({
			...queryOptions,
			queryKey: ["net", value],
			enabled: value !== ""
		});
	})

	return query;
}
