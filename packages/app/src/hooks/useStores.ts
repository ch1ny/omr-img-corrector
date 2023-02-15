import { IStores } from '@/stores';
import { MobXProviderContext } from 'mobx-react';
import { useContext } from 'react';

export default () => {
	return useContext(MobXProviderContext) as IStores;
};
