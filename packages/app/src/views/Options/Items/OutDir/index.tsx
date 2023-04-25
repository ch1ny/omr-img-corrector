import path from '@/core/path';
import useLocalStorage from '@/hooks/useLocalStorage';
import useMount from '@/hooks/useMount';
import { Paths } from '@/utils';
import { Button, Input } from '@mui/material';
import * as dialog from '@tauri-apps/api/dialog';
import * as shell from '@tauri-apps/api/shell';
import { useCallback } from 'react';
import styles from './index.module.less';

const OutDir = () => {
	const [outputDir, setOutputDir] = useLocalStorage<string>('default_output_dir');
	useMount(async () => {
		if (!!outputDir) return;

		if (!Paths.exePath) {
			await Paths.initPaths();
		}

		setOutputDir(path.resolveSync(Paths.exePath, '..', 'output'));
	});

	const handleSelectOutputDir = useCallback(async () => {
		const selected = await dialog.open({
			multiple: false,
			directory: true,
		});
		if (selected === null) return;

		setOutputDir(selected as string);
	}, []);

	const openOutputDir = useCallback(() => {
		new shell.Command('windows-explorer', [outputDir]).spawn();
	}, [outputDir]);

	return (
		<div className={styles.outDir}>
			<div className={styles.outDirPath}>
				<Input
					type='text'
					style={{ fontSize: '12px', marginRight: '6px' }}
					value={outputDir}
					fullWidth
				/>
			</div>
			<div className={styles.outDirButtons}>
				<Button variant={'contained'} size={'small'} onClick={handleSelectOutputDir}>
					更改输出文件夹
				</Button>
				<Button variant={'outlined'} size={'small'} onClick={openOutputDir}>
					打开输出文件夹
				</Button>
			</div>
		</div>
	);
};

export default OutDir;
