import type { NetworkModelType } from "./network";

export type Net = NetListMember & {
	// if false - some settings cannot be modified, e.g. neuron count, hidden layer count
	initialSetting: boolean
};

type NetListMember = {
	name: string;
	id: string;
	modelType: NetworkModelType
};
export type NetList = NetListMember[];
