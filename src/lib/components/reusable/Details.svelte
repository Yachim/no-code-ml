<script lang="ts">
	export let heading: string;
	export let defaultState = false;
</script>

<details open={defaultState}>
	<summary class="font-bold text-xl">{heading}</summary>
	<hr class="my-2" />
	<slot />
</details>
