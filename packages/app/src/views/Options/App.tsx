import useMount from '@/hooks/useMount';
import { Accordion, AccordionDetails, AccordionSummary, Typography } from '@mui/material';
import { useState } from 'react';
import styles from './App.module.less';
import OutDir from './Items/OutDir';
import onAppStart from './onAppStart';

type TOptionKey = 'OutDir' | 'SystemInfo';

function App() {
	useMount(onAppStart);

	const [expanded, setExpanded] = useState<Record<TOptionKey, boolean>>({
		OutDir: false,
		SystemInfo: false,
	});

	return (
		<div className={styles.app}>
			<div className={styles.content}>
				<div className={styles.setting}>
					<Accordion
						expanded={expanded.OutDir}
						onChange={(_, exp) =>
							setExpanded({
								...expanded,
								OutDir: exp,
							})
						}
						style={{ width: '100%' }}
					>
						<AccordionSummary aria-controls='panel1bh-content' id='panel1bh-header'>
							<Typography sx={{ width: '33%', flexShrink: 0 }}>General settings</Typography>
							<Typography sx={{ color: 'text.secondary' }}>I am an accordion</Typography>
						</AccordionSummary>
						<AccordionDetails>
							<OutDir />
						</AccordionDetails>
					</Accordion>
				</div>
			</div>
		</div>
	);
}

export default App;
