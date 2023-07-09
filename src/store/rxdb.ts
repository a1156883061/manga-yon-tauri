import { ReadProcess } from '@/interface';
type ProcessType<T, K extends keyof T> = T[K];

export type ComicDocType = {
  id: string;
  title: string;
  readProcess?: ProcessType<ReadProcess, 'process'>;
  path: string[];
};
