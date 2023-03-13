<script lang="ts">
	import { faSave } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";
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

<div class="w-full h-full flex">
	<div class="p-4 border-r-border border-r overflow-auto">
		<div class="flex flex-col-gap-4">
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

		<div class="mt-auto flex flex-col gap-2 items-start">
			<p>* cannot be changed after initial setting</p>
			<button class="flex gap-1 justify-center items-center">
				<Fa icon={faSave} class="inline" /> Save
			</button>
		</div>
	</div>
	<Controls />
</div>
