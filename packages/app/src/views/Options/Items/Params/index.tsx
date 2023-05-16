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
import { useCallback } from 'react';
import styles from './index.module.less';

const ProjectionParams = () => {
	const [projectionParams, setProjectionParams] = useLocalStorage<IProjectionParams>(
		'projection_params',
		{
			defaultValue: {
				maxAngle: 45,
				angleStep: 0.2,
				maxWidth: 248,
				maxHeight: 230,
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
						<Typography variant='body2'>图像最大尺寸：</Typography>
						<Typography variant='caption' sx={{ color: '#8d8d8d' }}>
							图像尺寸越大会带来更高的精确度，也会带来更长的运行时间
						</Typography>
						<br />
						<Typography variant='caption' sx={{ color: '#8d8d8d' }}>
							输入为0时表示不对图像尺寸做限制
						</Typography>
						<br />
						<Input
							size='small'
							value={projectionParams.maxWidth}
							inputProps={{
								step: 1,
								min: 0,
								type: 'number',
							}}
							onChange={(ev) => {
								setProjectionParams({
									...projectionParams,
									maxWidth: Math.max(parseInt(ev.target.value || '0'), 0),
								});
							}}
						/>
						<Input
							size='small'
							value={projectionParams.maxHeight}
							inputProps={{
								step: 1,
								min: 0,
								type: 'number',
							}}
							onChange={(ev) => {
								setProjectionParams({
									...projectionParams,
									maxHeight: Math.max(parseInt(ev.target.value || '0'), 0),
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
			cannyThresholdLower: 125.0,
			cannyThresholdHigher: 150.0,
			minLineLength: 150.0,
			maxLineGap: 75.0,
		},
	});

	const updateCannyThresholdLower = useCallback(
		(ev: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
			setFftParams((fftParams) => ({
				...fftParams,
				cannyThresholdLower: Math.min(
					Math.max(parseFloat(ev.target.value), 0.0),
					fftParams.cannyThresholdHigher
				),
			}));
		},
		[]
	);
	const updateCannyThresholdHigher = useCallback(
		(ev: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
			setFftParams((fftParams) => ({
				...fftParams,
				cannyThresholdHigher: Math.max(
					Math.min(parseFloat(ev.target.value), 255.0),
					fftParams.cannyThresholdLower
				),
			}));
		},
		[]
	);
	const updateMinLineLength = useCallback(
		(ev: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
			setFftParams((fftParams) => ({
				...fftParams,
				minLineLength: Math.max(parseFloat(ev.target.value), 1),
			}));
		},
		[]
	);
	const updateMaxLineGap = useCallback(
		(ev: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
			setFftParams((fftParams) => ({
				...fftParams,
				maxLineGap: Math.max(parseFloat(ev.target.value), 1),
			}));
		},
		[]
	);

	return (
		<div className={styles.fftParams}>
			<Accordion defaultExpanded={true} style={{ backgroundColor: '#fafafa' }}>
				<AccordionSummary expandIcon={<DownIcon />}>快速傅里叶变换方案参数</AccordionSummary>
				<Divider />
				<AccordionDetails style={{ paddingTop: '16px' }}>
					<div className={styles.param} style={{ display: 'flex' }}>
						<Typography variant='body2'>多级过滤弱边缘阈值</Typography>
						<div style={{ marginLeft: 'auto' }}>
							<Input
								size='small'
								value={fftParams.cannyThresholdLower}
								inputProps={{
									step: 0.1,
									min: 0.0,
									max: fftParams.cannyThresholdHigher,
									type: 'number',
								}}
								sx={{ width: '5em' }}
								onChange={updateCannyThresholdLower}
							/>
						</div>
					</div>
					<Divider />
					<div className={styles.param} style={{ display: 'flex' }}>
						<Typography variant='body2'>多级过滤强边缘阈值</Typography>
						<div style={{ marginLeft: 'auto' }}>
							<Input
								size='small'
								value={fftParams.cannyThresholdHigher}
								inputProps={{
									step: 0.1,
									min: 255.0,
									type: 'number',
								}}
								sx={{ width: '5em' }}
								onChange={updateCannyThresholdHigher}
							/>
						</div>
					</div>
					<Divider />
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
								onChange={updateMinLineLength}
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
								onChange={updateMaxLineGap}
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
