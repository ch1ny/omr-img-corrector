import { invoke } from '@tauri-apps/api';
import { useEffect } from 'react';
import styles from './App.module.less';

function App() {
	useEffect(() => {
		invoke('show_main_window');
	}, []);

	return (
		<div className={styles.app}>
			<h1>Welcome to Tauri!</h1>
		</div>
	);
}

export default App;
