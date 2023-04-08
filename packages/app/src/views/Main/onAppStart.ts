import path from '@/core/path';
import { disableWebviewContextMenu, Invokers, Paths } from '@/utils';
import { fs, window } from '@tauri-apps/api';

const START_TIME = Date.now();
const MIN_SPLASH_DURATION = 750; // splash window 至少 0.75 秒

export default async () => {
	disableWebviewContextMenu();

	const mainWindow = window.getCurrent();
	await mainWindow.onCloseRequested(async (ev) => {
		ev.preventDefault();
		Invokers.exitApp();
	});

	try {
		await Paths.initPaths();
		const defaultOutputPath = path.resolveSync(Paths.exePath, '..', 'output');
		if (!(await fs.exists(defaultOutputPath))) {
			await fs.createDir(defaultOutputPath);
		}
	} catch (error) {
		console.error(error);
	}

	setTimeout(async () => {
		await Invokers.showMainWindow();
		mainWindow.requestUserAttention(1);
	}, MIN_SPLASH_DURATION - Date.now() + START_TIME);
};
