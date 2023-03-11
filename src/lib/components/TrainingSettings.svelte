<script lang="ts">
	import Fa from "svelte-fa";
	import { faPlay } from "@fortawesome/free-solid-svg-icons";
	import DynamicInput from "./reusable/DynamicInput.svelte";

	let iterationCnt = 10000;
</script>

<div class="absolute top-3 right-3 text-right flex flex-col gap-1">
	<p>
		<label for="iteration_cnt">Iterations:</label>
		<DynamicInput
			id="iteration-cnt"
			bind:value={iterationCnt}
			minWidth={8}
		/>
		<button class="ml-2 ignore">
			<Fa icon={faPlay} />
		</button>
	</p>
	<p>
		<label for="cost-function-select">Cost function: </label>
		<select id="cost-function-select">
			<option>MSE</option>
		</select>
	</p>
	<p>Cost: 0.0</p>
	<button class="ml-auto">Load data</button>
</div>
