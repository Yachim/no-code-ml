<script lang="ts">
	import type {
		ActivationFunc,
		CostFunc,
		HiddenLayers,
		NetworkType,
		NormalizationFunc,
	} from "../types/network";

	import Controls from "./components/Controls.svelte";
	import DataReader from "./components/DataReader.svelte";
	import HiddenLayersSettings from "./components/HiddenLayersSettings.svelte";
	import InputSettings from "./components/InputSettings.svelte";
	import NetworkSettings from "./components//NetworkSettings.svelte";
	import OutputSettings from "./components/OutputSettings.svelte";
	import { saveFunc } from "../utils/stores";

	import { writeTextFile, BaseDirectory } from "@tauri-apps/api/fs";

	let hiddenLayersCnt = 2;
	let networkType: NetworkType = "regression";

	let inputNormalizationFunc: NormalizationFunc = "normalization";
	let inputNeuronCnt = 10;

	let hiddenLayersSettings: HiddenLayers = [];

	let outputNeuronCnt = 10;
	let outputActivationFunc: ActivationFunc = "sigmoid";
	let outputNeuronLabels: string[];

	let costFunc: CostFunc = "mse";
	let iterationCnt = 10_000;

	let files: FileList;
	let outputCol: number;
	let includedCols: number[];

	saveFunc.set(() => {
		/*writeTextFile(
			"nets.json",
			JSON.stringify({
				name: "",
				type: "multilayer-perceptron",

				hiddenLayersCnt,
				networkType,

				inputNormalizationFunc,
				inputNeuronCnt,

				hiddenLayersSettings,

				outputNeuronCnt,
				outputActivationFunc,
				outputNeuronLabels,

				costFunc,
				iterationCnt,

				// TODO: file
				outputCol,
				includedCols,
			}),
			{
				dir: BaseDirectory.AppData,
			}
		);*/
		console.log("saved");
	});

	let buttonDisabled = true;
	$: {
		buttonDisabled = false;

		if (!networkType) buttonDisabled = true;
		if (!inputNormalizationFunc) buttonDisabled = true;
		if (!inputNeuronCnt || inputNeuronCnt <= 0) buttonDisabled = true;
		if (hiddenLayersCnt > 0) {
			if (
				!hiddenLayersSettings.every(
					(layer) => layer.activationFunc && layer.neuronCnt > 0
				)
			)
				buttonDisabled = true;
		}
		if (!outputNeuronCnt || outputNeuronCnt <= 0) buttonDisabled = true;
		if (!outputActivationFunc) buttonDisabled = true;
		if (!costFunc) buttonDisabled = true;
		if (!iterationCnt || iterationCnt <= 0) buttonDisabled = true;
		if (!files) buttonDisabled = true;
		if (outputCol === undefined || outputCol === null)
			buttonDisabled = true;
		if (!includedCols || includedCols.length <= 0) buttonDisabled = true;
	}
</script>

<div class="w-full h-full flex overflow-hidden">
	<div class="border-r-border border-r p-4 flex flex-col overflow-auto gap-4">
		<p>
			<span class="text-red-600">*</span> cannot be changed after the first
			save
		</p>
		<NetworkSettings bind:hiddenLayersCnt bind:type={networkType} />
		<InputSettings
			bind:normalizationFunc={inputNormalizationFunc}
			bind:neuronCnt={inputNeuronCnt}
		/>
		{#if hiddenLayersCnt > 0}
			<HiddenLayersSettings
				bind:hiddenLayersCnt
				bind:hiddenLayersSettings
			/>
		{/if}
		<OutputSettings
			bind:neuronCnt={outputNeuronCnt}
			bind:activationFunc={outputActivationFunc}
			bind:neuronLabels={outputNeuronLabels}
		/>
	</div>

	<div class="flex flex-col w-full p-4 gap-4">
		<Controls bind:costFunc bind:iterationCnt {buttonDisabled} />
		<DataReader bind:outputCol bind:includedCols bind:files />
	</div>
</div>
