export type MapParams = {
	query: string;
	page: number;
	sortOrder: string;
	filters: {
		name: string;
		active: boolean;
	}[];
	minBpm: number;
	maxBpm: number;
	startDate: Date;
	endDate: Date;
};
