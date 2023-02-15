import classNames from 'classnames';
import styles from './index.module.less';

export interface ICheckboxProps {
	checked: boolean;
	children: React.ReactNode;
	disabled?: boolean;
	onChange?: (checked: boolean) => void;
}

export const Checkbox: React.FC<ICheckboxProps> = (props) => {
	const { checked, children, disabled = false, onChange } = props;

	return (
		<label
			className={classNames({
				[styles.checkboxWrapper]: true,
				[styles.checkboxWrapperChecked]: checked,
				[styles.checkboxWrapperDisabled]: disabled,
			})}
			onClick={() => {
				if (disabled) return;
				onChange?.(!checked);
			}}
		>
			<div className={styles.checkbox} />
			<div>{children}</div>
		</label>
	);
};
