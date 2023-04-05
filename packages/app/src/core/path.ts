import {
	CHAR_BACKWARD_SLASH,
	CHAR_COLON,
	CHAR_DOT,
	CHAR_FORWARD_SLASH,
	CHAR_LOWERCASE_A,
	CHAR_LOWERCASE_Z,
	CHAR_UPPERCASE_A,
	CHAR_UPPERCASE_Z,
} from '@/constants';
import { Paths } from '@/utils';
import * as path from '@tauri-apps/api/path';

function isPathSeparator(code: number) {
	return code === CHAR_FORWARD_SLASH || code === CHAR_BACKWARD_SLASH;
}

function isPosixPathSeparator(code: number) {
	return code === CHAR_FORWARD_SLASH;
}

function isWindowsDeviceRoot(code: number) {
	return (
		(code >= CHAR_UPPERCASE_A && code <= CHAR_UPPERCASE_Z) ||
		(code >= CHAR_LOWERCASE_A && code <= CHAR_LOWERCASE_Z)
	);
}

function normalizeString(
	path: string,
	allowAboveRoot: boolean,
	separator: string,
	isPathSeparator: (code: number) => boolean
) {
	let res = '';
	let lastSegmentLength = 0;
	let lastSlash = -1;
	let dots = 0;
	let code = 0;
	for (let i = 0; i <= path.length; ++i) {
		if (i < path.length) code = path.charCodeAt(i);
		else if (isPathSeparator(code)) break;
		else code = CHAR_FORWARD_SLASH;

		if (isPathSeparator(code)) {
			if (lastSlash === i - 1 || dots === 1) {
				// NOOP
			} else if (dots === 2) {
				if (
					res.length < 2 ||
					lastSegmentLength !== 2 ||
					res.charCodeAt(res.length - 1) !== CHAR_DOT ||
					res.charCodeAt(res.length - 2) !== CHAR_DOT
				) {
					if (res.length > 2) {
						const lastSlashIndex = res.lastIndexOf(separator);
						if (lastSlashIndex === -1) {
							res = '';
							lastSegmentLength = 0;
						} else {
							res = res.slice(0, lastSlashIndex);
							lastSegmentLength = res.length - 1 - res.lastIndexOf(separator);
						}
						lastSlash = i;
						dots = 0;
						continue;
					} else if (res.length !== 0) {
						res = '';
						lastSegmentLength = 0;
						lastSlash = i;
						dots = 0;
						continue;
					}
				}
				if (allowAboveRoot) {
					res += res.length > 0 ? `${separator}..` : '..';
					lastSegmentLength = 2;
				}
			} else {
				if (res.length > 0) res += `${separator}${path.slice(lastSlash + 1, i)}`;
				else res = path.slice(lastSlash + 1, i);
				lastSegmentLength = i - lastSlash - 1;
			}
			lastSlash = i;
			dots = 0;
		} else if (code === CHAR_DOT && dots !== -1) {
			++dots;
		} else {
			dots = -1;
		}
	}
	return res;
}

const win32 = {
	resolve(...args: string[]): string {
		let resolvedDevice = '';
		let resolvedTail = '';
		let resolvedAbsolute = false;

		for (let i = args.length - 1; i >= -1; i--) {
			let path;
			if (i >= 0) {
				path = args[i];

				// Skip empty entries
				if (path.length === 0) {
					continue;
				}
			} else if (resolvedDevice.length === 0) {
				path = Paths.exePath;
			} else {
				// Windows has the concept of drive-specific current working
				// directories. If we've resolved a drive letter but not yet an
				// absolute path, get cwd for that drive, or the process cwd if
				// the drive cwd is not available. We're sure the device is not
				// a UNC path at this points, because UNC paths are always absolute.
				path = Paths.exePath;

				// Verify that a cwd was found and that it actually points
				// to our drive. If not, default to the drive's root.
				if (
					path === undefined ||
					(path.slice(0, 2).toLowerCase() !== resolvedDevice.toLowerCase() &&
						path.charCodeAt(2) === CHAR_BACKWARD_SLASH)
				) {
					path = `${resolvedDevice}\\`;
				}
			}

			const len = path.length;
			let rootEnd = 0;
			let device = '';
			let isAbsolute = false;
			const code = path.charCodeAt(0);

			// Try to match a root
			if (len === 1) {
				if (isPathSeparator(code)) {
					// `path` contains just a path separator
					rootEnd = 1;
					isAbsolute = true;
				}
			} else if (isPathSeparator(code)) {
				// Possible UNC root

				// If we started with a separator, we know we at least have an
				// absolute path of some kind (UNC or otherwise)
				isAbsolute = true;

				if (isPathSeparator(path.charCodeAt(1))) {
					// Matched double path separator at beginning
					let j = 2;
					let last = j;
					// Match 1 or more non-path separators
					while (j < len && !isPathSeparator(path.charCodeAt(j))) {
						j++;
					}
					if (j < len && j !== last) {
						const firstPart = path.slice(last, j);
						// Matched!
						last = j;
						// Match 1 or more path separators
						while (j < len && isPathSeparator(path.charCodeAt(j))) {
							j++;
						}
						if (j < len && j !== last) {
							// Matched!
							last = j;
							// Match 1 or more non-path separators
							while (j < len && !isPathSeparator(path.charCodeAt(j))) {
								j++;
							}
							if (j === len || j !== last) {
								// We matched a UNC root
								device = `\\\\${firstPart}\\${path.slice(last, j)}`;
								rootEnd = j;
							}
						}
					}
				} else {
					rootEnd = 1;
				}
			} else if (isWindowsDeviceRoot(code) && path.charCodeAt(1) === CHAR_COLON) {
				// Possible device root
				device = path.slice(0, 2);
				rootEnd = 2;
				if (len > 2 && isPathSeparator(path.charCodeAt(2))) {
					// Treat separator following drive name as an absolute path
					// indicator
					isAbsolute = true;
					rootEnd = 3;
				}
			}

			if (device.length > 0) {
				if (resolvedDevice.length > 0) {
					if (device.toLowerCase() !== resolvedDevice.toLowerCase())
						// This path points to another device so it is not applicable
						continue;
				} else {
					resolvedDevice = device;
				}
			}

			if (resolvedAbsolute) {
				if (resolvedDevice.length > 0) break;
			} else {
				resolvedTail = `${path.slice(rootEnd)}\\${resolvedTail}`;
				resolvedAbsolute = isAbsolute;
				if (isAbsolute && resolvedDevice.length > 0) {
					break;
				}
			}
		}

		// At this point the path should be resolved to a full absolute path,
		// but handle relative paths to be safe (might happen when process.cwd()
		// fails)

		// Normalize the tail path
		resolvedTail = normalizeString(resolvedTail, !resolvedAbsolute, '\\', isPathSeparator);

		return resolvedAbsolute
			? `${resolvedDevice}\\${resolvedTail}`
			: `${resolvedDevice}${resolvedTail}` || '.';
	},

	normalize(path: string): string {
		const len = path.length;
		if (len === 0) return '.';
		let rootEnd = 0;
		let device;
		let isAbsolute = false;
		const code = path.charCodeAt(0);

		// Try to match a root
		if (len === 1) {
			// `path` contains just a single char, exit early to avoid
			// unnecessary work
			return isPosixPathSeparator(code) ? '\\' : path;
		}
		if (isPathSeparator(code)) {
			// Possible UNC root

			// If we started with a separator, we know we at least have an absolute
			// path of some kind (UNC or otherwise)
			isAbsolute = true;

			if (isPathSeparator(path.charCodeAt(1))) {
				// Matched double path separator at beginning
				let j = 2;
				let last = j;
				// Match 1 or more non-path separators
				while (j < len && !isPathSeparator(path.charCodeAt(j))) {
					j++;
				}
				if (j < len && j !== last) {
					const firstPart = path.slice(last, j);
					// Matched!
					last = j;
					// Match 1 or more path separators
					while (j < len && isPathSeparator(path.charCodeAt(j))) {
						j++;
					}
					if (j < len && j !== last) {
						// Matched!
						last = j;
						// Match 1 or more non-path separators
						while (j < len && !isPathSeparator(path.charCodeAt(j))) {
							j++;
						}
						if (j === len) {
							// We matched a UNC root only
							// Return the normalized version of the UNC root since there
							// is nothing left to process
							return `\\\\${firstPart}\\${path.slice(last)}\\`;
						}
						if (j !== last) {
							// We matched a UNC root with leftovers
							device = `\\\\${firstPart}\\${path.slice(last, j)}`;
							rootEnd = j;
						}
					}
				}
			} else {
				rootEnd = 1;
			}
		} else if (isWindowsDeviceRoot(code) && path.charCodeAt(1) === CHAR_COLON) {
			// Possible device root
			device = path.slice(0, 2);
			rootEnd = 2;
			if (len > 2 && isPathSeparator(path.charCodeAt(2))) {
				// Treat separator following drive name as an absolute path
				// indicator
				isAbsolute = true;
				rootEnd = 3;
			}
		}

		let tail =
			rootEnd < len ? normalizeString(path.slice(rootEnd), !isAbsolute, '\\', isPathSeparator) : '';
		if (tail.length === 0 && !isAbsolute) tail = '.';
		if (tail.length > 0 && isPathSeparator(path.charCodeAt(len - 1))) tail += '\\';
		if (device === undefined) {
			return isAbsolute ? `\\${tail}` : tail;
		}
		return isAbsolute ? `${device}\\${tail}` : `${device}${tail}`;
	},

	join(...args: string[]): string {
		if (args.length === 0) return '.';

		let joined;
		let firstPart!: string;
		for (let i = 0; i < args.length; ++i) {
			const arg = args[i];
			if (arg.length > 0) {
				if (joined === undefined) joined = firstPart = arg;
				else joined += `\\${arg}`;
			}
		}

		if (joined === undefined) return '.';

		// Make sure that the joined path doesn't start with two slashes, because
		// normalize() will mistake it for a UNC path then.
		//
		// This step is skipped when it is very clear that the user actually
		// intended to point at a UNC path. This is assumed when the first
		// non-empty string arguments starts with exactly two slashes followed by
		// at least one more non-slash character.
		//
		// Note that for normalize() to treat a path as a UNC path it needs to
		// have at least 2 components, so we don't filter for that here.
		// This means that the user can use join to construct UNC paths from
		// a server name and a share name; for example:
		//   path.join('//server', 'share') -> '\\\\server\\share\\')
		let needsReplace = true;
		let slashCount = 0;
		if (isPathSeparator(firstPart.charCodeAt(0))) {
			++slashCount;
			const firstLen = firstPart.length;
			if (firstLen > 1 && isPathSeparator(firstPart.charCodeAt(1))) {
				++slashCount;
				if (firstLen > 2) {
					if (isPathSeparator(firstPart.charCodeAt(2))) ++slashCount;
					else {
						// We matched a UNC path in the first part
						needsReplace = false;
					}
				}
			}
		}
		if (needsReplace) {
			// Find any more consecutive slashes we need to replace
			while (slashCount < joined.length && isPathSeparator(joined.charCodeAt(slashCount))) {
				slashCount++;
			}

			// Replace the slashes if needed
			if (slashCount >= 2) joined = `\\${joined.slice(slashCount)}`;
		}

		return win32.normalize(joined);
	},

	dirname(path: string): string {
		const len = path.length;
		if (len === 0) return '.';
		let rootEnd = -1;
		let offset = 0;
		const code = path.charCodeAt(0);

		if (len === 1) {
			// `path` contains just a path separator, exit early to avoid
			// unnecessary work or a dot.
			return isPathSeparator(code) ? path : '.';
		}

		// Try to match a root
		if (isPathSeparator(code)) {
			// Possible UNC root

			rootEnd = offset = 1;

			if (isPathSeparator(path.charCodeAt(1))) {
				// Matched double path separator at beginning
				let j = 2;
				let last = j;
				// Match 1 or more non-path separators
				while (j < len && !isPathSeparator(path.charCodeAt(j))) {
					j++;
				}
				if (j < len && j !== last) {
					// Matched!
					last = j;
					// Match 1 or more path separators
					while (j < len && isPathSeparator(path.charCodeAt(j))) {
						j++;
					}
					if (j < len && j !== last) {
						// Matched!
						last = j;
						// Match 1 or more non-path separators
						while (j < len && !isPathSeparator(path.charCodeAt(j))) {
							j++;
						}
						if (j === len) {
							// We matched a UNC root only
							return path;
						}
						if (j !== last) {
							// We matched a UNC root with leftovers

							// Offset by 1 to include the separator after the UNC root to
							// treat it as a "normal root" on top of a (UNC) root
							rootEnd = offset = j + 1;
						}
					}
				}
			}
			// Possible device root
		} else if (isWindowsDeviceRoot(code) && path.charCodeAt(1) === CHAR_COLON) {
			rootEnd = len > 2 && isPathSeparator(path.charCodeAt(2)) ? 3 : 2;
			offset = rootEnd;
		}

		let end = -1;
		let matchedSlash = true;
		for (let i = len - 1; i >= offset; --i) {
			if (isPathSeparator(path.charCodeAt(i))) {
				if (!matchedSlash) {
					end = i;
					break;
				}
			} else {
				// We saw the first non-path separator
				matchedSlash = false;
			}
		}

		if (end === -1) {
			if (rootEnd === -1) return '.';

			end = rootEnd;
		}
		return path.slice(0, end);
	},
};

// const myPath = path.sep === '/' ? posix : win32;
const myPath = win32;

export default {
	...path,
	joinSync: myPath.join.bind(myPath),
	resolveSync: myPath.resolve.bind(myPath),
	dirnameSync: myPath.dirname.bind(myPath),
};
