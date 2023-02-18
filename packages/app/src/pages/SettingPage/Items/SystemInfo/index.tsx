import useStores from '@/hooks/useStores';
import { ICpuInfo } from '@/types';
import { useMemo } from 'react';
import Option from '../../Option';
import styles from './index.module.less';

const SystemInfo = () => {
	const stores = useStores();

	const cpu = useMemo<ICpuInfo>(
		() =>
			stores.hardware.cpu || {
				chipName: '未知芯片组',
				physicalCoreCounts: 1,
				maxThreadCounts: 1,
			},
		[stores.hardware.cpu]
	);

	console.log(stores.hardware);

	return (
		<div className={styles.systemInfo}>
			<div></div>
		</div>
	);
};

export default () => (
	<Option title='系统信息'>
		<SystemInfo />
	</Option>
);
