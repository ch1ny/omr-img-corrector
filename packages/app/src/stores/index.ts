import HardwareStore from './HardwareStore';

export interface IStores {
	hardware: HardwareStore;
}

const stores: IStores = {
	hardware: new HardwareStore(),
};

export default stores;
