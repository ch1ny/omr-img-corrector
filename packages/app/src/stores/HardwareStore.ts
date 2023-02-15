import { ICpuInfo } from '@/types';
import { action, makeObservable, observable } from 'mobx';

export default class {
	constructor() {
		this.setCpuInfo = this.setCpuInfo.bind(this);

		makeObservable(this, {
			cpu: observable,
			setCpuInfo: action,
		});
	}

	cpu?: ICpuInfo;

	setCpuInfo(cpu: ICpuInfo) {
		this.cpu = cpu;
	}
}
