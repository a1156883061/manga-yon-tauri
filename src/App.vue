<template>
  <a-config-provider :locale="locale">
    <div id="nav">
      <router-view v-slot="{ Component }">
        <keep-alive include="HelloWorld">
          <component :is="Component" />
        </keep-alive>
      </router-view>
    </div>
  </a-config-provider>
</template>

<script lang="ts">
  import zhCN from 'ant-design-vue/es/locale/zh_CN';
  import { defineComponent, ref } from 'vue';

  export default defineComponent({
    setup() {
      const locale = ref(zhCN);
      return {
        locale,
      };
    },
  });
</script>

<style>
  #app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    height: 100%;
  }

  #nav {
    padding-top: 16px;
    height: 100%;
  }
</style>
