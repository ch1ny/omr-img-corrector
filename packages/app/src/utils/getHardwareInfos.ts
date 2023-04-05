import { Invokers } from './invokers';

/**
 * 获取硬件信息
 * @returns
 */
export const getHardwareInfos = async () => {
	const gpuInfo = getGPUInfo();

	return {
		cpu: await Invokers.getCpuInfo(),
		gpu: gpuInfo,
	};
};

interface IGpuInfo {
	renderer: string;
	vendor: string;
}

let cachedGPUInfo: IGpuInfo;

/**
 * 获取显卡信息
 * @returns
 */
const getGPUInfo = () => {
	if (!!cachedGPUInfo) return cachedGPUInfo;

	const cvs = document.createElement('canvas');
	const gl = cvs.getContext('webgl');

	const unmaskedInfo = {
		renderer: '',
		vendor: '',
	};

	const dbgRendererInfo = gl?.getExtension('WEBGL_debug_renderer_info');
	if (!dbgRendererInfo) return unmaskedInfo;

	unmaskedInfo.renderer =
		gl?.getParameter(dbgRendererInfo.UNMASKED_RENDERER_WEBGL) || unmaskedInfo.renderer;
	unmaskedInfo.vendor =
		gl?.getParameter(dbgRendererInfo.UNMASKED_VENDOR_WEBGL) || unmaskedInfo.vendor;

	cachedGPUInfo = unmaskedInfo;
	return unmaskedInfo;
};
