import { FireIcon, SettingIcon } from '@/components';
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
			</div>
		</div>
	);
}

export default App;
