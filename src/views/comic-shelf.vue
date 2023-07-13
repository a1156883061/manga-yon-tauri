<template>
  <div class="container">
    <a-card class="mo-card">
      <a-card-grid class="mo-card-grid mo-card-grid-add">
        <a-button class="open-file-btn" block @click="addComic()">
          <icon-comic-shelf style="text-align: center"/>
        </a-button>
        <a-button class="open-file-btn" block @click="addComicFolder">
          <folder-open-filled style="text-align: center; font-size: 50px"/>
        </a-button>
      </a-card-grid>
      <template v-if="comicSources != null && comicSources.length != 0">
        <a-card-grid
            class="mo-card-grid mo-card-grid-comic"
            v-for="(comic, index) in comicSources"
            :key="index"
            @click="readComic(comic.path, comic.title, comic.id, comic.isLoading)"
            @contextmenu.prevent="showContext(comic)"
        >
          <a-spin
              :spinning="comic.isLoading"
              :delay="300"
              size="large"
              class="mo-spin"
          >
            <div class="cover-container">
              <img :src="comic.coverPath ? convertFileSrc(comic.coverPath): null" :alt="comic.title"/>
            </div>
          </a-spin>
          <a-card-meta :title="comic.title"></a-card-meta>
          <div class="action-container" v-if="comic.showActionFlag" @click.stop>
            <a-button type="primary" danger block @click="deleteComic(comic.id)">
              <template #icon>
                <delete-filled/>
              </template>
            </a-button>
          </div>
        </a-card-grid>
      </template>
    </a-card>
  </div>
</template>

<script lang="ts" setup>
import iconComicShelf from '@/components/icon/icon-comic-shelf.vue';
import {ComicSource} from '@/interface';
import request from '@/util/request';
import {DeleteFilled, FolderOpenFilled} from '@ant-design/icons-vue';
import {reactive} from 'vue';
import {convertFileSrc} from "@tauri-apps/api/tauri";

interface ComicSourceLoad extends ComicSource {
  isLoading?: boolean;
  /**是否显示操作按钮 */
  showActionFlag: boolean;
}

const comicSources = reactive<ComicSourceLoad[]>([]);

async function addComic(
    channel: 'add_comic' | 'add_comic_folder' = 'add_comic'
) {
  const index = 0;
  comicSources.unshift({
    id: 0,
    isLoading: true,
    showActionFlag: false,
    path: [''],
    coverPath: '',
    title: '',
  });
  try {
    const newComic = (await request(channel)) as unknown as
        | ComicSourceLoad
        | ComicSourceLoad[];
    comicSources[index] = reactive(comicSources[index]);
    comicSources[index].isLoading = false;
    // 添加失败，删除占位元素
    if (newComic == null) {
      comicSources.splice(index, 1);
      return;
    }
    if (!(newComic instanceof Array)) {
      newComic.isLoading = false;
      // newComic.coverPath = newComic.coverPath;
      comicSources[index] = newComic;
      return;
    }
    if (newComic.length > 0) {
      comicSources.splice(index, 1);
      newComic
          .map((eachComic) => {
            return Object.assign(eachComic, {
              isLoading: false,
            });
          })
          .forEach((eachComic) => comicSources.unshift(eachComic));
    } else {
      comicSources.splice(index, 1);
    }
  } catch {
    comicSources.splice(index, 1);
  }
}

/**
 * 从文件夹中添加漫画
 */
async function addComicFolder() {
  addComic('add_comic_folder');
}

async function getComics() {
  const comics = (await request('get_store_comic')) as ComicSourceLoad[];
  comics.forEach((each) => {
    each.isLoading = false;
  });
  comicSources.push(...comics);
}

function readComic(
    comicPaths: string[],
    title: string,
    id: number,
    isLoading?: boolean
) {
  if (isLoading || isLoading == undefined) {
    return;
  }
  request('read_comic', {title, id});
}

function showContext(comicPath: ComicSourceLoad) {
  comicPath.showActionFlag = !comicPath.showActionFlag;
}

async function deleteComic(comicId: ComicSourceLoad['id']) {
  try {
    await request('comic_delete', {id: comicId});
    const index = comicSources.findIndex(({id}) => id == comicId);
    comicSources.splice(index, 1);
  } catch {
    console.error('delete fail');
  }
}

getComics();
</script>

<style scoped>
.container {
  /* background: linear-gradient(to right top, #65dfc9, #6cdbeb); */
  height: 100%;
}

.mo-card {
  background-color: transparent;
  border: none;
}

.mo-card ::v-deep(.ant-card-body) {
  display: grid;
  grid-template-columns: repeat(auto-fill, 320px);
  justify-content: center;
}

.mo-card ::v-deep(.ant-card-body::before) {
  content: none;
}

.mo-card-grid {
  position: relative;
  cursor: pointer;
  user-select: none;
  width: 320px;
  height: 510px;
  background: linear-gradient(
      to right bottom,
      rgba(255, 255, 255, 0.7),
      rgba(255, 255, 255, 0.3)
  );
  /* background-color: #b0b5be; */
}

/* .mo-card-grid:nth-child(2n) {
  background-color: #8a919e;
  background: linear-gradient(
    to right bottom,
    rgba(255, 255, 255, 0.7),
    rgba(255, 255, 255, 0.3)
  );
} */
.cover-container {
  overflow: hidden;
  display: flex;
  align-items: center;
  height: 100%;
}

.cover-container img {
  width: 100%;
}

.mo-card-grid ::v-deep(.ant-card-meta-title) {
  text-overflow: ellipsis;
  white-space: normal;
}

/* 居中封面图片 */
.mo-card-grid ::v-deep(.ant-spin-nested-loading) {
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: 90%;
}

.mo-card-grid ::v-deep(.ant-spin-container) {
  height: 100%;
}

.mo-card-grid-add {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: stretch;
}

.mo-card-grid-comic {

}

.action-container {
  position: absolute;
  top: 0px;
  left: 0px;
  width: 100%;
  height: 100%;
  z-index: 100;
  padding: 24px;
  background: rgba(138, 145, 158, 0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: default;
}

.open-file-btn {
  flex-grow: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  outline: none;
}
</style>
