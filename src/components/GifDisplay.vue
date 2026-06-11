<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';
  import { eventBus } from '../utils/eventBus';
  import { invoke } from '@tauri-apps/api/core';

  const gifList = [
    // 三个图片来自百度图片搜索，如有侵权请与我联系
    'https://ww2.sinaimg.cn/mw690/6f250299gy1hsr186q5z0g208w0ftx6t.gif',
    'https://pic.rmb.bdstatic.com/bjh/cms/241213/3907dad47ae9f1610ac54e97e8fc2ec0_1734084296.0824_720.gif',
    'https://wx2.sinaimg.cn/mw690/a007f1e0ly1i8ig2nsl9hg204702jq3a.gif',
  ];

  const currentIndex = ref(0);
  const currentGif = ref(gifList[0]);
  const lastKey = ref('');
  const isDragging = ref(false);
  const initialMouseX = ref(0);
  const initialMouseY = ref(0);
  const initialWindowX = ref(0);
  const initialWindowY = ref(0);
  const isMouseOver = ref(false);

  function handleKeyPress(data: unknown) {
    const key = data as string;
    console.log('handleKeyPress called with:', key);
    lastKey.value = key;

    switch (key) {
      case '1':
        currentIndex.value = 0;
        break;
      case '2':
        currentIndex.value = 1;
        break;
      case '3':
        currentIndex.value = 2;
        break;
      case 'ArrowRight':
      case 'Right':
      case 'A':
        currentIndex.value = (currentIndex.value + 1) % gifList.length;
        break;
      case 'ArrowLeft':
      case 'Left':
      case 'D':
        currentIndex.value = (currentIndex.value - 1 + gifList.length) % gifList.length;
        break;
      case 'ArrowUp':
      case 'Up':
        currentIndex.value = (currentIndex.value - 1 + gifList.length) % gifList.length;
        break;
      case 'ArrowDown':
      case 'Down':
        currentIndex.value = (currentIndex.value + 1) % gifList.length;
        break;
    }

    currentGif.value = gifList[currentIndex.value];
  }

  async function onDragStart(event: MouseEvent) {
    isDragging.value = true;

    initialMouseX.value = event.screenX;
    initialMouseY.value = event.screenY;

    const posResult = await invoke('get_window_position');
    const windowPos = posResult as { 0: number, 1: number };
    initialWindowX.value = windowPos[0];
    initialWindowY.value = windowPos[1];

    console.log('Drag Start:');
    console.log('  Mouse: (', initialMouseX.value, ', ', initialMouseY.value, ')');
    console.log('  Window: (', initialWindowX.value, ', ', initialWindowY.value, ')');

    window.addEventListener('mousemove', onDragMove);
    window.addEventListener('mouseup', onDragEnd);
  }

  async function onDragMove(event: MouseEvent) {
    if (!isDragging.value) return;

    const deltaX = event.screenX - initialMouseX.value;
    const deltaY = event.screenY - initialMouseY.value;

    const newWindowX = initialWindowX.value + deltaX;
    const newWindowY = initialWindowY.value + deltaY;

    console.log('Drag Move:');
    console.log('  Current Mouse: (', event.screenX, ', ', event.screenY, ')');
    console.log('  Delta: (', deltaX, ', ', deltaY, ')');
    console.log('  New Window: (', newWindowX, ', ', newWindowY, ')');

    await invoke('set_window_position', { x: newWindowX, y: newWindowY });
  }

  function onDragEnd() {
    isDragging.value = false;
    window.removeEventListener('mousemove', onDragMove);
    window.removeEventListener('mouseup', onDragEnd);
  }

  function onMouseEnter() {
    console.log('Mouse entered');
    isMouseOver.value = true;
    console.log('isMouseOver:', isMouseOver.value);
  }

  function onMouseLeave() {
    console.log('Mouse left');
    if (!isDragging.value) {
      isMouseOver.value = false;
    }
    console.log('isMouseOver:', isMouseOver.value);
  }

  onMounted(() => {
    eventBus.on('key-press', handleKeyPress);
  });

  onUnmounted(() => {
    eventBus.off('key-press', handleKeyPress);
    window.removeEventListener('mousemove', onDragMove);
    window.removeEventListener('mouseup', onDragEnd);
  });
</script>

<template>
  <div
    class="gif-container"
    @mouseenter="onMouseEnter"
    @mouseleave="onMouseLeave"
  >
    <div
      v-show="isMouseOver"
      class="drag-handle"
      @mousedown="onDragStart"
    >
      <svg
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="white"
        stroke-width="2"
      >
        <circle
          cx="12"
          cy="5"
          r="3"
        />
        <circle
          cx="12"
          cy="12"
          r="3"
        />
        <circle
          cx="12"
          cy="19"
          r="3"
        />
      </svg>
    </div>
    <img
      :src="currentGif"
      alt="GIF"
      class="gif-image"
    />
    <div class="info-panel">
      <div class="indicator">
        {{ currentIndex + 1 }} / {{ gifList.length }}
      </div>
      <div class="key-hint">
        Last Key: <span>{{ lastKey || 'Press any key' }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .gif-container {
    width: 100%;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: transparent;
    padding: 20px;
    box-sizing: border-box;
    position: relative;
  }

  .drag-handle {
    position: absolute;
    top: 10px;
    right: 10px;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(10px);
    border-radius: 8px;
    cursor: move;
    transition: background 0.2s;
    z-index: 100;
  }

  .drag-handle:hover {
    background: rgba(0, 0, 0, 0.7);
  }

  .gif-image {
    max-width: 100%;
    /* max-height: 60vh; */
    width: auto;
    height: auto;
    object-fit: contain;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    image-rendering: -webkit-optimize-contrast;
    image-rendering: crisp-edges;
  }

  .info-panel {
    position: fixed;
    top: 10px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 5px;
    align-items: center;
    /* background: rgba(0, 0, 0, 0.5); */
    /* backdrop-filter: blur(10px); */
    padding: 10px;
    /* border-radius: 20px; */
  }

  .indicator {
    color: white;
    font-size: 14px;
    font-weight: 500;
  }

  .key-hint {
    color: rgba(255, 255, 255, 0.8);
    font-size: 14px;
  }

  .key-hint span {
    color: #4ade80;
    font-weight: 600;
  }
</style>