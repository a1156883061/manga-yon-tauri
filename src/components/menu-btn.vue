<template>
  <div class="menu-container">
    <a-button class="menu-btn" shape="circle" size="large">
      <template #icon><menu-outlined /></template>
    </a-button>
  </div>
</template>
<script setup lang="ts">
  import { MenuOutlined } from '@ant-design/icons-vue';
</script>
<style scoped>
  .menu-container {
    position: fixed;
    background: transparent;
    right: 50px;
    top: 50px;
  }

  .menu-btn {
    opacity: 0.4;
    background: #292d3e;
    border-width: 0px;
  }
  .menu-btn:hover,
  .menu-btn:active,
  .menu-btn:focus {
    opacity: 1;
    background-color: #272b3a;
    color: currentColor;
  }
</style>
