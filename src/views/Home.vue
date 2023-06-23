<template>
  <a-layout id="layout-trigger">
    <a-layout-sider
      v-model:collapsed="collapsed"
      :trigger="null"
      collapsible
      style="background: #fafafa"
    >
      <div class="logo" />
      <menu-unfold-outlined
        v-if="collapsed"
        class="trigger"
        @click="() => (collapsed = !collapsed)"
      />
      <menu-fold-outlined
        v-else
        class="trigger"
        @click="() => (collapsed = !collapsed)"
      />
      <a-menu
        theme="light"
        mode="inline"
        class="mo-menu"
        v-model:selectedKeys="selectedKeys"
      >
        <a-menu-item key="1">
          <user-outlined />
          <span>书架</span>
        </a-menu-item>
        <a-menu-item key="2">
          <video-camera-outlined />
          <span>设置</span>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
    <a-layout>
      <a-layout-content :class="comicList">
        <comic-shelf />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script lang="ts" setup>
  import { ref } from 'vue';
  import comicShelf from '@/views/comic-shelf.vue';
  import {
    UserOutlined,
    VideoCameraOutlined,
    MenuUnfoldOutlined,
    MenuFoldOutlined,
  } from '@ant-design/icons-vue';

  const selectedKeys = ref(['1']);
  const collapsed = ref(false);
  const comicList = ref('comic-list');
</script>

<style>
  body {
    background: rgb(250, 250, 250);
  }
</style>

<style scoped>
  #layout-trigger {
    height: 100%;
  }

  #layout-trigger .trigger {
    font-size: 18px;
    line-height: 64px;
    padding: 0 24px;
    cursor: pointer;
    transition: color 0.3s;
  }

  #layout-trigger .trigger:hover {
    color: #1890ff;
  }

  .comic-list {
    margin-left: 16px;
    margin-right: 16px;
    padding: 24px;
    min-height: 280px;
  }
  .mo-menu {
    background-color: #fafafa;
  }
</style>
