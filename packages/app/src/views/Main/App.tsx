import useMount from '@/hooks/useMount';
import { Invokers } from '@/utils';
import Button from '@mui/material/Button';
import styles from './App.module.less';
import onAppStart from './onAppStart';

function App() {
	useMount(onAppStart);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<Button variant='contained' disableElevation onClick={Invokers.showSettingsWindow}>
					设置
				</Button>
			</div>
		</div>
	);
}

export default App;
