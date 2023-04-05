import { ICpuInfo } from '@/types';
import { invoke } from '@tauri-apps/api';

/**
 * 显示应用启动窗口
 * @returns
 */
const showSplashWindow = () => {
	return invoke('show_splash_window');
};

/**
 * 显示应用主窗口
 * @returns
 */
const showMainWindow = () => {
	return invoke('show_main_window');
};

/**
 * 显示应用设置窗口
 * @returns
 */
const showSettingsWindow = () => {
	return invoke('show_settings_window');
};

/**
 * 获取系统硬件信息
 * @returns
 */
const getCpuInfo = async () => {
	const cpu_info: any = await invoke('system_cpu_info');

	const cpuInfo: ICpuInfo = {
		chipName: cpu_info.chip_name.trim(),
		physicalCoreCounts: cpu_info.physical_core_counts,
		maxThreadCounts: cpu_info.max_thread_counts,
	};

	return cpuInfo;
};

const getExePath = async (): Promise<string> => {
	return invoke('get_exe_path');
};

export const Invokers = {
	showSplashWindow,
	showMainWindow,
	showSettingsWindow,
	getCpuInfo,
	getExePath,
};
