import { disableWebviewContextMenu, Invokers } from '@/utils';
import { window } from '@tauri-apps/api';

disableWebviewContextMenu();

window.getCurrent().onCloseRequested((ev) => {
	ev.preventDefault();
	Invokers.exitApp();
});

Invokers.showSplashWindow();
