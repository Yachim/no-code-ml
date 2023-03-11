<script lang="ts">
	import Details from "./reusable/Details.svelte";

	let hiddenLayersCntInput = 2;
	let type: "classification" | "regression" = "regression";
</script>

<Details heading="Network settings">
	<div class="flex flex-col gap-2">
		<label>
			Type:
			<select bind:value={type}>
				<option value="regression">Regression</option>
				<option value="classification">Classification</option>
			</select>
		</label>

		<label>
			Hidden layers:
			<input type="number" bind:value={hiddenLayersCntInput} />
		</label>
	</div>
</Details>
