/**
 * 禁用页面右键菜单
 * @returns 解除禁用
 */
export const disableWebviewContextMenu = () => {
	const disableHandler = (ev: MouseEvent) => {
		ev.preventDefault();
	};
	document.addEventListener('contextmenu', disableHandler);

	return () => {
		document.removeEventListener('contextmenu', disableHandler);
	};
};

/**
 * 禁用页面刷新
 * @returns 解除禁用
 * @deprecated
 */
export const disableWebviewRefresh = () => {
	const disableHandler = (ev: BeforeUnloadEvent) => {
		ev.preventDefault();
	};
	window.addEventListener('beforeunload', disableHandler);

	return () => {
		window.removeEventListener('beforeunload', disableHandler);
	};
};

/**
 * 禁用页面打印
 * @returns 解除禁用
 * @deprecated
 */
export const disableWebviewPrint = () => {
	const disableHandler = (ev: Event) => {
		ev.preventDefault();
	};
	window.addEventListener('beforeprint', disableHandler);

	return () => {
		window.removeEventListener('beforeprint', disableHandler);
	};
};
