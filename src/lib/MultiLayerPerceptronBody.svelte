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

	let costFunc: CostFunc = "mse";
	let iterationCnt = 10_000;
</script>

<div class="w-full h-full flex">
	<div class="border-r-border border-r p-4 flex flex-col overflow-auto gap-4">
		<p>* cannot be changed after the first save</p>
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
		/>
	</div>

	<div class="flex flex-col w-full p-4 gap-4">
		<Controls bind:costFunc bind:iterationCnt />
		<DataReader />
	</div>
</div>
