<script lang="ts">
	import type { NetworkType } from "../../../types/network";
	import SettingsSection from "./SettingsSection.svelte";

	export let hiddenLayersCnt: number;
	export let type: NetworkType;
</script>

<SettingsSection heading="Network settings">
	<label>
		Type:
		<select bind:value={type}>
			<option value="regression">Regression</option>
			<option value="classification">Classification</option>
		</select>
	</label>

	<label>
		Hidden layers<span class="text-red-600">*</span>:
		<input type="number" bind:value={hiddenLayersCnt} />
	</label>
</SettingsSection>
