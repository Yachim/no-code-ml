<script lang="ts">
	import Fa from "svelte-fa";
	import {
		faBars,
		faPencil,
		faSave,
		faTrashCan,
	} from "@fortawesome/free-solid-svg-icons";
	import { currentNetId, isNotSaved, saveFunc } from "../utils/stores";
	import NetSelect from "./NetSelect.svelte";
	import { useDeleteNet, useNet, useRenameNet } from "../utils/queries";
	import type { NetworkModelType } from "../types/network";

	const modelTypeDict: {
		[key in NetworkModelType]: string;
	} = {
		multilayerPerceptron: "Multilayer Perceptron",
	};

	const renameNetMutation = useRenameNet();

	function renameNet() {
		let name = prompt("Enter the new name of the network");

		$renameNetMutation.mutate({
			id: $currentNetId,
			name,
		});
	}

	const deleteNetMutation = useDeleteNet();

	function deleteNet() {
		if (confirm("Are you sure you want to delete this network?")) {
			$deleteNetMutation.mutate($currentNetId);
		}
	}

	const netQuery = useNet();
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
			{#if $netQuery.isSuccess}
				<p>
					{$netQuery.data.name}{#if $isNotSaved}
						<span class="font-bold italic" title="Not saved">
							*
						</span>
					{/if}
				</p>
				<button
					class="ignore transition-colors hover:text-opacity-70 text-text"
					title="Edit network"
					on:click={renameNet}
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
				<button
					class="ignore transition-colors hover:text-opacity-70 text-text"
					title="Delete network"
					on:click={deleteNet}
				>
					<Fa icon={faTrashCan} />
				</button>
			{:else}
				&nbsp;
			{/if}
		</h1>
		<h2 class="text-base">
			{#if $netQuery.isSuccess}
				{modelTypeDict[$netQuery.data.modelType]}
			{:else}
				&nbsp;
			{/if}
		</h2>
	</div>
</header>
