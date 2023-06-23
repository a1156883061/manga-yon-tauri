<template>
  <Icon class="mo-icon" :component="ComicShelfSvg" />
</template>

<script lang="ts">
  import { defineComponent, markRaw } from 'vue';
  import Icon from '@ant-design/icons-vue';
  import ComicShelfSvg from './comic-shelf-svg.vue';

  export default defineComponent({
    components: { Icon },
    setup() {
      return {
        ComicShelfSvg: markRaw(ComicShelfSvg),
      };
    },
  });
</script>

<style scoped>
  .mo-icon >>> .icon {
    font-size: 75px;
  }
</style>
