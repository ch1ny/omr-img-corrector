import { disableWebviewContextMenu, disableWebviewPrint, disableWebviewRefresh } from '@/utils';
import { window } from '@tauri-apps/api';

export default async () => {
	disableWebviewContextMenu();
	disableWebviewRefresh();
	disableWebviewPrint();
	const currentWindow = window.getCurrent();
	await currentWindow.onCloseRequested((ev) => {
		ev.preventDefault();

		currentWindow.hide();
	});
};
