<script>
	import DynamicInput from "./reusable/DynamicInput.svelte";

	let hiddenLayersCnt = 2;
</script>

<div class="absolute top-3 left-3 flex flex-col gap-1">
	<p>
		<label for="hidden-layers-cnt">Hidden layers: </label>
		<DynamicInput
			id="hidden-layers-cnt"
			bind:value={hiddenLayersCnt}
			minWidth={4}
		/>
	</p>
	<p>
		<label for="type-select">Type:</label>
		<select id="type-select">
			<option>Regression</option>
			<option>Classification</option>
		</select>
	</p>
</div>
