import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query";
import type { Net, NetList, NetListMember } from "../types/savedData";

import { exists, readTextFile, BaseDirectory, writeTextFile, createDir } from "@tauri-apps/api/fs";
import type { NetworkModelType } from "../types/network";
import { currentNetId } from "./stores";

if (!await exists("nets", { dir: BaseDirectory.AppData }))
	await createDir("nets", { dir: BaseDirectory.AppData });

if (!await exists("trained_params", { dir: BaseDirectory.AppData }))
	await createDir("trained_params", { dir: BaseDirectory.AppData });

// reads $APPDATA/nets.json
async function readNetsFile(): Promise<NetList> {
	const data: NetList = (await exists("nets.json", { dir: BaseDirectory.AppData })) ?
		JSON.parse(await readTextFile("nets.json", { dir: BaseDirectory.AppData })) :
		[];

	return data
}

// writes to $APPDATA/nets.json
async function writeNetsFile(data: NetList) {
	await writeTextFile("nets.json", JSON.stringify(data), { dir: BaseDirectory.AppData });
}

// reads $APPDATA/nets/net_{netId}.json
async function readNetFile(netId: string): Promise<Net> {
	const net: Net = JSON.parse(await readTextFile(`nets/net_${netId}.json`, { dir: BaseDirectory.AppData }));
	return net
}

// writes to $APPDATA/nets/net_{netId}.json
async function writeNetFile(data: Net) {
	await writeTextFile(`nets/net_${data.id}.json`, JSON.stringify(data), { dir: BaseDirectory.AppData });
}

// loads the data file containing list of networks
export function useNets() {
	return useQuery<NetList>("netList", async () => {
		const data = await readNetsFile();
		return data;
	})
}

// creates a new network and returns info about it
export function useCreateNet() {
	const client = useQueryClient();

	return useMutation(async (modelType: NetworkModelType) => {
		let data = await readNetsFile();

		let id = crypto.randomUUID();

		// because I am scared
		while (data.findIndex((net) => net.id === id) !== -1) {
			id = crypto.randomUUID();
			console.log("1 in 2.71 quintillion moment happened.")
		}

		const newNetListMember: NetListMember = {
			name: "Unnamed network",
			id,
			modelType
		}

		data = [...data, newNetListMember];

		const newNet: Net = {
			...newNetListMember,
			initialSetting: true
		}

		await writeNetFile(newNet);
		await writeTextFile(`trained_params/net_${newNet.id}.json`, JSON.stringify({ id: newNet.id }), { dir: BaseDirectory.AppData });
		await writeNetsFile(data);

		return newNet;
	}, {
		onSuccess: () => client.invalidateQueries("netList")
	})
}

export function useRenameNet() {
	const client = useQueryClient();

	return useMutation(async (data: { name: string, id: string }) => {
		const netList = await readNetsFile();

		const netIndex = netList.findIndex((net) => net.id === data.id);
		netList[netIndex].name = data.name;

		await writeNetsFile(netList);

		const net = await readNetFile(data.id);

		net.name = data.name;

		await writeNetFile(net);
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
			const net = await readNetFile(selectedNetId);

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
