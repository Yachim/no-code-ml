<script lang="ts">
	let files: FileList;

	const encoding = "utf-8";
	const decoder = new TextDecoder(encoding);

	const maxCols = 10;
	const maxLines = 20;

	// all headers of a file
	let allHeaders: string[] = [];

	// length equal to maxCols
	let headers: string[] = [];
	let data: string[][] = [];

	let outputCol: number;
	let includedCols: number[] = [];

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
			((outputCol ?? false) &&
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

<div class={!files ? "grid place-items-center h-full" : "flex flex-col gap-4"}>
	<div class={!files ? "" : "flex justify-between"}>
		<input type="file" accept=".csv" class="ignore" bind:files />

		{#if files}
			<div class="flex gap-2 items-end">
				<button on:click={toggleAllCols}>Toggle all columns</button>

				<label>
					Included columns:
					<select multiple bind:value={includedCols}>
						{#each allHeaders as header, i}
							{#if i !== outputCol}
								<option value={i}>
									{header}
								</option>
							{/if}
						{/each}
					</select>
				</label>

				<label>
					Output column:
					<select bind:value={outputCol}>
						{#each allHeaders as header, i}
							<option value={i}>
								{header}
							</option>
						{/each}
					</select>
				</label>
			</div>
		{/if}
	</div>

	{#if files && data.length > 0 && headers.length > 0}
		<table class="w-full border">
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
	{/if}
</div>
