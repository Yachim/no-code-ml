<script lang="ts">
	import type { NormalizationFunc } from "../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let neuronCnt: number;
	export let normalizationFunc: NormalizationFunc;
	// if false some options will be disabled
	export let initialSetting: boolean;
</script>

<SettingsSection heading="Input settings">
	<label>
		Number of input neurons
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
		Input normalization:
		<select bind:value={normalizationFunc}>
			<option value="normalization">Normalization</option>
		</select>
	</label>
</SettingsSection>
