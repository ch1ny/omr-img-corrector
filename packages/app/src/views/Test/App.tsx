import { IFftParams, IHoughParams, IProjectionParams } from '@/types';
import { getLibParams, Invokers } from '@/utils';
import { Button, CircularProgress, Divider, Grid, LinearProgress } from '@mui/material';
// NOTE: 这里使用 window 会报错：Error: @vitejs/plugin-react can't detect preamble. Something is wrong.
import { event, window as tauriApiWindow } from '@tauri-apps/api';
import { useCallback, useEffect, useState } from 'react';
import styles from './App.module.less';
import logTestResult, { TestResult } from './logTestResult';
import onAppStart from './onAppStart';
import ResultCard from './ResultCard';

type RunTestProgressEventPayload = {
	method: 'Projection' | 'Hough' | 'FFT';
	processed_count: number;
	processed_progress: number;
	test_id: number;
	total_count: number;
};

interface MethodTestProgress {
	status: 'WAITING' | 'RUNNING' | 'PROCESSED';
	details: RunTestProgressEventPayload;
}

const createDefaultMethodProgress = (testId: number, method: 'Projection' | 'Hough' | 'FFT') =>
	({
		status: 'WAITING',
		details: {
			method: method,
			processed_count: 0,
			processed_progress: 0,
			test_id: testId,
			total_count: 0,
		},
	} as MethodTestProgress);

type TestInfo = {
	testId: number;
	testParams?: {
		projection: IProjectionParams;
		hough: IHoughParams;
		fft: IFftParams;
	};
};

function App() {
	const [initStatus] = useState(onAppStart());
	useEffect(() => {
		const unListen = event.listen('request_show', async ({ windowLabel }) => {
			if (windowLabel !== 'test') return;

			await initStatus;
			await Invokers.showTestWindow();
		});

		return () => {
			unListen.then((unListenFn) => {
				unListenFn();
			});
		};
	}, []);

	const [testInfo, setTestInfo] = useState<TestInfo>({
		testId: 0,
	});
	const [projectionTestProgress, setProjectionTestProgress] = useState<MethodTestProgress>(
		createDefaultMethodProgress(0, 'Projection')
	);
	const [houghTestProgress, setHoughTestProgress] = useState<MethodTestProgress>(
		createDefaultMethodProgress(0, 'Hough')
	);
	const [fftTestProgress, setFftTestProgress] = useState<MethodTestProgress>(
		createDefaultMethodProgress(0, 'FFT')
	);

	const [testResult, setTestResult] = useState<TestResult | null>(null);
	useEffect(() => {
		const unListenTestResult = event.listen('test_result', (ev) => {
			if (ev.windowLabel !== 'test') return;

			setTestInfo((currentTestInfo) => {
				const receivedResult = ev.payload as TestResult;
				if (receivedResult.test_id !== currentTestInfo.testId) return currentTestInfo;

				setTestResult(receivedResult);
				logTestResult({
					...receivedResult,
					params: currentTestInfo.testParams!,
				});
				return {
					testId: 0,
				};
			});
			const currentWindow = tauriApiWindow.getCurrent();
			currentWindow.show().then(() => {
				currentWindow.setFocus();
			});
		});
		const unListenTestProgress = event.listen('run_test_progress_event', (ev) => {
			if (ev.windowLabel !== 'test') return;

			const payload = ev.payload as RunTestProgressEventPayload;

			const getNewProgress = (oldProgress: MethodTestProgress): MethodTestProgress => {
				if (payload.test_id !== oldProgress.details.test_id) return oldProgress;

				const newProgressDetails =
					oldProgress.details.processed_progress < payload.processed_progress
						? payload
						: oldProgress.details;

				const newState: MethodTestProgress = {
					status: newProgressDetails.processed_progress === 1 ? 'PROCESSED' : 'RUNNING',
					details: newProgressDetails,
				};

				// console.log(newState);
				return newState;
			};

			switch (payload.method) {
				case 'Projection':
					setProjectionTestProgress((oldProgress) => getNewProgress(oldProgress));
					break;
				case 'Hough':
					setHoughTestProgress((oldProgress) => getNewProgress(oldProgress));
					break;
				case 'FFT':
					setFftTestProgress((oldProgress) => getNewProgress(oldProgress));
			}
		});

		return () => {
			unListenTestProgress.then((unListenFn) => {
				unListenFn();
			});
			unListenTestResult.then((unListenFn) => {
				unListenFn();
			});
		};
	}, []);

	const runTest = useCallback(() => {
		const newTestId = Date.now();
		const libParams = getLibParams();
		setTestInfo({
			testId: newTestId,
			testParams: {
				projection: {
					maxAngle: libParams.projectionMaxAngle,
					angleStep: libParams.projectionAngleStep,
					imageResizeScale: libParams.projectionResizeScale,
				},
				hough: {
					minLineLength: libParams.houghMinLineLength,
					maxLineGap: libParams.houghMaxLineGap,
				},
				fft: {
					cannyThresholdLower: libParams.fftCannyThresholdLower,
					cannyThresholdHigher: libParams.fftCannyThresholdHigher,
					minLineLength: libParams.fftMinLineLength,
					maxLineGap: libParams.fftMaxLineGap,
				},
			},
		});
		setTestResult(null);
		setProjectionTestProgress(createDefaultMethodProgress(newTestId, 'Projection'));
		setHoughTestProgress(createDefaultMethodProgress(newTestId, 'Hough'));
		setFftTestProgress(createDefaultMethodProgress(newTestId, 'FFT'));
		Invokers.runTest(newTestId, libParams);
	}, []);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.header}>
					<Button
						variant='contained'
						startIcon={testInfo.testId !== 0 && <CircularProgress size={'1em'} color={'inherit'} />}
						onClick={runTest}
						disabled={testInfo.testId !== 0}
						color={!!testResult ? 'success' : 'primary'}
						fullWidth
					>
						{testInfo.testId === 0 ? `Run Test${!!testResult ? ' Again' : ''}` : 'Test Running'}
					</Button>
				</div>
				<Divider />
				<div className={styles.progressWrapper}>
					<div className={styles.progress}>
						<LinearProgress
							color={projectionTestProgress.status === 'PROCESSED' ? 'success' : 'primary'}
							variant={
								testInfo.testId !== 0 && projectionTestProgress.status === 'WAITING'
									? 'indeterminate'
									: 'determinate'
							}
							value={projectionTestProgress.details.processed_progress * 100}
						/>
					</div>
					<div className={styles.progress}>
						<LinearProgress
							color={houghTestProgress.status === 'PROCESSED' ? 'success' : 'secondary'}
							variant={
								testInfo.testId !== 0 && houghTestProgress.status === 'WAITING'
									? 'indeterminate'
									: 'determinate'
							}
							value={houghTestProgress.details.processed_progress * 100}
						/>
					</div>
					<div className={styles.progress}>
						<LinearProgress
							color={fftTestProgress.status === 'PROCESSED' ? 'success' : 'warning'}
							variant={
								testInfo.testId !== 0 && fftTestProgress.status === 'WAITING'
									? 'indeterminate'
									: 'determinate'
							}
							value={fftTestProgress.details.processed_progress * 100}
						/>
					</div>
				</div>
				<Divider />
				<div className={styles.result}>
					<Grid container spacing={2}>
						{testResult && (
							<>
								<Grid item xs={4}>
									<ResultCard
										test_id={testResult.test_id}
										method='Projection'
										duration={testResult.durations.projection}
										mistakes={testResult.mistakes.projection}
									/>
								</Grid>
								<Grid item xs={4}>
									<ResultCard
										test_id={testResult.test_id}
										method='Hough'
										duration={testResult.durations.hough}
										mistakes={testResult.mistakes.hough}
									/>
								</Grid>
								<Grid item xs={4}>
									<ResultCard
										test_id={testResult.test_id}
										method='FFT'
										duration={testResult.durations.fft}
										mistakes={testResult.mistakes.fft}
									/>
								</Grid>
							</>
						)}
					</Grid>
				</div>
			</div>
		</div>
	);
}

export default App;
