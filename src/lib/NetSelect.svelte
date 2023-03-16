<script lang="ts">
	import { faPlus } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";
	import { useCreateNet, useNets } from "../utils/queries";
	import { currentNetId } from "../utils/stores";

	const creteNetMutation = useCreateNet();
	const netListQuery = useNets();

	let checked: boolean;
</script>

<input bind:checked type="checkbox" id="toggle-nav-menu" class="hidden" />

<div
	class="p-4 border-border border-r-2 border-t-2 absolute bg-headerBg left-0 bottom-0 origin-top-left flex-col gap-4 net-select"
>
	<button
		on:click={() => {
			$creteNetMutation.mutate("multilayerPerceptron", {
				onSuccess: (res) => currentNetId.set(res.id),
			});
			checked = false;
		}}
	>
		<Fa class="inline" icon={faPlus} />
		Add network
	</button>

	<div class="flex flex-col gap-2">
		{#if $netListQuery.isSuccess}
			{#each $netListQuery.data as net}
				<button
					on:click={() => {
						currentNetId.set(net.id);
						checked = false;
					}}
				>
					{net.name}
				</button>
			{/each}
		{/if}
	</div>
</div>

<style>
	.net-select {
		height: calc(100vh - 100%);
		display: none;
		translate: 0 100%;
	}

	#toggle-nav-menu:checked ~ .net-select {
		display: flex;
	}
</style>
