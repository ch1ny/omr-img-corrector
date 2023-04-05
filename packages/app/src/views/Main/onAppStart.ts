import { Invokers } from '@/utils';
import { process, window } from '@tauri-apps/api';

const START_TIME = Date.now();
const MIN_SPLASH_DURATION = 3000; // splash window 至少 3 秒

export default async () => {
	await window.getCurrent().onCloseRequested(async (ev) => {
		ev.preventDefault();
		await Promise.all(window.getAll().map((win) => win.hide()));
		process.exit(1);
	});

	setTimeout(() => {
		Invokers.showMainWindow();
	}, MIN_SPLASH_DURATION - Date.now() + START_TIME);
};
