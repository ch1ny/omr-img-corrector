import { useMemo, useState } from 'react';
import styles from './index.module.less';

interface ITaskProps {
	/**
	 * 源文件路径
	 */
	src: string;
	/**
	 * 输出文件路径
	 */
	out: string;
}

type TTaskStatus = 'waiting' | 'running' | 'finished' | 'error';

export default (props: ITaskProps) => {
	const { src, out } = props;

	const [status, setStatus] = useState<TTaskStatus>('waiting');
	const [progress, setProgress] = useState(0.0);

	const runTask = useMemo(() => {
		if (status !== 'waiting') {
			return () => {
				// TODO:
			};
		} else {
			return () => {
				setStatus('running');
			};
		}
	}, [status]);

	return (
		<div className={styles.task}>
			<div className={styles.preview}></div>
			<div className={styles.status}></div>
		</div>
	);
};
