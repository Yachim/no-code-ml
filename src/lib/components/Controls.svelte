<script lang="ts">
	import { faPlay } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";
</script>

<!-- 
	number of iterations
	load data
	cost function
	cost info
	start training
	save
	info about *
-->
<div class="w-full mt-auto">
	<div class="flex pl-4 flex-col gap-0.5">
		<label>
			Number of iterations:
			<input type="number" />
		</label>
		<div class="flex justify-between gap-3">
			<button>Load data</button>
			<label>
				Cost function:
				<select>
					<option>MSE</option>
				</select>
			</label>
			<span>Cost: 0.0</span>
		</div>
	</div>
	<button
		class="
			ignore 
			rounded-[50%] aspect-square p-4
			inline-grid place-items-center
			bg-green-500 hover:bg-green-600 transition-colors
		"
		title="Start training"
	>
		<Fa class="text-[2rem]" fw icon={faPlay} />
	</button>
</div>
