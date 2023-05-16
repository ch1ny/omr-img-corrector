import { forwardRef, useCallback, useEffect, useImperativeHandle, useMemo, useState } from 'react';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import * as event from '@tauri-apps/api/event';
import styles from './index.module.less';
import { Chip, CircularProgress, Divider, LinearProgress } from '@mui/material';
import { Invokers } from '@/utils';
import { CaretRightIcon, CheckIcon, CloseIcon, DeleteIcon, ExclamationIcon } from '@/components';
import openModifyWindow from '../openModifyWindow';

export interface ITaskProps {
	id: number;
	src: string;
	omrConfig: {
		outputDir: string;
		projectionMaxAngle: number;
		projectionAngleStep: number;
		projectionMaxWidth: number;
		projectionMaxHeight: number;
		houghMinLineLength: number;
		houghMaxLineGap: number;
	};
}

type TTaskStatus = 'ready' | 'waiting' | 'running' | 'finished' | 'debatable' | 'error';

export interface ITaskRef {
	runTask: () => void;
}

interface ITaskComponentProps extends ITaskProps {
	onDelete?: React.MouseEventHandler<HTMLDivElement>;
}

export default forwardRef<ITaskRef, ITaskComponentProps>((props, ref) => {
	const { id, src, onDelete } = props;
	const previewSrc = useMemo(() => convertFileSrc(src), [src]);

	const [status, setStatus] = useState<TTaskStatus>('ready');

	const [outputPath, setOutputPath] = useState('');
	const onDebate = useCallback(() => {
		openModifyWindow(id, outputPath);
	}, [id, outputPath]);

	useEffect(() => {
		const unListenOnTaskRunning = event.listen('start_running_task', (ev) => {
			// console.log(ev);
			if (ev.windowLabel !== 'main') return;

			const { task_id } = ev.payload as { task_id: number };
			if (task_id !== id) return;
			setStatus((currentStatus) => {
				if (currentStatus !== 'waiting') return currentStatus;

				return 'running';
			});
		});
		const unListenOnTaskCompleted = event.listen('task_completed', (ev) => {
			// console.log(ev);
			if (ev.windowLabel !== 'main') return;
			const { task_id, result, output_path } = ev.payload as {
				task_id: number;
				result: 'finished' | 'debatable' | 'error';
				output_path: string;
			};
			if (task_id !== id) return;
			setStatus((currentStatus) => {
				if (currentStatus !== 'running') return currentStatus;
				setOutputPath(output_path);
				return result;
			});
		});

		return () => {
			Promise.all([unListenOnTaskRunning, unListenOnTaskCompleted]).then((unListeners) => {
				unListeners.forEach((unListener) => unListener());
			});
		};
	}, [id]);

	const runTask = useCallback(() => {
		setStatus((oldStatus) => {
			if (oldStatus !== 'ready') return oldStatus;

			Invokers.addTask(props);
			return 'waiting';
		});
	}, [props]);
	useImperativeHandle(
		ref,
		() => ({
			runTask,
		}),
		[runTask]
	);

	/* 管理进度条状态 */
	const progressColor = useMemo(() => {
		switch (status) {
			case 'ready':
				return 'secondary';
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

	/* 管理 Chip 状态 */
	const [chipLabel, chipIcon] = useMemo(() => {
		switch (status) {
			case 'debatable':
				return [
					'待确认',
					<div style={{ fontSize: '1.25em' }}>
						<ExclamationIcon />
					</div>,
				];
			case 'error':
				return [
					'已失败',
					<div style={{ fontSize: '1.25em' }}>
						<CloseIcon />
					</div>,
				];
			case 'finished':
				return [
					'已完成',
					<div style={{ fontSize: '1.25em' }}>
						<CheckIcon />
					</div>,
				];
			case 'ready':
				return [
					'已就绪',
					<div style={{ fontSize: '1.25em' }}>
						<CaretRightIcon />
					</div>,
				];
			case 'running':
				return ['处理中', <CircularProgress size={'1.25em'} color={'primary'} />];
			case 'waiting':
				return ['等待中', <CircularProgress size={'1.25em'} color={'primary'} />];
		}
	}, [status]);

	return (
		<>
			<div className={styles.task}>
				<div className={styles.preview}>
					<img src={previewSrc} className={styles.previewImage} />
				</div>
				<div className={styles.status}>
					<div
						style={{
							width: '100%',
							display: 'flex',
							justifyContent: 'space-evenly',
							fontSize: '0.75em',
							fontWeight: 'bold',
						}}
					>
						<div>{src}</div>
					</div>
					<div style={{ width: '100%' }}>
						<LinearProgress
							sx={{ width: '100%' }}
							color={progressColor}
							variant={progressVariant}
							value={progressValue}
						/>
					</div>
				</div>
				<div className={styles.panel}>
					<div
						style={{ cursor: status === 'ready' || status === 'debatable' ? 'pointer' : 'auto' }}
					>
						<Chip
							label={chipLabel}
							color={progressColor}
							clickable={status === 'ready' || status === 'debatable'}
							icon={chipIcon}
							onClick={status === 'debatable' ? onDebate : runTask}
						/>
					</div>
					<div>
						<div className={styles.delete} onClick={onDelete}>
							<DeleteIcon />
						</div>
					</div>
				</div>
			</div>
			<Divider />
		</>
	);
});
