import { ComicDocType } from '@/store/rxdb';
import { ComicSource } from '.';

/**
 * 返回结构体
 */
interface Response<T> {
  code: number;
  message?: string;
  data?: T;
}

/**
 * 请求的Channel对应的返回值
 */
interface Channel {
  /**
   * 删除漫画
   */
  'comic_delete': void;
  /**
   * 保存宽度百分比
   */
  'reader/save_width': void;
  /**
   * 获取宽度百分比
   */
  'reader/get_width': number;

  'add_comic': ComicDocType | false;

  'add_comic_folder': ComicDocType[] | false;

  'get_store_comic': ComicSource[];

  'read_comic': void;

  'get_comic': [ComicDocType, string[]];

  'reader/save_read-process': void;
}

type saveReadProcess = 'reader/save-read-process';

type SendChannel = saveReadProcess;

export type { Channel, Response, SendChannel };
