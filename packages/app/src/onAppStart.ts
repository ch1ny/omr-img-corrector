import stores from './stores';
import { Invokers } from './utils';

export default async () => {
	{
		const hardwareInfo = await Invokers.getSystemHardwareInfo();
		stores.hardware.setCpuInfo(hardwareInfo.cpu);
	}

	Invokers.showMainWindow();
};
