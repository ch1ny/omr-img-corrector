import { useEffect, useMemo, useState } from 'react';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import * as event from '@tauri-apps/api/event';
import styles from './index.module.less';
import { Button, Divider, LinearProgress } from '@mui/material';
import { Invokers } from '@/utils';

export interface ITaskProps {
	id: number;
	src: string;
	omrConfig: {
		outputDir: string;
		projectionMaxAngle: number;
		projectionAngleStep: number;
		projectionResizeScale: number;
		houghMinLineLength: number;
		houghMaxLineGap: number;
	};
}

type TTaskStatus = 'ready' | 'waiting' | 'running' | 'finished' | 'debatable' | 'error';

export default (props: ITaskProps) => {
	const { id, src } = props;
	const previewSrc = useMemo(() => convertFileSrc(src), [src]);

	const [status, setStatus] = useState<TTaskStatus>('ready');

	useEffect(() => {
		const unListenOnTaskRunning = event.listen('start_running_task', (ev) => {
			if (ev.windowLabel !== 'main') return;

			const { taskId } = ev.payload as { taskId: number };
			if (taskId !== id) return;
			setStatus((currentStatus) => {
				if (currentStatus !== 'waiting') return currentStatus;

				return 'running';
			});
		});
		const unListenOnTaskCompleted = event.listen('task_completed', (ev) => {
			if (ev.windowLabel !== 'main') return;
			const { taskId, result } = ev.payload as {
				taskId: number;
				result: 'finished' | 'debatable' | 'error';
			};
			if (taskId !== id) return;
			setStatus((currentStatus) => {
				if (currentStatus !== 'running') return currentStatus;

				return result;
			});
		});

		return () => {
			Promise.all([unListenOnTaskRunning, unListenOnTaskCompleted]).then((unListeners) => {
				unListeners.forEach((unListener) => unListener());
			});
		};
	}, [id]);

	const runTask = useMemo(() => {
		if (status !== 'ready') {
			return () => {
				// TODO:
			};
		} else {
			return () => {
				setStatus('waiting');
				Invokers.addTask(props);
			};
		}
	}, [status, props]);

	/* 管理进度条状态 */
	const progressColor = useMemo(() => {
		switch (status) {
			case 'ready':
			case 'waiting':
			case 'running':
				return 'primary';
			case 'finished':
				return 'success';
			case 'debatable':
				return 'warning';
			case 'error':
				return 'error';
		}
	}, [status]);
	const progressVariant = useMemo(
		() => (status === 'running' ? 'indeterminate' : 'determinate'),
		[status]
	);
	const progressValue = useMemo(() => {
		switch (status) {
			case 'debatable':
			case 'error':
			case 'finished':
				return 100;
			default:
				return 0;
		}
	}, [status]);

	return (
		<>
			<div className={styles.task}>
				<div className={styles.preview}>
					<img src={previewSrc} className={styles.previewImage} />
				</div>
				<div className={styles.status}>
					<LinearProgress
						sx={{ width: '100%' }}
						color={progressColor}
						variant={progressVariant}
						value={progressValue}
					/>
				</div>
				<div className={styles.panel}>
					<div>
						<Button color='success' variant='contained' onClick={runTask}>
							开始
						</Button>
					</div>
				</div>
			</div>
			<Divider />
		</>
	);
};
