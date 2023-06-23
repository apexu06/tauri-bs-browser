import { invoke } from '@tauri-apps/api';
import type { MapDetail } from 'src/types/MapDetail';
import type { MapParams } from 'src/types/MapParams';

export const fetchMaps = (
	params: MapParams,
	maps: MapDetail[]
): Promise<MapDetail[]> => {
	const response = invoke('get_maps', {
		params: params,
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
