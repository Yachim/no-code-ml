<script lang="ts">
	import type { ActivationFunc } from "../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let neuronCnt: number;
	export let activationFunc: ActivationFunc;
	export let neuronLabels: string[];
	neuronLabels = Array(neuronCnt);

	// if false some options will be disabled
	export let initialSetting: boolean;

	$: if (neuronCnt >= neuronLabels.length) {
		neuronLabels = [
			...neuronLabels,
			...Array(neuronCnt - neuronLabels.length),
		];
	} else {
		neuronLabels = neuronLabels.slice(0, neuronCnt);
	}
</script>

<SettingsSection heading="Output settings">
	<label>
		Number of output neurons
		{#if initialSetting}
			<span class="text-red-600">*</span>:
		{/if}
		<input
			disabled={!initialSetting}
			type="number"
			bind:value={neuronCnt}
		/>
	</label>

	<label>
		Output layer activation function:
		<select bind:value={activationFunc}>
			<option value="relu">ReLU</option>
			<option value="linear">Linear</option>
			<option value="sigmoid">Sigmoid</option>
			<option value="softmax">Softmax</option>
		</select>
	</label>

	<p class="text-lg font-bold">Labels</p>

	{#each neuronLabels as label, i}
		<label for="">
			{i + 1}: <input type="text" bind:value={label} />
		</label>
	{/each}
</SettingsSection>
