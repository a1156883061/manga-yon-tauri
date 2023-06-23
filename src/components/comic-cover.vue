<template>
  <div ref="container"></div>
</template>

<script lang="ts">
  import { defineComponent, ref } from 'vue';
  function addCover(path: string, container: HTMLElement | undefined) {
    const coverImg = new Image();
    coverImg.addEventListener('onload', () => {
      if (container != null) {
        container.append(coverImg);
      }
    });
    coverImg.src = path;
  }

  export default defineComponent({
    props: ['path'],
    setup(props) {
      const container = ref<HTMLDivElement>();
      addCover(props.path, container.value);
      return {
        container,
      };
    },
  });
</script>

<style></style>
