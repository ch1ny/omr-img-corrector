import { ICpuInfo, IGpuInfo } from '@/types';
import { getGPUInfo } from '@/utils';
import { action, makeObservable, observable } from 'mobx';

export default class {
	constructor() {
		this.gpu = {
			gpuName: getGPUInfo().renderer,
		};

		this.setCpuInfo = this.setCpuInfo.bind(this);

		makeObservable(this, {
			cpu: observable,
			setCpuInfo: action,
			gpu: observable,
		});
	}

	cpu?: ICpuInfo;
	setCpuInfo(cpu: ICpuInfo) {
		this.cpu = cpu;
	}

	gpu: IGpuInfo;
}
