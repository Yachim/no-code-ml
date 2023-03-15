<script lang="ts">
	let files: FileList;

	const encoding = "utf-8";
	const decoder = new TextDecoder(encoding);

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

	let headers: string[] = [];
	let data: string[][] = [];

	$: if (files) {
		readCsvFile(files.item(0), { linesCnt: 20 }).then((res) => {
			headers = res[0];
			data = res.slice(1);
		});
	} else {
		headers = [];
		data = [];
	}

	const maxCols = 10;
</script>

<div class={!files ? "grid place-items-center h-full" : "flex flex-col gap-4"}>
	<input type="file" accept=".csv" class="ignore" bind:files />
	{#if files}
		<table class="w-full border">
			<tr class="bg-border">
				{#if headers}
					{#each headers.slice(0, maxCols) as cell}
						<th class="text-center border py-2">{cell}</th>
					{/each}
				{/if}
			</tr>
			{#each data as line, i}
				<tr class:bg-headerBg={i % 2} class:bg-opacity-50={i % 2}>
					{#if line}
						{#each line.slice(0, maxCols) as cell}
							<td class="text-center border py-2">{cell}</td>
						{/each}
					{/if}
				</tr>
			{/each}
		</table>
	{/if}
</div>
