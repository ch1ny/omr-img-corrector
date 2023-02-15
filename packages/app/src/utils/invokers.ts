import { ISystemHardwareInfo } from '@/types';
import { invoke } from '@tauri-apps/api';

/**
 * 显示应用主窗口
 * @returns
 */
const showMainWindow = () => {
	return invoke('show_main_window');
};

/**
 * 获取系统硬件信息
 * @returns
 */
const getSystemHardwareInfo = async () => {
	const hardwares = await invoke('system_hardware_info');

	return <ISystemHardwareInfo>hardwares;
};

export const Invokers = {
	showMainWindow,
	getSystemHardwareInfo,
};
