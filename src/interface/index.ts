interface ComicSource {
  id: number;
  path: string[];
  title: string;
  coverPath: string;
}

/** readComicSource */
type RawComicSource = Omit<ComicSource, 'id'>;

interface Comic {
  path: string[];
  title: string;
}

/**
 * 窗口信息
 */
interface WindowInfo {
  /**
   * 是否最大化
   */
  isMaxSized: boolean;
  /**
   * 窗口位置与大小
   * TODO 修正
   */
  windowConfig: any;
}

/**
 * IPC message interface
 */
interface IpcMsg {
  /**
   * message code
   * code = 0:success
   * code = 1xx info
   * code = 2xx warning
   * code = 3xx error
   */
  code: number;
  /**
   * message
   */
  msg: string;
  /**
   * data to transfer data
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  data?: any;
}

/**
 * IpcRender.handle 的返回值
 */
interface Response<T> {
  code: number;
  message: string;
  data?: T;
}

export interface ReadProcess {
  /** comic id */
  id: number;
  /** read process, img url */
  process: string;
}

export type { ComicSource, Comic, WindowInfo, IpcMsg, Response, RawComicSource };
