import { DownIcon } from '@/components';
import { Accordion, AccordionDetails, AccordionSummary, Divider, Typography } from '@mui/material';
import React from 'react';
import styles from './Option.module.less';

interface IOptionProps {
	expanded: boolean;
	title: string;
	subtitle?: string;
	onChange?: (event: React.SyntheticEvent, expanded: boolean) => void;
	children?: React.ReactNode;
}

const Option: React.FC<IOptionProps> = (props) => {
	const { expanded, title, subtitle, onChange, children } = props;

	return (
		<Accordion expanded={expanded} onChange={onChange} style={{ width: '100%' }}>
			<AccordionSummary expandIcon={<DownIcon />}>
				<Typography sx={{ width: '33%', flexShrink: 0 }}>{title}</Typography>
				<Typography sx={{ color: 'text.secondary' }}>{subtitle}</Typography>
			</AccordionSummary>
			<Divider />
			<AccordionDetails>
				<div className={styles.content}>{children}</div>
			</AccordionDetails>
		</Accordion>
	);
};

export default Option;
