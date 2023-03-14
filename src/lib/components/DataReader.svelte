<script lang="ts">
	let files: FileList;

	const encoding = "utf-8";
	const decoder = new TextDecoder(encoding);

	async function readFile(
		f: File,
		linesCnt: number,
		delimeter = "\n"
	): Promise<string[]> {
		const reader = f.stream().getReader();
		const lines: string[] = [];
		let lastLine = "";
		let linesCntTotal = 0;

		let reading = true;

		while (reading) {
			const { value, done } = await reader.read();

			if (done) break;

			const text = decoder.decode(value, { stream: true });

			if (text.includes(delimeter)) {
				const linesSplit = text.split(delimeter);

				lastLine += linesSplit[0];
				lines.push(lastLine);
				linesCntTotal++;
				if (linesCntTotal === linesCnt) return lines;

				for (let i = 1; i < linesSplit.length - 1; i++) {
					const line = linesSplit[1];

					lines.push(line);
					linesCntTotal++;
					if (linesCntTotal === linesCnt) return lines;
				}

				lastLine = linesSplit[linesSplit.length - 1];

				continue;
			}
			lastLine += text;
		}

		return lines;
	}

	$: if (files) {
		console.log(readFile(files.item(0), 4, "\r\n"));
	}
</script>

{#if !files}
	<div class="grid place-items-center h-full">
		<input type="file" accept=".csv" class="ignore" bind:files />
	</div>
{:else}
	<div>ff</div>
{/if}
