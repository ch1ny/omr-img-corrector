export interface ICpuInfo {
	/**
	 * 芯片名称
	 */
	chipName: string;
	/**
	 * 物理核心数
	 */
	physicalCoreCounts: number;
	/**
	 * 最大支持线程数
	 */
	maxThreadCounts: number;
}

export interface IGpuInfo {
	/**
	 * 显卡型号
	 */
	gpuName: string;
}

export interface ISystemHardwareInfo {
	cpu: ICpuInfo;
	gpu: IGpuInfo;
}
