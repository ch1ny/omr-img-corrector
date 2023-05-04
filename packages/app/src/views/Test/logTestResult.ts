import path from '@/core/path';
import { IFftParams, IHoughParams, IProjectionParams } from '@/types';
import { getHardwareInfos, Invokers, Paths } from '@/utils';
import { fs } from '@tauri-apps/api';
import { TestResultMistakes } from './ResultCard';

export type TestResult = {
	test_id: number;
	durations: {
		projection: number;
		hough: number;
		fft: number;
	};
	mistakes: {
		projection: TestResultMistakes;
		hough: TestResultMistakes;
		fft: TestResultMistakes;
	};
};

type OutputTestResult = TestResult & {
	params: {
		projection: IProjectionParams;
		hough: IHoughParams;
		fft: IFftParams;
	};
};

export default async function (result: OutputTestResult) {
	if (!Paths.exePath) {
		await Paths.initPaths();
	}

	const logDirPath = path.resolveSync(Paths.exePath, '..', 'logs');
	if (!(await fs.exists(logDirPath))) {
		await fs.createDir(logDirPath);
	}

	const resultLogFilePath = path.resolveSync(logDirPath, 'test_result.log');

	const hardwareInfo = await getHardwareInfos();

	const now = new Date();
	const resultString = [
		`[${now.getFullYear()}-${
			now.getMonth() + 1
		}-${now.getDate()} ${now.getHours()}:${now.getMinutes()}:${now.getSeconds()}] TEST_ID: "${
			result.test_id
		}"`,
		`=== [ HARDWARE INFO ] ===`,
		`[CPU CHIP_NAME] ${hardwareInfo.cpu.chipName}`,
		`[CPU PHYSICAL_CORE_COUNTS] ${hardwareInfo.cpu.physicalCoreCounts}`,
		`[CPU MAX_THREAD_COUNTS] ${hardwareInfo.cpu.maxThreadCounts}`,
		`[GPU RENDERER] ${hardwareInfo.gpu.renderer}`,
		`[GPU VENDOR] ${hardwareInfo.gpu.vendor}`,
		`=== [ PARAMS OPTION ] ===`,
		`[PROJECTION | MAX_SEARCH_ANGLE] ${result.params.projection.maxAngle}°`,
		`[PROJECTION | SEARCH_ANGLE_STEP] ${result.params.projection.angleStep}°`,
		`[PROJECTION | MAX_WIDTH] ${result.params.projection.maxWidth}`,
		`[PROJECTION | MAX_HEIGHT] ${result.params.projection.maxHeight}`,
		`[HOUGH | MIN_LINE_LENGTH] ${result.params.hough.minLineLength}`,
		`[HOUGH | MAX_LINE_GAP] ${result.params.hough.maxLineGap}`,
		`[FFT | CANNY_THRESHOLD_1] ${result.params.fft.cannyThresholdLower}`,
		`[FFT | CANNY_THRESHOLD_2] ${result.params.fft.cannyThresholdHigher}`,
		`[FFT | MIN_LINE_LENGTH] ${result.params.fft.minLineLength}`,
		`[FFT | MAX_LINE_GAP] ${result.params.fft.maxLineGap}`,
		`=== [ TEST RESULT ] ===`,
		`[PROJECTION] ${result.durations.projection}ms | AVE: ${result.mistakes.projection.arithmetic_mean}° | STD: ${result.mistakes.projection.standard_deviation}° | MAX: ${result.mistakes.projection.max_mistake}°`,
		`[HOUGH] ${result.durations.hough}ms | AVE: ${result.mistakes.hough.arithmetic_mean}° | STD: ${result.mistakes.hough.standard_deviation}° | MAX: ${result.mistakes.hough.max_mistake}°`,
		`[FFT] ${result.durations.fft}ms | AVE: ${result.mistakes.fft.arithmetic_mean}° | STD: ${result.mistakes.fft.standard_deviation}° | MAX: ${result.mistakes.fft.max_mistake}°`,
		``,
		``,
	].join('\n');

	await Invokers.appendFile(resultLogFilePath, resultString);
}
