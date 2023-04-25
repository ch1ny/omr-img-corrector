const cp = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const { version } = require(path.resolve(__dirname, '..', 'package.json'));

if (!!process.env.GITHUB_OUTPUT)
	fs.appendFileSync(process.env.GITHUB_OUTPUT, `release_version=${version}`);

const compilerPath = path.resolve(__dirname, 'Compiler/ISCC.exe');
const setupPath = path.resolve(__dirname, 'setup.iss');

cp.execSync(`${compilerPath} "${setupPath}" /DMyAppVersion="${version}"`);
