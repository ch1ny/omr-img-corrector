import classNames from 'classnames';
import React, { lazy, useEffect, useMemo } from 'react';
import { Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import styles from './App.module.less';
import onAppStart from './onAppStart';

const HomePage = lazy(() => import('@/pages/HomePage'));
const SettingPage = lazy(() => import('@/pages/SettingPage'));

interface INavigatorProps {
	title: string;
	target: string;
}

const Navigator: React.FC<INavigatorProps> = (props) => {
	const { title, target } = props;

	const navigate = useNavigate();
	const location = useLocation();

	const isChecked = useMemo(() => target === location.pathname, [target, location]);

	return (
		<div
			className={classNames({
				[styles.navigatorContainer]: true,
				[styles.navigatorContainerChecked]: isChecked,
			})}
			onClick={() => {
				navigate(target);
			}}
		>
			<div></div>
			<div>{title}</div>
		</div>
	);
};

function App() {
	useEffect(() => {
		onAppStart();
	}, []);

	return (
		<div className={styles.app}>
			<div className={styles.navigator}>
				<Navigator title='Home' target='/' />
				<Navigator title='应用设置' target='/setting' />
			</div>
			<div className={styles.content}>
				<React.Suspense fallback={null}>
					<Routes>
						<Route path='/' element={<HomePage />} />
						<Route path='/setting' element={<SettingPage />} />
					</Routes>
				</React.Suspense>
			</div>
		</div>
	);
}

export default App;
