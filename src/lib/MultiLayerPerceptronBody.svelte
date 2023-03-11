<script>
	import Controls from "./components/Controls.svelte";
	import InputSettings from "./components/InputSettings.svelte";
	import NetworkSettings from "./components/NetworkSettings.svelte";
	import OutputSettings from "./components/OutputSettings.svelte";
</script>

<div class="w-full h-full grid place-items-center">
	<div
		class="relative bg-headerBg p-4 rounded-xl w-2/5 h-[70%] flex flex-col gap-4"
	>
		<NetworkSettings />
		<InputSettings />
		<!-- TODO: add settings for hidden layers - activation and neuron cnt -->
		<OutputSettings />

		<Controls />
	</div>
</div>
