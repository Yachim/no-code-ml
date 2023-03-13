<script>
	import { faPlay } from "@fortawesome/free-solid-svg-icons";

	import Fa from "svelte-fa";
</script>

<!-- 
	cost info
	start training
-->
<div class="w-full flex gap-4">
	<label
		>Cost function
		<select>
			<option value="mse">MSE</option>
		</select>
	</label>

	<p>Cost: 0.0</p>

	<label class="ml-auto">
		Number of iterations:
		<input type="number" />
	</label>

	<button class="flex gap-2 justify-center items-center">
		<Fa icon={faPlay} />
		Start training
	</button>
</div>
