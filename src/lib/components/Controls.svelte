<script lang="ts">
	import { faPlay } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";
	import type { CostFunc } from "../../types/network";

	export let costFunc: CostFunc;
	export let iterationCnt: number;
	export let buttonDisabled: boolean;
</script>

<!-- 
	cost info
	start training
-->
<div class="w-full flex gap-4">
	<label>
		Cost function:
		<select bind:value={costFunc}>
			<option value="mse">MSE</option>
		</select>
	</label>

	<!--p>Cost: 0.0</p-->

	<label class="ml-auto">
		Number of iterations:
		<input type="number" bind:value={iterationCnt} />
	</label>

	<button
		disabled={buttonDisabled}
		class="flex gap-2 justify-center items-center"
	>
		<Fa icon={faPlay} />
		Start training
	</button>
</div>
