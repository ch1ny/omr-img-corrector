import React from 'react';
import styles from './Option.module.less';

interface IOptionProps {
	title: string;
	children?: React.ReactNode;
}

const Option: React.FC<IOptionProps> = (props) => {
	const { title, children } = props;

	return (
		<div className={styles.option}>
			<div className={styles.optionTitle}>{title}</div>
			<div className={styles.optionContent}>{children}</div>
		</div>
	);
};

export default Option;
