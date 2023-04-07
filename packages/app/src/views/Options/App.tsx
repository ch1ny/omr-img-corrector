import useMount from '@/hooks/useMount';
import { useMemo, useState } from 'react';
import styles from './App.module.less';
import OutDir from './Items/OutDir';
import SystemInfo from './Items/SystemInfo';
import onAppStart from './onAppStart';
import Option from './Option';

type TOptionKey = 'OutDir' | 'SystemInfo';

type TOptionItem = {
	key: TOptionKey;
	title: string;
	element?: React.ReactNode;
	subtitle?: string;
};
const renderOptionsFromTemplate = (
	template: TOptionItem[],
	expanded: Record<TOptionKey, boolean>,
	setExpanded: React.Dispatch<React.SetStateAction<Record<TOptionKey, boolean>>>
) => {
	return template.map(({ key, element, title, subtitle }) => (
		<Option
			title={title}
			subtitle={subtitle}
			expanded={expanded[key]}
			onChange={(_, exp) =>
				setExpanded({
					...expanded,
					[key]: exp,
				})
			}
			key={key}
		>
			{element}
		</Option>
	));
};

function App() {
	useMount(onAppStart);

	const [expanded, setExpanded] = useState<Record<TOptionKey, boolean>>({
		OutDir: true,
		SystemInfo: true,
	});

	const renderedOptions = useMemo(
		() =>
			renderOptionsFromTemplate(
				[
					{
						key: 'OutDir',
						title: '输出文件夹',
						element: <OutDir />,
					},
					{
						key: 'SystemInfo',
						title: '系统信息',
						element: <SystemInfo />,
					},
				],
				expanded,
				setExpanded
			),
		[expanded]
	);

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.setting}>{renderedOptions}</div>
			</div>
		</div>
	);
}

export default App;
