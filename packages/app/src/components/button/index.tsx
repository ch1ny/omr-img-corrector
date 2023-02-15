import classNames from 'classnames';
import React from 'react';
import styles from './index.module.less';

export interface IButtonProps {
	children?: React.ReactNode;
}

export const Button: React.FC<IButtonProps> = (props) => {
	const { children } = props;

	return (
		<button
			className={classNames({
				[styles.button]: true,
			})}
		>
			<span></span>
			<span>{children}</span>
		</button>
	);
};
