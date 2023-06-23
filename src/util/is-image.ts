import { getType } from 'mime';

function isImage(fileName: string) {
  const mimeType = getType(fileName);
  if (mimeType == undefined) {
    return false;
  }
  if (mimeType.startsWith('image/')) {
    return true;
  }
}

export default isImage;
