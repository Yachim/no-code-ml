<script lang="ts">
	import Details from "./Details.svelte";

	let inputCnt = 10;

	let normalizationType: "normalization" = "normalization";
</script>

<Details heading="Input settings">
	<label>
		Number of input neurons:
		<input type="number" bind:value={inputCnt} />
	</label>

	<label>
		Input normalization:
		<select bind:value={normalizationType}>
			<option value="normalization">Normalization</option>
		</select>
	</label>
</Details>
