<script lang="ts">
	import {
		QueryClient,
		QueryClientProvider,
	} from "@sveltestack/svelte-query";

	import Header from "./lib/Header.svelte";
	import MultiLayerPerceptronBody from "./lib/MultiLayerPerceptronBody.svelte";
	import { currentNetId } from "./utils/stores";

	const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
	<Header />
	{#if $currentNetId}
		<MultiLayerPerceptronBody />
	{:else}
		<div class="grid place-items-center h-full">Select a network</div>
	{/if}
</QueryClientProvider>
