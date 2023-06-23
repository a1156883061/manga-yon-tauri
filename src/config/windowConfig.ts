import { store } from './store';
import { screen } from 'electron';
import { WindowInfo } from '@/interface';

const defaultWindow: WindowInfo = {
  isMaxSized: false,
  windowConfig: {
    width: 1920,
    height: 1080,
    x: 100,
    y: 100,
  },
};

/**
 * 验证窗口信息是否正确
 * @param bounds 窗口信息
 */
function verifyWindowBounds(bounds: WindowInfo): boolean {
  const winKeys = Object.keys(defaultWindow.windowConfig);
  const savedKeys = Object.keys(bounds.windowConfig);
  if (typeof bounds.isMaxSized != 'boolean') {
    return false;
  }
  for (const eachKey of savedKeys) {
    if (!winKeys.includes(eachKey)) {
      return false;
    }
  }
  const screenInfo = screen.getAllDisplays()[0].bounds;
  if (bounds.windowConfig.x < 0) {
    bounds.windowConfig.x = 0;
  }
  if (bounds.windowConfig.y < 0) {
    bounds.windowConfig.y = 0;
  }
  if (bounds.windowConfig.height > screenInfo.height) {
    bounds.windowConfig.height = screenInfo.height - 50;
  }
  if (bounds.windowConfig.width > screenInfo.width) {
    bounds.windowConfig.width = screenInfo.width - 50;
  }
  return true;
}

/**
 * 保存窗口位置与大小配置
 * @param windowConfig 窗口配置
 * @param winName 窗口名称
 */
function saveWindowBoundsConfig(
  winName: string,
  windowConfig: WindowInfo
): void {
  store.set(winName, windowConfig);
}
/**
 * 根据名称获取存储的窗口信息
 * @param winName 窗口名称
 */
function getWindowBounds(winName: string): WindowInfo {
  const winInfo = store.get(winName, defaultWindow) as WindowInfo;
  if (typeof winInfo.windowConfig === 'object' && verifyWindowBounds(winInfo)) {
    return winInfo as WindowInfo;
  }
  return defaultWindow;
}

export { saveWindowBoundsConfig, getWindowBounds };
