import stores from './stores';
import { Invokers } from './utils';

export default async () => {
	{
		const cpuInfo = await Invokers.getCpuInfo();
		stores.hardware.setCpuInfo(cpuInfo);
	}

	Invokers.showMainWindow();
};
