<script lang="ts">
	import {
		QueryClient,
		QueryClientProvider,
	} from "@sveltestack/svelte-query";

	import Header from "./lib/components/Header.svelte";
	import MultiLayerPerceptronBody from "./lib/MultiLayerPerceptronBody.svelte";

	const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
	<Header />
	<MultiLayerPerceptronBody />
</QueryClientProvider>
