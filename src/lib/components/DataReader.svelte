<script lang="ts">
	import { faTag } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";

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

	let outputCol: number;
	// indexes of columns to show in table
	let filteredIndexes: number[];

	$: {
		console.log("a");
		if (headers.length > 0 && data.length > 0) {
			console.log("b1");
			filteredIndexes = [...Array(maxCols).keys()];

			if (outputCol ?? false) {
				console.log("c");

				if (filteredIndexes.includes(outputCol)) {
					console.log("d1");
					const i = filteredIndexes.findIndex(
						(val) => val === outputCol
					);
					console.log(i);
					console.log(filteredIndexes);
					filteredIndexes.splice(i, 1);
					console.log(filteredIndexes);
				} else {
					console.log("d2");
					filteredIndexes.splice(filteredIndexes.length - 1, 1);
				}

				filteredIndexes = [outputCol, ...filteredIndexes];
			}
		} else {
			console.log("b2");
			filteredIndexes = [];
		}
	}
</script>

<div class={!files ? "grid place-items-center h-full" : "flex flex-col gap-4"}>
	<div class={!files ? "" : "flex justify-between"}>
		<input type="file" accept=".csv" class="ignore" bind:files />

		{#if files}
			<label>
				Label column:
				<select bind:value={outputCol}>
					{#each headers as header, i}
						<option value={i}>
							{header}
						</option>
					{/each}
				</select>
			</label>
		{/if}
	</div>

	{#if files && filteredIndexes.length > 0}
		<table class="w-full border">
			<tr class="bg-border">
				{#each filteredIndexes as cellI}
					<th class="text-center border py-2">
						{headers[cellI]}
						{#if cellI === outputCol}
							<span class="ml-1" title="Output column">
								<Fa class="inline" icon={faTag} />
							</span>
						{/if}
					</th>
				{/each}
			</tr>
			{#each data as line, i}
				<tr class:bg-headerBg={i % 2} class:bg-opacity-70={i % 2}>
					{#each filteredIndexes as cellI}
						<td class="text-center border py-2">{line[cellI]}</td>
					{/each}
				</tr>
			{/each}
		</table>
	{/if}
</div>
