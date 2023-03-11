<script lang="ts">
	import Details from "./reusable/Details.svelte";

	let outputCnt = 10;
	let activationFunc: "softmax" | "linear" | "relu" | "sigmoid" = "softmax";
</script>

<Details heading="Output settings">
	<label>
		Number of output neurons:
		<input type="number" bind:value={outputCnt} />
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
</Details>
