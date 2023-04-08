import { dialog, process } from '@tauri-apps/api';

export async function restoreAppParams() {
	if (
		!(await dialog.confirm('您确定要重置应用参数吗？', {
			title: '重置参数',
			type: 'warning',
		}))
	)
		return;

	localStorage.clear();
	return process.relaunch();
}
