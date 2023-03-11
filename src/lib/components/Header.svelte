<script lang="ts">
	import Fa from "svelte-fa";
	import { faBars, faPencil } from "@fortawesome/free-solid-svg-icons";
</script>

<header class="w-full px-4 py-2 gap-4 text-2xl bg-headerBg flex">
	<button class="ignore">
		<Fa icon={faBars} />
	</button>
	<div>
		<h1 class="text-lg flex gap-2">
			Unnamed network
			<button class="ignore" title="Edit network">
				<Fa icon={faPencil} />
			</button>
		</h1>
		<h2 class="text-base">Multilayer Perceptron</h2>
	</div>
</header>
