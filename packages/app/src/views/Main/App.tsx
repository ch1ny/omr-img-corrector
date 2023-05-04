import { CaretRightIcon, FileImageIcon, FireIcon, FolderAddIcon, SettingIcon } from '@/components';
import useMount from '@/hooks/useMount';
import { getLibParams, Invokers } from '@/utils';
import { Button, Divider } from '@mui/material';
import * as fs from '@tauri-apps/api/fs';
import * as dialog from '@tauri-apps/api/dialog';
import { createRef, useCallback, useEffect, useState } from 'react';
import styles from './App.module.less';
import onAppStart from './onAppStart';
import Task, { ITaskProps, ITaskRef } from './Task';
import useLocalStorage from '@/hooks/useLocalStorage';
import { IUseMultiThreadParams } from '@/types';

/**
 * 及时更新主进程的多线程配置
 */
const useMultiThread = () => {
	const [multiThreadOptions] = useLocalStorage<IUseMultiThreadParams>('use_multi_thread', {
		defaultValue: {
			use: false,
			threadCounts: 1,
		},
	});
	useEffect(() => {
		Invokers.setThreadCounts(multiThreadOptions.use ? multiThreadOptions.threadCounts : 1);
	}, [multiThreadOptions]);
};

function App() {
	useMount(onAppStart);
	useMultiThread();

	const [tasks, setTasks] = useState<Array<ITaskProps & { ref: React.RefObject<ITaskRef> }>>([]);

	const addTasks = useCallback((newTasks: ITaskProps[]) => {
		setTasks((oldTasks) => [
			...oldTasks,
			...newTasks.map((t) => ({
				...t,
				ref: createRef<ITaskRef>(),
			})),
		]);
	}, []);
	const importFileTask = useCallback(async () => {
		let selected = await dialog.open({
			multiple: true,
			filters: [
				{
					name: 'Image',
					extensions: ['png', 'jpg', 'jpeg'],
				},
			],
		});
		if (!selected) return;

		if (!Array.isArray(selected)) {
			selected = [selected];
		}

		const {
			outputDir,
			projectionMaxAngle,
			projectionAngleStep,
			projectionMaxWidth,
			projectionMaxHeight,
			houghMinLineLength,
			houghMaxLineGap,
		} = getLibParams();
		const taskGroupId = Date.now();
		const newTasks: ITaskProps[] = selected.map((src, idx) => ({
			id: taskGroupId + idx,
			src,
			omrConfig: {
				outputDir,
				projectionMaxAngle,
				projectionAngleStep,
				projectionMaxWidth,
				projectionMaxHeight,
				houghMinLineLength,
				houghMaxLineGap,
			},
		}));
		addTasks(newTasks);
	}, []);
	const importFolderTask = useCallback(async () => {
		const selected = await dialog.open({
			multiple: false,
			directory: true,
		});
		if (!selected) return;

		const {
			outputDir,
			projectionMaxAngle,
			projectionAngleStep,
			projectionMaxWidth,
			projectionMaxHeight,
			houghMinLineLength,
			houghMaxLineGap,
		} = getLibParams();
		const taskGroupId = Date.now();
		const newTasks: ITaskProps[] = (await fs.readDir(selected as string))
			.filter(
				({ path }) => path.endsWith('.png') || path.endsWith('.jpeg') || path.endsWith('.jpg')
			)
			.map(({ path: src }, idx) => ({
				id: taskGroupId + idx,
				src,
				omrConfig: {
					outputDir,
					projectionMaxAngle,
					projectionAngleStep,
					projectionMaxWidth,
					projectionMaxHeight,
					houghMinLineLength,
					houghMaxLineGap,
				},
			}));
		addTasks(newTasks);
	}, []);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.header}>
					<div className={styles.headerButton}>
						<Button
							variant='outlined'
							startIcon={<FileImageIcon />}
							onClick={importFileTask}
							title={'导入图片'}
						>
							导入
						</Button>
					</div>
					<div className={styles.headerButton}>
						<Button
							variant='outlined'
							startIcon={<FolderAddIcon />}
							onClick={importFolderTask}
							title={'导入文件夹'}
						>
							导入
						</Button>
					</div>
					<div className={styles.headerButton} style={{ marginLeft: 'auto' }}>
						<Button
							variant='outlined'
							startIcon={<CaretRightIcon />}
							onClick={() =>
								tasks.forEach(({ ref }) => {
									ref.current?.runTask();
								})
							}
						>
							开始
						</Button>
					</div>
					<div className={styles.headerButton}>
						<Button
							variant='outlined'
							startIcon={<SettingIcon />}
							onClick={() => Invokers.requestWindowShow('settings')}
						>
							设置
						</Button>
					</div>
					<div className={styles.headerButton}>
						<Button
							variant='outlined'
							startIcon={<FireIcon />}
							onClick={() => Invokers.requestWindowShow('test')}
						>
							测试
						</Button>
					</div>
				</div>
				<Divider />
				<div className={styles.main}>
					{tasks.map(({ id, src, omrConfig, ref }, idx) => (
						<Task
							id={id}
							src={src}
							omrConfig={omrConfig}
							key={id}
							ref={ref}
							onDelete={() => {
								setTasks((oldTasks) => [...oldTasks.slice(0, idx), ...oldTasks.slice(idx + 1)]);
							}}
						/>
					))}
				</div>
			</div>
		</div>
	);
}

export default App;
