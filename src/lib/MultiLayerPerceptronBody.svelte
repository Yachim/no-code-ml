<script lang="ts">
	import { fa0 } from "@fortawesome/free-solid-svg-icons";

	import type {
		ActivationFunc,
		CostFunc,
		HiddenLayers,
		NetworkType,
		NormalizationFunc,
	} from "../types/network";

	import Controls from "./components/reusable/Controls.svelte";
	import DataReader from "./components/reusable/DataReader.svelte";
	import HiddenLayersSettings from "./components/reusable/HiddenLayersSettings.svelte";
	import InputSettings from "./components/reusable/InputSettings.svelte";
	import NetworkSettings from "./components/reusable/NetworkSettings.svelte";
	import OutputSettings from "./components/reusable/OutputSettings.svelte";

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

	let outputCol: number;
	let includedCols: number[];

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
		<DataReader bind:outputCol bind:includedCols />
	</div>
</div>
