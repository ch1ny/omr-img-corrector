import Option from '../../Option';
import styles from './index.module.less';

const OutDir = () => {
	return (
		<div className={styles.outDir}>
			<div className={styles.outDirPath}></div>
		</div>
	);
};

export default () => (
	<Option title='输出文件夹'>
		<OutDir />
	</Option>
);
