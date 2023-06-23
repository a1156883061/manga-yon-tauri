/**
 *将原始路径转为自定义文件协议路径
 * @param filePath 文件路径
 */
function convertToSafeFile(filePath: string) {
  return 'safe-file://' + filePath;
}

export default convertToSafeFile;
