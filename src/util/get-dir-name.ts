import path from 'path';

/**
 * 获取文件所在文件夹的名称
 * @param filePath 文件路径
 */
function getParentDirName(filePath: string) {
  filePath = path.resolve(filePath);
  return path
    .dirname(filePath)
    .split(path.sep)
    .pop();
}

/**
 * 获取对应文件夹路径的名称
 * @param dirPath 文件夹路径
 */
export function getDirName(dirPath: string): string {
  const dirPathTmp = path.resolve(dirPath);
  const dirName = dirPathTmp.split(path.sep).pop();
  if (dirName) {
    return dirName;
  }
  return 'unknown';
}

export default getParentDirName;
