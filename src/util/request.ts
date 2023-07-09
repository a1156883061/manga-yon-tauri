import {Channel} from '@/interface/Api';
import {message} from 'ant-design-vue';
import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";

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
    args?: InvokeArgs
): Promise<T[K]> {
    let res = null as any;
    try {
            res = await invoke(channel, args);
            console.log("res: ", res);
            return res;
    } catch (e) {
        message.error(e as string, MSG_INTERVAL);
        console.error('error', e as string);
        throw new Error(e as string);
    }
}
