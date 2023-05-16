import * as tauriPath from '@tauri-apps/api/path';
import path from '@/core/path';
import { ICpuInfo } from '@/types';
import { ITaskProps } from '@/views/Main/Task';
import { fs, invoke } from '@tauri-apps/api';
import type { getLibParams } from './getLibParams';
import { Paths } from './paths';

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
 * 显示算法测试窗口
 * @returns
 */
const showTestWindow = () => {
	return invoke('show_test_window');
};

/**
 * 获取中央处理器信息
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
	const exePath: string = await invoke('get_exe_path');

	return exePath.replace(/^(\\\\\?\\)(.*?)/, '$2');
};

const runTest = async (testId: number, params: ReturnType<typeof getLibParams>) => {
	if (Paths.exePath === undefined) {
		await Paths.initPaths();
	}
	const testOutputDir = path.resolveSync(Paths.exePath, '..', 'resources', 'test', 'result');
	await Promise.all(
		['edges', 'fft', 'hough', 'projection', 'fft_lined'].map(async (dirName) => {
			if (!(await fs.exists(path.resolveSync(testOutputDir, dirName)))) {
				await fs.createDir(path.resolveSync(testOutputDir, dirName));
			}
		})
	);

	return invoke('run_test', {
		testId,
		...params,
	});
};

const exitApp = async () => invoke('exit_app');

const appendFile = async (filePath: string, fileData: string) => {
	return invoke('append_file', {
		targetFilePath: filePath,
		stringData: fileData,
	});
};

const requestWindowShow = async (windowLabel: string) =>
	invoke('request_window_show', {
		windowLabel,
	});

const addTask = async (taskProps: ITaskProps) => {
	const fileExt = await tauriPath.extname(taskProps.src);
	const fileName = await tauriPath.basename(taskProps.src, `.${fileExt}`);

	invoke('add_task', {
		taskId: taskProps.id,
		inputFile: taskProps.src,
		outputFile: path.resolveSync(taskProps.omrConfig.outputDir, `${fileName}.jpg`),
		projectionMaxAngle: taskProps.omrConfig.projectionMaxAngle,
		projectionAngleStep: taskProps.omrConfig.projectionAngleStep,
		projectionMaxWidth: taskProps.omrConfig.projectionMaxWidth,
		projectionMaxHeight: taskProps.omrConfig.projectionMaxHeight,
		houghMinLineLength: taskProps.omrConfig.houghMinLineLength,
		houghMaxLineGap: taskProps.omrConfig.houghMaxLineGap,
	});
};

const setThreadCounts = async (threadCounts: number) => {
	invoke('set_max_workers_count', {
		count: threadCounts,
	});
};

export const Invokers = {
	showSplashWindow,
	showMainWindow,
	showSettingsWindow,
	showTestWindow,
	requestWindowShow,
	appendFile,
	getCpuInfo,
	getExePath,
	runTest,
	exitApp,
	addTask,
	setThreadCounts,
};
