import { useMutation, useQuery, useQueryClient } from "@sveltestack/svelte-query";
import type { Net, NetList, NetListMember } from "../types/savedData";

import { exists, readTextFile, BaseDirectory, writeTextFile, createDir, removeFile, writeBinaryFile, readBinaryFile } from "@tauri-apps/api/fs";
import type { NetworkModelType } from "../types/network";
import { currentNetId } from "./stores";

if (!await exists("nets", { dir: BaseDirectory.AppData }))
	await createDir("nets", { dir: BaseDirectory.AppData });

if (!await exists("trained_params", { dir: BaseDirectory.AppData }))
	await createDir("trained_params", { dir: BaseDirectory.AppData });

if (!await exists("datasets", { dir: BaseDirectory.AppData }))
	await createDir("datasets", { dir: BaseDirectory.AppData });

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

// deletes $APPDATA/nets/net_{netId}.json
async function deleteNetFile(id: string) {
	await removeFile(`nets/net_${id}.json`, { dir: BaseDirectory.AppData });
}

// writes to $APPDATA/datasets/{type}_{net.id}.{format}
async function writeTrainingDatasetFile(file: File, type: "training" | "testing", netId: string, format: "csv") {
	await writeBinaryFile(
		`datasets/${type}_${netId}.${format}`,
		await file.arrayBuffer(),
		{ dir: BaseDirectory.AppData }
	)
}

// reads $APPDATA/datasets/{type}_{net.id}.{format}
async function readTrainingDatasetFile(type: "training" | "testing", netId: string, format: "csv"): Promise<File> {
	const fileName = `${type}_${netId}.${format}`;

	const fileData = await readBinaryFile(
		`datasets/${fileName}`,
		{ dir: BaseDirectory.AppData }
	);
	return new File([fileData], fileName);
}

const universalDefaultNetwork = {
	name: "Unnamed network",
	trainingFileSaved: false,
	initialSetting: true
}

export const defaultNetworks: {
	[key in NetworkModelType]: Omit<Net, "id">
} = {
	"multilayerPerceptron": {
		...universalDefaultNetwork,

		modelType: "multilayerPerceptron",

		hiddenLayersCnt: 2,
		networkType: "regression",

		inputNormalizationFunc: "normalization",
		inputNeuronCnt: 10,

		hiddenLayersSettings: [],

		outputNeuronCnt: 1,
		outputActivationFunc: "sigmoid",
		outputNeuronLabels: [],

		costFunc: "mse",
		iterationCnt: 10_000,

		outputCol: -1,
		includedCols: [],
	}
};

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

		const defaultNetwork = defaultNetworks[modelType];

		const newNetListMember: NetListMember = {
			name: defaultNetwork.name,
			modelType,
			id,
		}

		data = [...data, newNetListMember];

		const newNet: Net = {
			...defaultNetwork,
			id,
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

	const query = useQuery(["net", selectedNetId], async () => {
		const net = await readNetFile(selectedNetId);

		// FIXME: the app crashes
		//const trainingFile = net.trainingFileSaved ? await readTrainingDatasetFile("training", net.id, "csv") : undefined;

		return {
			...net,
			//trainingFile
		};
	}, {
		enabled: false
	});

	currentNetId.subscribe((value) => {
		selectedNetId = value;

		query.updateOptions({
			queryKey: ["net", value],
			enabled: value !== ""
		});
	})

	return query;
}

export function useSaveNet() {
	const client = useQueryClient();

	// TODO: omit values from the net parameter that cannot be modified after initial setting (e.g. hiddenLayersCnt)
	return useMutation(async (
		net: Omit<Net, "initialSetting" | "name" | "trainingFileSaved"> & {
			trainingFile: File | undefined
		}
	) => {
		const { name } = await readNetFile(net.id);

		// FIXME: the app slows down
		if (net.trainingFile) {
			await writeTrainingDatasetFile(net.trainingFile, "training", net.id, "csv");
		}

		await writeNetFile({
			name,
			...net,
			initialSetting: false,
			trainingFileSaved: !!net.trainingFile
		});
	}, {
		onSuccess: () => {
			client.invalidateQueries("net");
		}
	})
}

// FIXME: after deleting isNotSaved is still true
export function useDeleteNet() {
	const client = useQueryClient();

	return useMutation(async (id: string) => {
		const netList = await readNetsFile();
		const netIndex = netList.findIndex((net) => net.id === id);
		netList.splice(netIndex, 1);
		await writeNetsFile(netList);

		deleteNetFile(id);
		// TODO: delete $APPDATA/trained_params/net_{id}.json

		return netList;
	}, {
		onSuccess: () => {
			currentNetId.set("");
			client.invalidateQueries("netList");
			client.invalidateQueries("net");
		}
	})
}
