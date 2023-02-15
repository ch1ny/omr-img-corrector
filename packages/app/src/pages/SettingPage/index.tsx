import styles from './index.module.less';
import OutDir from './Items/OutDir';
import SystemInfo from './Items/SystemInfo';

export default () => {
	return (
		<div className={styles.setting}>
			<OutDir />
			<SystemInfo />
		</div>
	);
};
