<script lang="ts">
	// TODO: fix the ifs in h1 and h2

	import Fa from "svelte-fa";
	import {
		faBars,
		faPencil,
		faSave,
	} from "@fortawesome/free-solid-svg-icons";
	import { currentNetId, saveFunc } from "../utils/stores";
	import NetSelect from "./NetSelect.svelte";
</script>

<header class="relative w-full px-4 py-2 gap-4 bg-headerBg flex items-center">
	<label
		for="toggle-nav-menu"
		class="ignore transition-colors hover:text-opacity-70 text-text text-2xl cursor-pointer"
	>
		<Fa icon={faBars} />
	</label>

	<NetSelect />

	<div>
		<h1 class="text-lg flex gap-3">
			{#if $currentNetId}
				Unnamed network
				<button
					class="ignore transition-colors hover:text-opacity-70 text-text"
					title="Edit network"
				>
					<Fa icon={faPencil} />
				</button>
				<button
					class="ignore transition-colors hover:text-opacity-70 text-text"
					title="Save network"
					on:click={$saveFunc}
				>
					<Fa icon={faSave} />
				</button>
			{:else}
				&nbsp;
			{/if}
		</h1>
		<h2 class="text-base">
			{#if $currentNetId}
				Multilayer Perceptron
			{:else}
				&nbsp;
			{/if}
		</h2>
	</div>
</header>
