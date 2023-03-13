<script lang="ts">
	import type { Normalization } from "../../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let neuronCnt: number;
	export let normalizationFunc: Normalization;
</script>

<SettingsSection heading="Input settings">
	<label>
		Number of input neurons:
		<input type="number" bind:value={neuronCnt} />
		*
	</label>

	<label>
		Input normalization:
		<select bind:value={normalizationFunc}>
			<option value="normalization">Normalization</option>
		</select>
	</label>
</SettingsSection>
