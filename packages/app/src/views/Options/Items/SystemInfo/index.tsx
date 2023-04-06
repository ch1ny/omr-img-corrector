import { CpuIcon } from '@/components';
import useLocalStorage from '@/hooks/useLocalStorage';
import useMount from '@/hooks/useMount';
import { ICpuInfo } from '@/types';
import { getHardwareInfos } from '@/utils';
import { Checkbox, Divider, Input } from '@mui/material';
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

	const [multiThreadOptions, setMultiThreadOptions] = useLocalStorage('use_multi_thread', {
		defaultValue: {
			use: false,
			threadCounts: 1,
		},
	});

	return (
		<div className={styles.systemInfo}>
			<div className={styles.multiThread}>
				<div>
					<CpuIcon
						style={{
							fontSize: '20px',
							display: 'flex',
							justifyContent: 'center',
							alignItems: 'center',
						}}
					/>
				</div>
				<div>
					<Checkbox
						checked={multiThreadOptions.use}
						onChange={(ev) => {
							setMultiThreadOptions({
								...multiThreadOptions,
								use: ev.target.checked,
							});
						}}
					/>
				</div>
				<div>使用多线程</div>
				<div style={{ marginLeft: 'auto' }}>
					<Input
						size='small'
						value={multiThreadOptions.threadCounts}
						disabled={!multiThreadOptions.use}
						inputProps={{
							step: 1,
							min: 1,
							max: cpu.maxThreadCounts,
							type: 'number',
						}}
						onChange={(ev) => {
							setMultiThreadOptions({
								...multiThreadOptions,
								threadCounts: Math.min(parseInt(ev.target.value), cpu.maxThreadCounts),
							});
						}}
					/>
				</div>
				<div style={{ padding: '0 3px', fontSize: '12px' }}>[1-{cpu.maxThreadCounts}]</div>
			</div>
			<Divider />
		</div>
	);
};

export default () => (
	<Option title='系统信息'>
		<SystemInfo />
	</Option>
);
