import { DownIcon } from '@/components';
import useLocalStorage from '@/hooks/useLocalStorage';
import { IFftParams, IHoughParams, IProjectionParams } from '@/types';
import {
	Accordion,
	AccordionDetails,
	AccordionSummary,
	Divider,
	Input,
	Slider,
	Typography,
} from '@mui/material';
import styles from './index.module.less';

const ProjectionParams = () => {
	const [projectionParams, setProjectionParams] = useLocalStorage<IProjectionParams>(
		'projection_params',
		{
			defaultValue: {
				maxAngle: 45,
				angleStep: 0.2,
				imageResizeScale: 0.2,
			},
		}
	);

	return (
		<div className={styles.projectionParams}>
			<Accordion defaultExpanded={true} style={{ backgroundColor: '#fafafa' }}>
				<AccordionSummary expandIcon={<DownIcon />}>投影标准差方案参数</AccordionSummary>
				<Divider />
				<AccordionDetails style={{ paddingTop: '16px' }}>
					<div className={styles.param}>
						<Typography variant='body2'>
							角度搜索范围: 【 - {projectionParams.maxAngle}° ~ {projectionParams.maxAngle}° 】
						</Typography>
						<Slider
							value={projectionParams.maxAngle}
							size='small'
							step={1}
							min={1}
							max={45}
							onChange={(_, value) => {
								setProjectionParams({
									...projectionParams,
									maxAngle: value as number,
								});
							}}
						/>
					</div>
					<Divider />
					<div className={styles.param}>
						<Typography variant='body2'>搜索步进值: 【 {projectionParams.angleStep}° 】</Typography>
						<Slider
							value={projectionParams.angleStep}
							size='small'
							step={0.1}
							min={0.1}
							max={1.0}
							onChange={(_, value) => {
								setProjectionParams({
									...projectionParams,
									angleStep: value as number,
								});
							}}
						/>
					</div>
					<Divider />
					<div className={styles.param}>
						<Typography variant='body2'>
							图像缩放比: 【 {projectionParams.imageResizeScale} 】
						</Typography>
						<Typography variant='caption' sx={{ color: '#8d8d8d' }}>
							更高的缩放比会带来更高的精确度，也会带来更长的运行时间
						</Typography>
						<Slider
							value={projectionParams.imageResizeScale}
							size='small'
							step={0.01}
							min={0.01}
							max={1.0}
							onChange={(_, value) => {
								setProjectionParams({
									...projectionParams,
									imageResizeScale: value as number,
								});
							}}
						/>
					</div>
				</AccordionDetails>
			</Accordion>
		</div>
	);
};

const HoughParams = () => {
	const [houghParams, setHoughParams] = useLocalStorage<IHoughParams>('hough_params', {
		defaultValue: {
			minLineLength: 125.0,
			maxLineGap: 15.0,
		},
	});

	return (
		<div className={styles.houghParams}>
			<Accordion defaultExpanded={true} style={{ backgroundColor: '#fafafa' }}>
				<AccordionSummary expandIcon={<DownIcon />}>霍夫变换方案参数</AccordionSummary>
				<Divider />
				<AccordionDetails style={{ paddingTop: '16px' }}>
					<div className={styles.param} style={{ display: 'flex' }}>
						<Typography variant='body2'>可感知线段最小长度</Typography>
						<div style={{ marginLeft: 'auto' }}>
							<Input
								size='small'
								value={houghParams.minLineLength}
								inputProps={{
									step: 0.1,
									min: 1,
									type: 'number',
								}}
								sx={{ width: '5em' }}
								onChange={(ev) => {
									setHoughParams({
										...houghParams,
										minLineLength: Math.max(parseFloat(ev.target.value), 1),
									});
								}}
							/>
						</div>
					</div>
					<Divider />
					<div className={styles.param} style={{ display: 'flex' }}>
						<Typography variant='body2'>感知线段最大隔断</Typography>
						<div style={{ marginLeft: 'auto' }}>
							<Input
								size='small'
								value={houghParams.maxLineGap}
								inputProps={{
									step: 0.1,
									min: 1,
									type: 'number',
								}}
								sx={{ width: '5em' }}
								onChange={(ev) => {
									setHoughParams({
										...houghParams,
										maxLineGap: Math.max(parseFloat(ev.target.value), 1),
									});
								}}
							/>
						</div>
					</div>
				</AccordionDetails>
			</Accordion>
		</div>
	);
};

const FftParams = () => {
	const [fftParams, setFftParams] = useLocalStorage<IFftParams>('fft_params', {
		defaultValue: {
			minLineLength: 125.0,
			maxLineGap: 15.0,
		},
	});

	return (
		<div className={styles.fftParams}>
			<Accordion defaultExpanded={true} style={{ backgroundColor: '#fafafa' }}>
				<AccordionSummary expandIcon={<DownIcon />}>快速傅里叶变换方案参数</AccordionSummary>
				<Divider />
				<AccordionDetails style={{ paddingTop: '16px' }}>
					<div className={styles.param} style={{ display: 'flex' }}>
						<Typography variant='body2'>可感知线段最小长度</Typography>
						<div style={{ marginLeft: 'auto' }}>
							<Input
								size='small'
								value={fftParams.minLineLength}
								inputProps={{
									step: 0.1,
									min: 1,
									type: 'number',
								}}
								sx={{ width: '5em' }}
								onChange={(ev) => {
									setFftParams({
										...fftParams,
										minLineLength: Math.max(parseFloat(ev.target.value), 1),
									});
								}}
							/>
						</div>
					</div>
					<Divider />
					<div className={styles.param} style={{ display: 'flex' }}>
						<Typography variant='body2'>感知线段最大隔断</Typography>
						<div style={{ marginLeft: 'auto' }}>
							<Input
								size='small'
								value={fftParams.maxLineGap}
								inputProps={{
									step: 0.1,
									min: 1,
									type: 'number',
								}}
								sx={{ width: '5em' }}
								onChange={(ev) => {
									setFftParams({
										...fftParams,
										maxLineGap: Math.max(parseFloat(ev.target.value), 1),
									});
								}}
							/>
						</div>
					</div>
				</AccordionDetails>
			</Accordion>
		</div>
	);
};

export default () => {
	return (
		<div className={styles.paramsOption}>
			<ProjectionParams />
			<Divider style={{ marginBlock: '16px' }} />
			<HoughParams />
			<Divider style={{ marginBlock: '16px' }} />
			<FftParams />
		</div>
	);
};
