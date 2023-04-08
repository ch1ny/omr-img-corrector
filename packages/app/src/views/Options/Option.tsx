import { Typography } from '@mui/material';
import React from 'react';
import styles from './Option.module.less';

interface IOptionProps {
	title: string;
	children?: React.ReactNode;
}

const Option: React.FC<IOptionProps> = (props) => {
	const { title, children } = props;

	return (
		<div className={styles.options}>
			<Typography variant='h6' sx={{ width: '66%', flexShrink: 0 }}>
				{title}
			</Typography>
			<div className={styles.content}>{children}</div>
		</div>
	);
};

export default Option;
