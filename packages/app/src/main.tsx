import stores from '@/stores';
import { Provider } from 'mobx-react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import './style.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<BrowserRouter>
		<Provider {...stores}>
			<App />
		</Provider>
	</BrowserRouter>
);
