<script>
	import Controls from "./components/Controls.svelte";
	import HiddenLayersSettings from "./components/reusable/HiddenLayersSettings.svelte";
	import InputSettings from "./components/reusable/InputSettings.svelte";
	import NetworkSettings from "./components/reusable/NetworkSettings.svelte";
	import OutputSettings from "./components/reusable/OutputSettings.svelte";

	let hiddenLayersCnt = 2;
</script>

<div class="w-full h-full grid place-items-center">
	<div
		class="relative bg-headerBg p-4 rounded-xl w-2/5 h-[70%] flex flex-col gap-4"
	>
		<NetworkSettings bind:hiddenLayersCnt />
		<InputSettings />
		{#if hiddenLayersCnt > 0}
			<HiddenLayersSettings {hiddenLayersCnt} />
		{/if}
		<OutputSettings />

		<Controls />
	</div>
</div>
