<script lang="ts">
	export let heading: string;
</script>

<div>
	<p class="font-bold text-xl">{heading}</p>
	<hr class="my-2" />

	<div class="flex flex-col gap-2">
		<slot />
	</div>
</div>
