<script lang="ts">
	import type { ActivationFunction } from "../../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let neuronCnt: number;
	export let activationFunc: ActivationFunction;
</script>

<SettingsSection heading="Output settings">
	<label>
		Number of output neurons:
		<input type="number" bind:value={neuronCnt} />
		*
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
</SettingsSection>
