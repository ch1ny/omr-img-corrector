import { WebviewWindow } from '@tauri-apps/api/window';

export default async function (taskId: number, src: string) {
	const webview = new WebviewWindow(`modify-${taskId}`, {
		title: `人工审查 - ${src}`,
		url: `views/modify.html?src=${encodeURIComponent(src)}`,
		width: 600,
		height: 725,
		center: true,
		resizable: false,
	});
	webview.setFocus();
}
