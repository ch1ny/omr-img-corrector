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

export interface IUseMultiThreadParams {
	use: boolean;
	threadCounts: number;
}

export interface IProjectionParams {
	maxAngle: number;
	angleStep: number;
	maxWidth: number;
	maxHeight: number;
}

export interface IHoughParams {
	minLineLength: number;
	maxLineGap: number;
}

export interface IFftParams {
	cannyThresholdLower: number;
	cannyThresholdHigher: number;
	minLineLength: number;
	maxLineGap: number;
}
