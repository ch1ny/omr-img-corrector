import path from '@/core/path';
import { IFftParams, IHoughParams, IProjectionParams, IUseMultiThreadParams } from '@/types';
import { Paths } from './paths';

function getValueFromLocalStorageByKey<T>(
	key: string,
	defaultValue: T,
	serializer: (value: T) => string = JSON.stringify
): T {
	try {
		const stored = localStorage.getItem(key);
		if (stored === null) throw Error();

		return JSON.parse(stored);
	} catch (error) {
		localStorage.setItem(key, serializer(defaultValue));
		return defaultValue;
	}
}

export const getLibParams = () => {
	const useMultiThread = getValueFromLocalStorageByKey<IUseMultiThreadParams>('use_multi_thread', {
		use: false,
		threadCounts: 1,
	});
	const defaultOutputDir = getValueFromLocalStorageByKey<string>(
		'default_output_dir',
		path.resolveSync(Paths.exePath, '..', 'output')
	);
	const projectionParams = getValueFromLocalStorageByKey<IProjectionParams>('projection_params', {
		maxAngle: 45,
		angleStep: 0.2,
		imageResizeScale: 0.2,
	});
	const houghParams = getValueFromLocalStorageByKey<IHoughParams>('hough_params', {
		minLineLength: 125.0,
		maxLineGap: 15.0,
	});
	const fftParams = getValueFromLocalStorageByKey<IFftParams>('fft_params', {
		cannyThresholdLower: 125.0,
		cannyThresholdHigher: 150.0,
		minLineLength: 125.0,
		maxLineGap: 15.0,
	});

	return {
		outputDir: defaultOutputDir,
		usedThreads: useMultiThread?.use ? useMultiThread?.threadCounts ?? 0 : 1,
		projectionMaxAngle: projectionParams.maxAngle || 45,
		projectionAngleStep: projectionParams.angleStep || 0.2,
		projectionResizeScale: projectionParams.imageResizeScale || 0.2,
		houghMinLineLength: houghParams.minLineLength || 125.0,
		houghMaxLineGap: houghParams.maxLineGap || 5.0,
		fftCannyThresholdLower: fftParams.cannyThresholdLower || 125.0,
		fftCannyThresholdHigher: fftParams.cannyThresholdHigher || 150.0,
		fftMinLineLength: fftParams.minLineLength || 125.0,
		fftMaxLineGap: fftParams.maxLineGap || 5.0,
	};
};
