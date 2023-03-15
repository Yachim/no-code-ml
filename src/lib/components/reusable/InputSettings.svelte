<script lang="ts">
	import type { NormalizationFunc } from "../../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let neuronCnt: number;
	export let normalizationFunc: NormalizationFunc;
</script>

<SettingsSection heading="Input settings">
	<label>
		Number of input neurons<span class="text-red-600">*</span>:
		<input type="number" bind:value={neuronCnt} />
	</label>

	<label>
		Input normalization:
		<select bind:value={normalizationFunc}>
			<option value="normalization">Normalization</option>
		</select>
	</label>
</SettingsSection>
