import { Card, CardContent, CardHeader, Typography } from '@mui/material';
import React from 'react';

export type TestResultMistakes = {
	arithmetic_mean: number;
	max_mistake: number;
	standard_deviation: number;
};

interface IResultCardProps {
	test_id: number;
	method: 'Projection' | 'Hough' | 'FFT';
	duration: number;
	mistakes: TestResultMistakes;
}

enum MethodChinese {
	Projection = '投影标准差',
	Hough = '霍夫变换',
	FFT = '快速傅里叶变换',
}

const ResultCard: React.FC<IResultCardProps> = (props) => {
	const { test_id, method, duration, mistakes } = props;

	return (
		<Card>
			<CardHeader title={MethodChinese[method]} subheader={method} />
			<CardContent>
				<Typography variant='h6'>平均耗费时间</Typography>
				<Typography variant='body2' color='text.secondary'>
					{duration} ms
				</Typography>
				<Typography variant='h6'>方案平均误差</Typography>
				<Typography variant='body2' color='text.secondary'>
					{mistakes.arithmetic_mean}°
				</Typography>
				<Typography variant='h6'>误差标准差</Typography>
				<Typography variant='body2' color='text.secondary'>
					{mistakes.standard_deviation}°
				</Typography>
				<Typography variant='h6'>方案最大误差</Typography>
				<Typography variant='body2' color='text.secondary'>
					{mistakes.max_mistake}°
				</Typography>
			</CardContent>
		</Card>
	);
};

export default ResultCard;
