<script setup lang="ts">
  import { onMounted } from 'vue';
  import DNFSkill from './components/DNFSkill.vue';
  import { initKeyboardListener } from './utils/keyboardListener';

  onMounted(() => {
    initKeyboardListener();
  });
</script>

<template>
  <div class="app">
    <DNFSkill />
  </div>
</template>

<style>
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    -webkit-user-drag: none;
    user-drag: none;
  }

  html,
  body {
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: transparent;
    cursor: default;
  }

  #app {
    width: 100%;
    height: 100%;
    background: transparent;
  }

  .app {
    width: 100%;
    height: 100%;
    background: transparent;
  }
</style>