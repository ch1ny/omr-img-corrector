import { useEffect } from 'react';

export default (cb: () => void | Promise<void>) => {
	return useEffect(() => {
		cb();
	}, []);
};
