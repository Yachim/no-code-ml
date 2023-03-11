<script>
	import { faPlay } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";
</script>

<div
	class="bg-controlsBg absolute bottom-0 right-6 translate-y-[50%] flex rounded-full items-center gap-3"
>
	<!-- load data, play, iteration count, and cost info -->
	<div class="flex pl-4 flex-col gap-0.5">
		<label>
			Number of iterations:
			<input type="number" />
		</label>
		<div class="flex justify-between">
			<button>Load data</button>
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
