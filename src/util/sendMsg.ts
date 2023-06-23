import { SendChannel } from '@/interface/Api';

/**
 * 使用IpcRender.send的形式请求NodeJs
 * @param channel 请求名
 */
export default function <K extends SendChannel>(
  channel: K,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  ...args: any[]
): void {
  window.ipcRenderer.send(channel, ...args);
}
