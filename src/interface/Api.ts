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
  'reader_save_width': void;
  /**
   * 获取宽度百分比
   */
  'reader_get_width': number;

  'add_comic': ComicDocType | false;

  'add_comic_folder': ComicDocType[] | false;

  'get_store_comic': ComicSource[];

  'read_comic': void;

  'get_comic': [ComicDocType, string[]];

  'reader_save_read_process': void;
}
export type { Channel, Response };
