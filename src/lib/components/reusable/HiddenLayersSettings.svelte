<script lang="ts">
	import type { HiddenLayers } from "../../../types/network";

	import Details from "./Details.svelte";

	export let hiddenLayersCnt: number;
	export let hiddenLayersSettings: HiddenLayers;

	$: if (hiddenLayersCnt < hiddenLayersSettings.length) {
		hiddenLayersSettings = hiddenLayersSettings.slice(0, hiddenLayersCnt);
	} else if (hiddenLayersCnt > hiddenLayersSettings.length) {
		const range = [
			...Array(hiddenLayersCnt - hiddenLayersSettings.length).keys(),
		];
		const additionalLayers = range.map(() => ({
			activationFunc: "relu" as const,
			neuronCnt: 10,
		}));

		hiddenLayersSettings = [...hiddenLayersSettings, ...additionalLayers];
	}
</script>

<Details heading="Hidden layers settings">
	{#each hiddenLayersSettings as layer, i}
		<p class="text-lg font-bold">Layer {i + 1}</p>

		<label>
			Number of neurons:
			<input type="number" bind:value={layer.neuronCnt} />
		</label>

		<label>
			Layer activation function:
			<select bind:value={layer.activationFunc}>
				<option value="relu">ReLU</option>
				<option value="linear">Linear</option>
				<option value="sigmoid">Sigmoid</option>
				<option value="softmax">Softmax</option>
			</select>
		</label>
	{/each}
</Details>
