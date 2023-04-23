import { useCallback, useEffect, useState } from 'react';

interface IUseLocalStorageOptions<T> {
	serializer?: (value: T) => string;
	deserializer?: (raw: string) => T;
	defaultValue?: T | ((previousState?: T) => T);
}

export default <T>(key: string, options?: IUseLocalStorageOptions<T>) => {
	const { serializer, deserializer, defaultValue } = options || {};

	const getStoredValue = useCallback(() => {
		try {
			const raw = localStorage.getItem(key);
			if (raw) {
				return (deserializer || JSON.parse)(raw);
			}
		} catch (error) {}

		if (typeof defaultValue === 'function') {
			return (<(previousState?: T) => T>defaultValue)();
		}
		return defaultValue;
	}, [key, serializer, deserializer, defaultValue]);

	const [state, setState] = useState<T>(getStoredValue());

	const updateState = useCallback(
		(arg: T | ((prevState: T) => T)) => {
			if (typeof arg === 'function') {
				return setState((prevState: T) => {
					const newState = (<(prevState: T) => T>arg)(prevState);
					localStorage.setItem(key, (serializer || JSON.stringify)(newState));
					return newState;
				});
			}

			localStorage.setItem(key, (serializer || JSON.stringify)(arg));
			return setState(arg);
		},
		[key, serializer]
	);

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
