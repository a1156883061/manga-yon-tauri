import { Channel } from '@/interface/Api';
import { message } from 'ant-design-vue';

/** 提示消息显示时间 3s */
const MSG_INTERVAL = 3;

/**
 * 使用IpcRender.invoke的形式请求Node
 * @param channel 请求名
 * @param args 变量
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export default async function <K extends keyof Channel, T extends Channel>(
  channel: K,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  ...args: any[]
): Promise<T[K]> {
  const res = null as any;
  if (res.code == 0) {
    return res.data;
  }
  if (res.code > 500) {
    message.error(res.message, MSG_INTERVAL);
    console.error('error', res.message);
    throw new Error(res.message);
  }
  message.warning(res.message, MSG_INTERVAL);
  console.error('error', res.message);
  throw new Error(res.message);
}
