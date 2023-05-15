import { useCallback, useEffect, useState } from 'react';

interface IUseLocalStorageOptions<T> {
	serializer?: (value: T) => string; // 序列化函数
	deserializer?: (raw: string) => T; // 解序列化函数
	defaultValue?: T | ((previousState?: T) => T); // 默认值
}

export default <T>(key: string, options?: IUseLocalStorageOptions<T>) => {
	const { serializer, deserializer, defaultValue } = options || {};

	// 定义从 localStorage 中取出对应键的值的函数
	const getStoredValue = useCallback(() => {
		try {
			// 从 localStorage 中取出的字符串
			const raw = localStorage.getItem(key);
			if (raw) {
				// 默认使用 JSON.parse 进行序列化
				return (deserializer || JSON.parse)(raw);
			}
		} catch (error) {}

		if (typeof defaultValue === 'function') {
			// 默认值传入为函数时立即执行，以返回值作为默认值的值
			return (<(previousState?: T) => T>defaultValue)();
		}
		return defaultValue;
	}, [key, serializer, deserializer, defaultValue]);

	// hook 内部维持状态
	const [state, setState] = useState<T>(getStoredValue());

	// 定义更新状态函数
	const updateState = useCallback(
		(arg: T | ((prevState: T) => T)) => {
			if (typeof arg === 'function') {
				return setState((prevState: T) => {
					const newState = (<(prevState: T) => T>arg)(prevState);
					localStorage.setItem(key, (serializer || JSON.stringify)(newState));
					return newState;
				});
			}

			// 更新状态时同时更新 localStorage 的值
			localStorage.setItem(key, (serializer || JSON.stringify)(arg));
			return setState(arg);
		},
		[key, serializer]
	);

	// 通过监听 window 对象的 storage 事件实现监听其他窗口改动 localStorage 的值, 以此实现简单的跨窗口通信
	useEffect(() => {
		const listener = (ev: StorageEvent) => {
			const { key: evKey, newValue } = ev;
			if (evKey !== key) return;

			if (newValue === null) {
				setState(typeof defaultValue === 'function' ? (<() => T>defaultValue)() : <T>defaultValue);
			} else {
				setState((deserializer || JSON.parse)(newValue));
			}
		};

		window.addEventListener('storage', listener);
		return () => window.removeEventListener('storage', listener);
	}, [key, defaultValue, deserializer]);

	return [state, updateState] as [T, React.Dispatch<React.SetStateAction<T>>];
};
