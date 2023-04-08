import useMount from '@/hooks/useMount';
import { restoreAppParams } from '@/utils';
import { Button, Divider } from '@mui/material';
import { useMemo } from 'react';
import styles from './App.module.less';
import OutDir from './Items/OutDir';
import Params from './Items/Params';
import SystemInfo from './Items/SystemInfo';
import onAppStart from './onAppStart';
import Option from './Option';

type TOptionKey = 'OutDir' | 'SystemInfo' | 'Params';

type TOptionItem = {
	key: TOptionKey;
	title: string;
	element?: React.ReactNode;
	subtitle?: string;
};
const renderOptionsFromTemplate = (template: TOptionItem[]) => {
	return template.map(({ key, element, title }) => (
		<Option title={title} key={key}>
			{element}
		</Option>
	));
};

function App() {
	useMount(onAppStart);

	const renderedOptions = useMemo(
		() =>
			renderOptionsFromTemplate([
				{
					key: 'OutDir',
					title: '输出文件夹',
					element: <OutDir />,
				},
				{
					key: 'SystemInfo',
					title: '系统信息',
					element: <SystemInfo />,
				},
				{
					key: 'Params',
					title: '参数配置',
					element: <Params />,
				},
			]),
		[]
	);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.setting}>
					{renderedOptions}
					<div style={{ width: '100%' }}>
						<Divider style={{ marginBottom: '16px' }} />
						<div
							style={{
								display: 'flex',
								flexDirection: 'column',
								alignItems: 'center',
							}}
						>
							<div>
								<Button variant='contained' color='error' onClick={restoreAppParams}>
									重置应用参数
								</Button>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}

export default App;
