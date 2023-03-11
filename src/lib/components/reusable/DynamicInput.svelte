<script lang="ts">
	export let value: number;
	export let id = "";
	// min width in ch
	export let minWidth: number;
</script>

<input
	{id}
	type="number"
	bind:value
	style="
		width: {String(value).length + 3}ch;
		min-width: {minWidth}ch;
	"
	class="box-content"
/>
