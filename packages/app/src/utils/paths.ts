import { path } from '@tauri-apps/api';
import { Invokers } from './invokers';

export class Paths {
	static appCacheDir: string;
	static appConfigDir: string;
	static appDataDir: string;
	static appLocalDataDir: string;
	static appLogDir: string;
	static audioDir: string;
	static cacheDir: string;
	static configDir: string;
	static dataDir: string;
	static desktopDir: string;
	static documentDir: string;
	static downloadDir: string;
	static homeDir: string;
	static localDataDir: string;
	static pictureDir: string;
	static publicDir: string;
	static resourceDir: string;
	static templateDir: string;
	static videoDir: string;
	static exePath: string;

	static async initPaths() {
		[
			this.appCacheDir,
			this.appConfigDir,
			this.appDataDir,
			this.appLocalDataDir,
			this.appLogDir,
			this.audioDir,
			this.cacheDir,
			this.configDir,
			this.dataDir,
			this.desktopDir,
			this.documentDir,
			this.downloadDir,
			this.homeDir,
			this.localDataDir,
			this.pictureDir,
			this.publicDir,
			this.resourceDir,
			this.templateDir,
			this.videoDir,
			this.exePath,
		] = await Promise.all([
			path.appCacheDir(),
			path.appConfigDir(),
			path.appDataDir(),
			path.appLocalDataDir(),
			path.appLogDir(),
			path.audioDir(),
			path.cacheDir(),
			path.configDir(),
			path.dataDir(),
			path.desktopDir(),
			path.documentDir(),
			path.downloadDir(),
			path.homeDir(),
			path.localDataDir(),
			path.pictureDir(),
			path.publicDir(),
			path.resourceDir(),
			path.templateDir(),
			path.videoDir(),
			Invokers.getExePath(),
		]);
	}

	static getAll() {
		return {
			appCacheDir: this.appCacheDir,
			appConfigDir: this.appConfigDir,
			appDataDir: this.appDataDir,
			appLocalDataDir: this.appLocalDataDir,
			appLogDir: this.appLogDir,
			audioDir: this.audioDir,
			cacheDir: this.cacheDir,
			configDir: this.configDir,
			dataDir: this.dataDir,
			desktopDir: this.desktopDir,
			documentDir: this.documentDir,
			downloadDir: this.downloadDir,
			homeDir: this.homeDir,
			localDataDir: this.localDataDir,
			pictureDir: this.pictureDir,
			publicDir: this.publicDir,
			resourceDir: this.resourceDir,
			templateDir: this.templateDir,
			videoDir: this.videoDir,
			exePath: this.exePath,
		};
	}
}
