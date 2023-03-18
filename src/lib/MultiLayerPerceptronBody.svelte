<script lang="ts">
	import Controls from "./components/Controls.svelte";
	import DataReader from "./components/DataReader.svelte";
	import HiddenLayersSettings from "./components/HiddenLayersSettings.svelte";
	import InputSettings from "./components/InputSettings.svelte";
	import NetworkSettings from "./components//NetworkSettings.svelte";
	import OutputSettings from "./components/OutputSettings.svelte";
	import { isNotSaved, saveFunc } from "../utils/stores";
	import { defaultNetworks, useNet, useSaveNet } from "../utils/queries";

	let {
		hiddenLayersCnt,
		networkType,

		inputNormalizationFunc,
		inputNeuronCnt,

		hiddenLayersSettings,

		outputNeuronCnt,
		outputActivationFunc,
		outputNeuronLabels,

		costFunc,
		iterationCnt,

		outputCol,
		includedCols,
	} = defaultNetworks.multilayerPerceptron;

	let trainingFiles: FileList;

	const netQuery = useNet();

	let netId = "";
	$: if ($netQuery.isSuccess && netId !== $netQuery.data.id) {
		hiddenLayersCnt = $netQuery.data.hiddenLayersCnt;
		networkType = $netQuery.data.networkType;

		inputNormalizationFunc = $netQuery.data.inputNormalizationFunc;
		inputNeuronCnt = $netQuery.data.inputNeuronCnt;

		hiddenLayersSettings = $netQuery.data.hiddenLayersSettings;

		outputNeuronCnt = $netQuery.data.outputNeuronCnt;
		outputActivationFunc = $netQuery.data.outputActivationFunc;
		outputNeuronLabels = $netQuery.data.outputNeuronLabels;

		costFunc = $netQuery.data.costFunc;
		iterationCnt = $netQuery.data.iterationCnt;

		outputCol = $netQuery.data.outputCol;
		includedCols = $netQuery.data.includedCols;

		// so it won't be overwriting values
		netId = $netQuery.data.id;
	}

	// FIXME: setting true with objects
	// TODO: files
	$: if ($netQuery.isSuccess) {
		if (hiddenLayersCnt !== $netQuery.data.hiddenLayersCnt)
			isNotSaved.set(true);
		else if (networkType !== $netQuery.data.networkType)
			isNotSaved.set(true);
		else if (
			inputNormalizationFunc !== $netQuery.data.inputNormalizationFunc
		)
			isNotSaved.set(true);
		else if (inputNeuronCnt !== $netQuery.data.inputNeuronCnt)
			isNotSaved.set(true);
		else if (
			JSON.stringify(hiddenLayersSettings) !==
			JSON.stringify($netQuery.data.hiddenLayersSettings)
		)
			isNotSaved.set(true);
		else if (outputNeuronCnt !== $netQuery.data.outputNeuronCnt)
			isNotSaved.set(true);
		else if (outputActivationFunc !== $netQuery.data.outputActivationFunc)
			isNotSaved.set(true);
		else if (
			JSON.stringify(outputNeuronLabels) !==
			JSON.stringify($netQuery.data.outputNeuronLabels)
		)
			isNotSaved.set(true);
		else if (costFunc !== $netQuery.data.costFunc) isNotSaved.set(true);
		else if (iterationCnt !== $netQuery.data.iterationCnt)
			isNotSaved.set(true);
		else if (outputCol !== $netQuery.data.outputCol) isNotSaved.set(true);
		else if (
			JSON.stringify(includedCols) !==
			JSON.stringify($netQuery.data.includedCols)
		)
			isNotSaved.set(true);
		else isNotSaved.set(false);
	}

	const saveNetMutation = useSaveNet();

	saveFunc.set(() => {
		$saveNetMutation.mutate({
			id: $netQuery.data.id,
			modelType: $netQuery.data.modelType,

			hiddenLayersCnt,
			networkType,

			inputNormalizationFunc,
			inputNeuronCnt,

			hiddenLayersSettings,

			outputNeuronCnt,
			outputActivationFunc,
			outputNeuronLabels,

			costFunc,
			iterationCnt,

			outputCol,
			includedCols,
		});
	});

	let buttonDisabled = true;
	$: {
		if (!networkType) buttonDisabled = true;
		else if (!inputNormalizationFunc) buttonDisabled = true;
		else if (!inputNeuronCnt || inputNeuronCnt <= 0) buttonDisabled = true;
		else if (
			hiddenLayersCnt > 0 &&
			!hiddenLayersSettings.every(
				(layer) => layer.activationFunc && layer.neuronCnt > 0
			)
		) {
			buttonDisabled = true;
		} else if (!outputNeuronCnt || outputNeuronCnt <= 0)
			buttonDisabled = true;
		else if (!outputActivationFunc) buttonDisabled = true;
		else if (!costFunc) buttonDisabled = true;
		else if (!iterationCnt || iterationCnt <= 0) buttonDisabled = true;
		else if (!trainingFiles) buttonDisabled = true;
		else if (outputCol === undefined || outputCol === null)
			buttonDisabled = true;
		else if (!includedCols || includedCols.length <= 0)
			buttonDisabled = true;
		else buttonDisabled = false;
	}
</script>

{#if $netQuery.isError}
	Error while loading network
{:else if $netQuery.isSuccess}
	<div class="w-full h-full flex overflow-hidden">
		<div
			class="border-r-border border-r p-4 flex flex-col overflow-auto gap-4"
		>
			{#if $netQuery.data.initialSetting}
				<p>
					<span class="text-red-600">*</span>
					cannot be changed after the first save
				</p>
			{/if}
			<NetworkSettings
				bind:hiddenLayersCnt
				bind:type={networkType}
				initialSetting={$netQuery.data.initialSetting}
			/>
			<InputSettings
				bind:normalizationFunc={inputNormalizationFunc}
				bind:neuronCnt={inputNeuronCnt}
				initialSetting={$netQuery.data.initialSetting}
			/>
			{#if hiddenLayersCnt > 0}
				<HiddenLayersSettings
					bind:hiddenLayersCnt
					bind:hiddenLayersSettings
					initialSetting={$netQuery.data.initialSetting}
				/>
			{/if}
			<OutputSettings
				bind:neuronCnt={outputNeuronCnt}
				bind:activationFunc={outputActivationFunc}
				bind:neuronLabels={outputNeuronLabels}
				initialSetting={$netQuery.data.initialSetting}
			/>
		</div>

		<div class="flex flex-col w-full p-4 gap-4">
			<Controls bind:costFunc bind:iterationCnt {buttonDisabled} />
			<DataReader bind:outputCol bind:includedCols bind:trainingFiles />
		</div>
	</div>
{/if}
