<script lang="ts">
	import Fa from "svelte-fa";
	import { faPlay } from "@fortawesome/free-solid-svg-icons";

	let iteration_cnt = 10000;
</script>

<div class="absolute top-3 right-3 text-right">
	<p>
		Iterations: <input
			type="number"
			class="bg-bg border-2 border-gray-700 px-2 box-content rounded-lg underline cursor-pointer"
			style="width: {String(iteration_cnt).length + 2}ch"
			bind:value={iteration_cnt}
		/>
		<button>
			<Fa icon={faPlay} />
		</button>
	</p>
	<p>
		Cost (<button class="underline cursor-pointer">MSE</button>): 0.0
	</p>
	<button class="underline cursor-pointer">Load data</button>
</div>
