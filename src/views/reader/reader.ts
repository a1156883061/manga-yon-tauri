import { ComicDocType } from '@/store/rxdb';
import { nextTick } from 'vue';

/**
 * get top img as reader process
 * @param container img container
 * @param selector img selector
 * @returns top img
 */
export function getImgFromPoint(container: HTMLDivElement): string | null {
  const imgElement = document.elementFromPoint(
    container.getBoundingClientRect().width / 2,
    100
  );
  if (imgElement) {
    return imgElement.getAttribute('src');
  }
  return null;
}

export function jumpToReadProcess(returnData: [ComicDocType, string[]]) {
  nextTick(() => {
    const readProcess = returnData[0].readProcess as unknown as number;
    if (readProcess) {
      let alt = returnData[1][readProcess]
      const imgElement: HTMLImageElement | null = document.querySelector(
        `[alt="${alt.replaceAll('\\', '\\\\')}"]`
      );
      console.log('read imgElement', imgElement);
      if (imgElement) {
        imgElement.addEventListener('load', () => imgElement.scrollIntoView());
      }
    }
  });
}
