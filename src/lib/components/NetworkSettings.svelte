<script lang="ts">
	import type { NetworkType } from "../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let hiddenLayersCnt: number;
	export let type: NetworkType;
	// if false some options will be disabled
	export let initialSetting: boolean;
</script>

<SettingsSection heading="Network settings">
	<label>
		Type:
		<select id="asdf" bind:value={type}>
			<option value="regression">Regression</option>
			<option value="classification">Classification</option>
		</select>
	</label>

	<label>
		Hidden layers
		{#if initialSetting}
			<span class="text-red-600">*</span>:
		{/if}
		<input
			type="number"
			disabled={!initialSetting}
			bind:value={hiddenLayersCnt}
		/>
	</label>
</SettingsSection>
