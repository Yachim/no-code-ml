<script lang="ts">
	export let files: FileList;

	const encoding = "utf-8";
	const decoder = new TextDecoder(encoding);

	const maxCols = 10;
	const maxLines = 20;

	// all headers of a file
	let allHeaders: string[] = [];

	// length equal to maxCols
	let headers: string[] = [];
	let data: string[][] = [];

	export let outputCol: number;
	export let includedCols: number[] = [];

	// TODO: lift state up
	let type: "training" | "testing" = "training";

	async function readCsvFile(
		f: File,
		options?: {
			lineDelimeter?: string;
			delimeter?: string;
			maxCols?: number;
			linesCnt?: number;
		}
	): Promise<string[][]> {
		options = {
			lineDelimeter: "\n",
			delimeter: ",",
			maxCols: 0,
			linesCnt: 0,
			...options,
		};

		const reader = f.stream().getReader();
		const lines: string[][] = [];
		let lastLine = "";
		let linesCntTotal = 0;

		while (true) {
			const { value, done } = await reader.read();

			if (done) break;

			const text = decoder.decode(value, { stream: true });

			if (text.includes(options.lineDelimeter)) {
				const linesSplit = text.split(options.lineDelimeter);

				lastLine += linesSplit[0];
				let lastLineArr = lastLine.split(options.delimeter);
				if (options.maxCols !== 0) {
					lastLineArr = lastLineArr.slice(0, options.maxCols);
				}

				lines.push(lastLineArr);
				linesCntTotal++;
				if (linesCntTotal === options.linesCnt) return lines;

				for (let i = 1; i < linesSplit.length - 1; i++) {
					const line = linesSplit[i];
					let lineArr = line.split(options.delimeter);
					if (options.maxCols !== 0) {
						lineArr = lineArr.slice(0, options.maxCols);
					}

					lines.push(lineArr);
					linesCntTotal++;
					if (linesCntTotal === options.linesCnt) return lines;
				}

				lastLine = linesSplit[linesSplit.length - 1];

				continue;
			}
			lastLine += text;
		}

		return lines;
	}

	$: if (files) {
		readCsvFile(files.item(0), {
			linesCnt: 1,
		}).then((res) => {
			allHeaders = res[0];
		});

		readCsvFile(files.item(0), {
			linesCnt: maxLines,
			maxCols: maxCols,
		}).then((res) => {
			headers = res[0];
			data = res.slice(1);
		});
	} else {
		allHeaders = [];
		headers = [];
		data = [];
	}

	function toggleAllCols() {
		if (
			includedCols.length === allHeaders.length ||
			(!(outputCol === null || outputCol === undefined) &&
				includedCols.length === allHeaders.length - 1)
		) {
			includedCols = [];
		} else {
			includedCols = [...Array(allHeaders.length).keys()];
		}
	}

	$: if (includedCols.includes(outputCol)) {
		includedCols = includedCols.filter((val) => val !== outputCol);
	}
</script>

<div class="overflow-hidden flex flex-col gap-4">
	<div class="flex justify-evenly">
		<button
			class={`
				ignore w-full p-2 border-b-2 transition-colors hover:text-opacity-70
				${type === "training" ? "border-border" : "border-transparent"}
			`}
			on:click={() => (type = "training")}
		>
			Training
		</button>
		<button
			class={`
				ignore w-full p-2 border-b-2 transition-colors hover:text-opacity-70
				${type === "testing" ? "border-border" : "border-transparent"}
			`}
			on:click={() => (type = "testing")}
		>
			Testing/Predicting
		</button>
	</div>

	<div class={!files ? "" : "flex justify-between"}>
		<input type="file" accept=".csv" class="ignore" bind:files />

		{#if files}
			<div class="flex gap-2 items-end">
				<label>
					Output column:
					<select bind:value={outputCol}>
						<option value={-1} selected disabled
							>Select a value</option
						>
						{#each allHeaders as header, i}
							<option value={i}>
								{header}
							</option>
						{/each}
					</select>
				</label>

				<label
					class="custom-input select-cols-toggle-label relative"
					data-disabled={outputCol === null ||
						outputCol === undefined}
					title={outputCol === null || outputCol === undefined
						? "Select an output column first"
						: ""}
				>
					Select included columns

					<input
						disabled={outputCol === null || outputCol === undefined}
						type="checkbox"
						class="hidden select-cols-toggle"
					/>

					<div class="select-cols">
						<button on:click={toggleAllCols}>Toggle all</button>
						<select multiple bind:value={includedCols}>
							{#each allHeaders as header, i}
								{#if i !== outputCol}
									<option value={i}>
										{header}
									</option>
								{/if}
							{/each}
						</select>
					</div>
				</label>
			</div>
		{/if}
	</div>

	{#if files && data.length > 0 && headers.length > 0}
		Preview:
		<div class="overflow-auto">
			<table class="w-full border overflow-scroll">
				<tr class="bg-border">
					{#each headers as cell}
						<th class="text-center border py-2">
							{cell}
						</th>
					{/each}
				</tr>
				{#each data as line, i}
					<tr class:bg-headerBg={i % 2} class:bg-opacity-70={i % 2}>
						{#each line as cell}
							<td class="text-center border py-2">{cell}</td>
						{/each}
					</tr>
				{/each}
			</table>
		</div>
	{/if}
</div>

<style>
	.select-cols {
		display: none;
		position: absolute;
		bottom: 0;
		right: 0;
		translate: 0 calc(2px + 100%);
		padding: 0.5rem;
		gap: 0.5rem;
		flex-direction: column;
		@apply bg-headerBg rounded-lg;
	}

	.select-cols-toggle:checked ~ .select-cols {
		display: flex;
	}

	.select-cols-toggle-label[data-disabled="true"] {
		cursor: not-allowed !important;
		@apply opacity-50;
	}
</style>
