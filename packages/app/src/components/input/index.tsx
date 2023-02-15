import { useMemo } from 'react';
import styles from './index.module.less';

interface IBaseInputProps {
	value: string;
	placeholder?: string;
	onChange?: React.ChangeEventHandler<HTMLInputElement>;
	onPressEnter?: React.KeyboardEventHandler<HTMLInputElement>;
}

/**
 * 文本输入框
 */
interface ITextInputProps extends IBaseInputProps {}
const TextInput: React.FC<ITextInputProps> = (props) => {
	const { placeholder, value, onChange, onPressEnter } = props;

	return (
		<div className={styles.inputWrapper}>
			<div className={styles.inputPrefix}></div>
			<input
				type='text'
				className={styles.input}
				placeholder={placeholder}
				value={value}
				onChange={onChange}
				onKeyDown={(ev) => {
					if (!onPressEnter || ev.key !== 'Enter') return;
					onPressEnter(ev);
				}}
			/>
			<div className={styles.inputSuffix}></div>
		</div>
	);
};

/**
 * 密码输入框
 */
interface IPasswordInputProps extends IBaseInputProps {
	visible: boolean;
	onVisibleChange?: (visible: boolean) => void;
}
const PasswordInput: React.FC<IPasswordInputProps> = (props) => {
	const { placeholder, value, visible, onChange, onVisibleChange, onPressEnter } = props;

	const type = useMemo(() => (visible ? 'text' : 'password'), [visible]);

	return (
		<div className={styles.inputWrapper}>
			<div className={styles.inputPrefix}></div>
			<input
				type={type}
				className={styles.input}
				placeholder={placeholder}
				value={value}
				onChange={onChange}
				onKeyDown={(ev) => {
					if (!onPressEnter || ev.key !== 'Enter') return;
					onPressEnter(ev);
				}}
			/>
			<div className={styles.inputSuffix}></div>
		</div>
	);
};

export interface IInputProps {
	type: 'text' | 'password';
}

const Input: React.FC<IInputProps> = (props) => {
	const { type } = props;

	const InputComponent = useMemo(() => {
		switch (type) {
			case 'text':
				return TextInput;
			default:
				return () => null;
		}
	}, [type]);

	return <InputComponent {...(props as any)} />;
};
