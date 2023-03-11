<script lang="ts">
	export let heading: string;
</script>

<details open={true}>
	<summary class="font-bold text-xl">{heading}</summary>
	<hr class="my-2" />
	<slot />
</details>
