export type NetworkType = "classification" | "regression";

export type Normalization = "normalization";

export type ActivationFunction = "softmax" | "relu" | "linear" | "sigmoid";

export type HiddenLayer = {
	activationFunc: ActivationFunction;
	neuronCnt: number;
};

export type HiddenLayers = HiddenLayer[];
