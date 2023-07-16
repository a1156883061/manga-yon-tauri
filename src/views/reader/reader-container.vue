<template>
  <div ref="container" class="container resize-control" @pointermove="resize">
    <menu-btn @click="showSetting"/>
    <div
      class="resize-bar left-resize-bar"
      @pointerdown="dragStart(-1)"
      @touchstart.prevent
    ></div>
    <div class="content" :style="{ width: contentWidth + 'px' }">
      <div
        v-for="(comic, index) in comics.path"
        :key="index"
        class="img-container"
      >
        <img :src="comic" :alt="comic"
             :data-index="index" @dragstart.prevent class="comic-img" />
      </div>
    </div>
    <div
      class="resize-bar right-resize-bar"
      @pointerdown="dragStart()"
      @touchstart.prevent
    ></div>
    <SettingMenu v-model:show="show" />
  </div>
</template>

<script lang="ts" setup>
  import { Comic } from '@/interface';
  import { ComicDocType } from '@/store/rxdb';
  import request from '@/util/request';
  import { computed, onMounted, onUnmounted, reactive, Ref, ref } from 'vue';
  import { getImgFromPoint, jumpToReadProcess } from './reader';
  import MenuBtn from '@/components/menu-btn.vue';
  import SettingMenu from '@/components/setting-menu.vue';
  import {window as taWin} from "@tauri-apps/api";
  import { appWindow } from "@tauri-apps/api/window";
  import {convertFileSrc} from "@tauri-apps/api/tauri";
  let id = 0;
  /**默认宽度百分比 */
  const WIDTH_PERCENT = 0.9;

  /** 鼠标拖动方向 */
  enum DragDirection {
    Left,
    Right,
  }

  /**鼠标事件左键 */
  const LEFT_KEY = 1;
  /** 显示的漫画数据 */
  const comics: Comic = reactive({
    path: [],
    title: '',
  });

  let show = ref(false);
  /** 显示宽度 */
  const contentWidth = ref(0);
  const dragFlag = ref(false);
  const direction = ref(DragDirection.Left);
  // const resizeState = ref('unset');
  const container = ref(null as unknown) as Ref<HTMLDivElement>;
  const resizeState = computed(() => (dragFlag.value ? 'ew-resize' : 'unset'));
  /**拖动事件开始时当前滚动高度的百分比 */
  const scrollTopPercent = ref(0);
  /**
   * 获取漫画的地址
   */
  async function getComicPath() {
    const winId = taWin.getCurrent().label;
    request('get_comic', {id: Number(winId)}).then(
      (returnData: [ComicDocType, string[]]) => {
        comics.path = returnData[1].map(comicPath => convertFileSrc(comicPath));
        id = returnData[0].id as unknown as number;
        console.log('readProcess', returnData[0].readProcess);
        jumpToReadProcess(returnData);
      }
    );
  }
  /**
   * 初始化宽度
   */
  async function initWidth() {
    const width = container.value.getBoundingClientRect().width;
    try {
      let widthPercent = await request('reader_get_width');
      if (widthPercent > WIDTH_PERCENT) {
        widthPercent = WIDTH_PERCENT;
      }
      contentWidth.value = width * widthPercent;
    } catch {
      contentWidth.value = width * WIDTH_PERCENT;
    }
  }
  /**
   * 鼠标拖动改变宽度
   */
  function resize(mouseEvent: PointerEvent) {
    if (dragFlag.value) {
      if (mouseEvent instanceof MouseEvent && mouseEvent.buttons != LEFT_KEY) {
        return;
      }
      let clientX;
      if (mouseEvent instanceof TouchEvent && mouseEvent.touches.length === 1) {
        clientX = mouseEvent.touches[0].clientX;
      } else if (mouseEvent instanceof MouseEvent) {
        clientX = mouseEvent.clientX;
      } else {
        return;
      }
      const containerWidth = container.value.getBoundingClientRect().width;
      if (direction.value == DragDirection.Left) {
        contentWidth.value = containerWidth - clientX * 2 - 6;
        return;
      }
      contentWidth.value = containerWidth - (containerWidth - clientX) * 2;
    }
  }
  /**
   * 拖动开始回调函数
   * @param dir 拖动的方向
   */
  function dragStart(dir = 1) {
    direction.value = dir == 1 ? DragDirection.Right : DragDirection.Left;
    dragFlag.value = true;
    scrollTopPercent.value =
      window.scrollY / container.value.getBoundingClientRect().height;
  }

  function saveWidth() {
    console.log("width change");
    // 获取宽度百分比
    const widthPercent =
      contentWidth.value / container.value.getBoundingClientRect().width;
    request('reader_save_width', {width: widthPercent});
  }

  function dragEnd() {
    // 保存函数在更改宽度时才执行
    if (dragFlag.value) {
      window.scrollTo({
        left: 0,
        top:
          container.value.getBoundingClientRect().height *
          scrollTopPercent.value,
        behavior: 'smooth',
      });
      saveWidth();
    }
    dragFlag.value = false;
  }
  getComicPath();
  onMounted(() => {
    initWidth();
  });

  async function saveReadProcess() {
    await request('reader_save_read_process', {
      id,
      process: getImgFromPoint(container.value),
    });
  }
  appWindow.onCloseRequested(async () => {
    await saveReadProcess();
  })
  function showSetting() {
    show.value = true
  }
  // window.ipcRenderer.receive('request-read-process', (arg) => {
  //   console.log('received request-read-process msg', arg);
  //   saveReadProcess();
  // });

  // 清除监听
  onUnmounted(() => {
    document.removeEventListener('pointerup', dragEnd);
  });
  document.addEventListener('pointerup', dragEnd);
</script>

<style scoped>
  .comic-img {
    width: 100%;
  }

  .container {
    display: flex;
    justify-content: center;
    min-height: 100%;
  }

  .content {
    width: 90%;
    max-width: 98%;
    min-width: 300px;
    user-select: none;
  }

  .resize {
    position: relative;
  }

  .resize-bar {
    content: '';
    display: block;
    cursor: col-resize;
    cursor: ew-resize;
    background: transparent;
  }

  .left-resize-bar {
    border-right: 3px red solid;
    border-left: 8px transparent solid;
  }

  .right-resize-bar {
    left: 0;
    border-left: 3px red solid;
    border-right: 8px transparent solid;
  }

  .resize-control {
    cursor: v-bind(resizeState);
  }
</style>
