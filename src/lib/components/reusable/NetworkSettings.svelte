<script lang="ts">
	import type { NetworkType } from "../../../types/network";

	import Details from "./Details.svelte";

	export let hiddenLayersCnt: number;
	export let type: NetworkType;
</script>

<Details heading="Network settings" defaultState={true}>
	<label>
		Type:
		<select bind:value={type}>
			<option value="regression">Regression</option>
			<option value="classification">Classification</option>
		</select>
	</label>

	<label>
		Hidden layers:
		<input type="number" bind:value={hiddenLayersCnt} />
		*
	</label>
</Details>
