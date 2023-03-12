<script lang="ts">
	import type {
		ActivationFunction,
		HiddenLayers,
		NetworkType,
		Normalization,
	} from "../types/network";

	import Controls from "./components/Controls.svelte";
	import HiddenLayersSettings from "./components/reusable/HiddenLayersSettings.svelte";
	import InputSettings from "./components/reusable/InputSettings.svelte";
	import NetworkSettings from "./components/reusable/NetworkSettings.svelte";
	import OutputSettings from "./components/reusable/OutputSettings.svelte";

	let hiddenLayersCnt = 2;
	let networkType: NetworkType = "regression";

	let inputNormalizationFunc: Normalization = "normalization";
	let inputNeuronCnt = 10;

	let hiddenLayersSettings: HiddenLayers = [];

	let outputNeuronCnt = 10;
	let outputActivationFunc: ActivationFunction = "sigmoid";
</script>

<div class="w-full h-full grid place-items-center">
	<div
		class="relative bg-headerBg p-4 rounded-xl w-2/5 h-[70%] flex flex-col gap-4"
	>
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

		<Controls />
	</div>
</div>
