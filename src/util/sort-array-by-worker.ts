// import { runTask } from './my-thread';
import { Worker } from 'worker_threads';

// async function sortArrayByWorker(dirName: string, array: string[]) {
//   // const sortWorker = await getThread();
//   const promiseResult = new Promise<string[]>((resolve) => {
//     runTask(async (worker) => {
//       resolve(worker.sortArray(dirName, array));
//     });
//   });
//   const result = await promiseResult;
//   return result;
// }

function sortArrayByWorker5(dirName: string, array: string[]) {
  console.log('metaUrl', import.meta.url);
  console.log('url', new URL('../worker/sort-array.js', import.meta.url));
  const sortWorker = new Worker(
    new URL('../worker/sort-array.js', import.meta.url).pathname.substring(1)
  );
  sortWorker.postMessage({ dirName, array });
  return new Promise((resolve: (val: string[]) => void) => {
    sortWorker.addListener(
      'message',
      ({ data: { resultPaths } }: { data: { resultPaths: string[] } }) =>
        resolve(resultPaths)
    );
  });
}

export default sortArrayByWorker5;
