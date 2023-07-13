<template>
  <reader-container />
</template>

<script lang="ts">
  import { defineComponent } from 'vue';
  import readerContainer from '@/views/reader/reader-container.vue';

  export default defineComponent({
    components: { readerContainer },
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
    background: #292d3e;
    overflow: auto;
  }
</style>
