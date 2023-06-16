// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Metadata } from './Metadata';
import type { Stats } from './Stats';
import type { UserDetail } from './UserDetail';
import type { Version } from './Version';

export interface MapDetail {
	id: string;
	name: string;
	lastPublishedAt: string;
	curatedAt: string | null;
	metadata: Metadata;
	stats: Stats;
	description: string;
	ranked: boolean;
	qualified: boolean;
	versions: Array<Version>;
	automapper: boolean;
	uploader: UserDetail;
}