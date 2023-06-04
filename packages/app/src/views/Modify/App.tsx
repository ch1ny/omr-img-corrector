import React, { useCallback, useEffect, useRef, useState } from 'react';
import useMount from '@/hooks/useMount';
import { writeBinaryFile } from '@tauri-apps/api/fs';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { getCurrent } from '@tauri-apps/api/window';
import Cropper, { ReactCropperElement } from 'react-cropper';
import { Button, CircularProgress, Slider, Snackbar } from '@mui/material';
import MuiAlert, { AlertProps } from '@mui/material/Alert';
import styles from './App.module.less';
import 'cropperjs/dist/cropper.css';

const Alert = React.forwardRef<HTMLDivElement, AlertProps>(function Alert(props, ref) {
	return <MuiAlert elevation={6} ref={ref} variant='filled' {...props} />;
});

export default function App() {
	const [imageUrl, setImageUrl] = useState('');
	const [outputting, setOutputting] = useState(false);

	useMount(() => {
		let { search } = location;
		if (search.startsWith('?')) search = search.slice(1);
		const query = new Map<string, string>();
		search.split('&').map((queryPair) => {
			const [key, value] = queryPair.split('=').map(decodeURIComponent);
			query.set(key, value);
		});
		const src = query.get('src')!;
		setImageUrl(src);

		const currentWindow = getCurrent();
		currentWindow.setFocus();
	});

	const [rotate, setRotate] = useState(0.0);
	const editorRef = useRef<ReactCropperElement>(null);
	useEffect(() => {
		editorRef.current?.cropper.rotateTo(rotate);
	}, [rotate]);

	const [cropCallback, setCropCallback] = React.useState<-1 | 0 | 1>(0);
	const handleClose = useCallback((event?: React.SyntheticEvent | Event, reason?: string) => {
		// cSpell: disable-next-line
		if (reason === 'clickaway') {
			return;
		}

		setCropCallback(0);
	}, []);

	return (
		<>
			<Snackbar
				anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
				open={cropCallback !== 0}
				autoHideDuration={2000}
				onClose={handleClose}
			>
				<Alert
					onClose={handleClose}
					severity={cropCallback === -1 ? 'error' : 'success'}
					sx={{ width: '100%' }}
				>
					{cropCallback === -1 ? '裁剪失败！' : '输出成功！'}
				</Alert>
			</Snackbar>
			<div className={styles.app}>
				<div className={styles.editorWrapper}>
					{!!imageUrl && (
						<Cropper
							ref={editorRef}
							className={styles.editor}
							src={convertFileSrc(imageUrl)}
							rotatable
						/>
					)}
				</div>
				<div className={styles.panel}>
					<div className={styles.rotate}>
						<Slider
							value={rotate}
							step={0.1}
							min={-45.0}
							max={45.0}
							valueLabelDisplay='auto'
							onChange={(evt, value) => {
								setRotate(value as number);
							}}
						/>
					</div>
					<div className={styles.buttons}>
						<Button
							variant='contained'
							onClick={() => {
								setRotate(0.0);
								editorRef.current?.cropper.reset();
							}}
						>
							重置
						</Button>
						<Button
							color='success'
							variant='contained'
							disabled={outputting}
							startIcon={outputting && <CircularProgress size={'1em'} color={'inherit'} />}
							onClick={async () => {
								setOutputting(true);
								try {
									const blob = await new Promise<Blob>((resolve, reject) => {
										editorRef.current?.cropper
											.getCroppedCanvas({
												fillColor: '#ffffff',
											})
											.toBlob((blob) => {
												if (blob === null) reject();
												resolve(blob as Blob);
											});
									});

									if (!blob) return;
									const buffer = await blob.arrayBuffer();
									await writeBinaryFile(imageUrl, buffer);
									setCropCallback(1);
								} catch (ex) {
									setCropCallback(-1);
								} finally {
									setOutputting(false);
								}
							}}
						>
							输出
						</Button>
					</div>
				</div>
			</div>
		</>
	);
}
