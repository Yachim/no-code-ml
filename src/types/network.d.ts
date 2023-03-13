export type NetworkType = "classification" | "regression";

export type NormalizationFunc = "normalization";

export type ActivationFunc = "softmax" | "relu" | "linear" | "sigmoid";

export type HiddenLayer = {
	activationFunc: ActivationFunc;
	neuronCnt: number;
};

export type HiddenLayers = HiddenLayer[];

export type CostFunc = "mse";
