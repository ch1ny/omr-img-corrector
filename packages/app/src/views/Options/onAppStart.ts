import { window } from '@tauri-apps/api';

export default async () => {
	const currentWindow = window.getCurrent();
	currentWindow.onCloseRequested((ev) => {
		ev.preventDefault();

		currentWindow.hide();
	});
};
