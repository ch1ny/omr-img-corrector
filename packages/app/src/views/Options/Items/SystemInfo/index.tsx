import { CpuIcon } from '@/components/icons';
import useLocalStorage from '@/hooks/useLocalStorage';
import useMount from '@/hooks/useMount';
import { ICpuInfo } from '@/types';
import { getHardwareInfos } from '@/utils';
import { useMemo } from 'react';
import Option from '../../Option';
import styles from './index.module.less';

const SystemInfo = () => {
	const [hardwareInfo, setHardwareInfo] =
		useLocalStorage<Awaited<ReturnType<typeof getHardwareInfos>>>('hardware_info');

	useMount(async () => {
		const awaitedHardwareInfo = await getHardwareInfos();
		setHardwareInfo(awaitedHardwareInfo);
	});

	const cpu = useMemo<ICpuInfo>(
		() =>
			hardwareInfo?.cpu || {
				chipName: '未知芯片组',
				physicalCoreCounts: 1,
				maxThreadCounts: 1,
			},
		[hardwareInfo?.cpu]
	);

	return (
		<div className={styles.systemInfo}>
			<div>
				<CpuIcon />
			</div>
		</div>
	);
};

export default () => (
	<Option title='系统信息'>
		<SystemInfo />
	</Option>
);
