import { SettingIcon } from '@/components';
import useMount from '@/hooks/useMount';
import { Invokers } from '@/utils';
import { Button, Divider } from '@mui/material';
import styles from './App.module.less';
import onAppStart from './onAppStart';

function App() {
	useMount(onAppStart);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.header}>
					<Button
						variant='outlined'
						startIcon={<SettingIcon />}
						onClick={Invokers.showSettingsWindow}
					>
						设置
					</Button>
				</div>
				<Divider />
			</div>
		</div>
	);
}

export default App;
