import useMount from '@/hooks/useMount';
import styles from './App.module.less';
import OutDir from './Items/OutDir';
import SystemInfo from './Items/SystemInfo';
import onAppStart from './onAppStart';

function App() {
	useMount(onAppStart);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.setting}>
					<OutDir />
					<SystemInfo />
				</div>
			</div>
		</div>
	);
}

export default App;
