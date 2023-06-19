import { invoke } from '@tauri-apps/api';
import type { MapDetail } from 'src/types/MapDetail';

export const fetchMaps = (
	query: string,
	page: number,
	sortOrder: string,
	onlyRanked: boolean,
	onlyVerified: boolean,
	onlyCurated: boolean,
	bpmValues: number[],
	maps: MapDetail[]
): Promise<MapDetail[]> => {
	let filters = [
		{
			name: 'ranked',
			active: onlyRanked,
		},
		{
			name: 'verified',
			active: onlyVerified,
		},
		{
			name: 'curated',
			active: onlyCurated,
		},
	];
	const response = invoke('get_maps', {
		query: query,
		page: page,
		sortMode: sortOrder,
		filters: filters,
		minBpm: bpmValues[0],
		maxBpm: bpmValues[1],
		currentMaps: maps,
	})
		.then((res) => {
			return res as MapDetail[];
		})
		.catch((err) => {
			throw err;
		});

	return response;
};
