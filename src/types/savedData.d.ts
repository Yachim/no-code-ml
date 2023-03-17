import type { ActivationFunc, CostFunc, HiddenLayers, NetworkModelType, NetworkType, NormalizationFunc } from "./network";

export type Net = NetListMember & {
	// if false - some settings cannot be modified, e.g. neuron count, hidden layer count
	initialSetting: boolean
} & ({
	modelType: "multilayerPerceptron",

	hiddenLayersCnt: number,
	networkType: NetworkType,

	inputNormalizationFunc: NormalizationFunc,
	inputNeuronCnt: number,

	hiddenLayersSettings: HiddenLayers,

	outputNeuronCnt: number,
	outputActivationFunc: ActivationFunc,
	outputNeuronLabels: string[],

	costFunc: CostFunc,
	iterationCnt: number,

	outputCol: number,
	includedCols: number[],
});

type NetListMember = {
	name: string;
	id: string;
	modelType: NetworkModelType
};
export type NetList = NetListMember[];
