import type { NetworkModelType } from "./network";

export type Net = NetListMember & {
};

type NetListMember = {
	name: string;
	id: string;
	modelType: NetworkModelType
};
export type NetList = NetListMember[];
