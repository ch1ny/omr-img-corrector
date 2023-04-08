import { disableWebviewContextMenu, Paths } from '@/utils';
import { window } from '@tauri-apps/api';

export default async () => {
	disableWebviewContextMenu();
	const currentWindow = window.getCurrent();
	await currentWindow.onCloseRequested((ev) => {
		ev.preventDefault();

		currentWindow.hide();
	});
	await Paths.initPaths();
};
