<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';
  import { eventBus } from '../utils/eventBus';
  import { invoke } from '@tauri-apps/api/core';
  import type { KeyboardEventData } from '../utils/keyboardListener';

  // 三个图片来自百度图片搜索，如有侵权请与我联系

  const down = 'https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fc-ssl.duitang.com%2Fuploads%2Fitem%2F201910%2F08%2F20191008123152_qfquy.thumb.400_0.gif&refer=http%3A%2F%2Fc-ssl.duitang.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=auto?sec=1783845882&t=8eb5c61d587bac670785199af26e4162'
  const right = 'https://img.soogif.com/vqqgKD83MSeEjmw3JHi9nlZvghhtPV56.gif_s400x0'
  const left = 'https://pic.rmb.bdstatic.com/bjh/down/fe7e393653abab669185921289a87937.gif'
  const up = 'https://wx4.sinaimg.cn/bmiddle/006APoFYly1fkeq8ov0zpg308c08cq3u.gif'
  const C_ = 'https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fc-ssl.duitang.com%2Fuploads%2Fitem%2F202003%2F27%2F20200327020300_8UVjz.thumb.400_0.gif&refer=http%3A%2F%2Fc-ssl.duitang.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=auto?sec=1783846708&t=b16640ce6ddd1b2844bec59c563e5e4a'

  const C = {
    gif: C_,
    key: 'C',
    description: '跳跃'
  }
  const att = {
    gif: 'https://news-vod.voc.com.cn/9/2023/08/30/e14372b528a2a242b6c81ac489d1bf99446688ac1693397303.jpg?pid=6177335',
    key: 'x',
    description: '攻击'
  }
  const defined = {
    gif: 'https://pic.rmb.bdstatic.com/bjh/719fbef7d2c545406f7f9d0e3a63cdb52734.gif',
    key: '双手离开键盘',
    description: '站立'
  }

  const move = {
    l: {
      gif: left,
      key: '←',
      description: '向左走'
    },
    d: {
      gif: down,
      key: '↓',
      description: '向下走'
    },
    r: {
      gif: right,
      key: '→',
      description: '向右走'
    },
    u: {
      gif: up,
      key: '↑',
      description: '向上走'
    }
  }
  const run = {
    r: {
      gif: 'https://pic.rmb.bdstatic.com/0bee242d564e26ba033343e166e34d9f.gif',
      key: '→',
      description: '向右跑'
    },
    l: {
      gif: 'https://pic.rmb.bdstatic.com/bjh/down/fe7e393653abab669185921289a87937.gif',
      key: '←',
      description: '向左跑'
    },
  }
  const state = ref({
    gif: defined.gif,
    key: defined.key,
    description: defined.description
  });
  const isDragging = ref(false);
  const initialMouseX = ref(0);
  const initialMouseY = ref(0);
  const initialWindowX = ref(0);
  const initialWindowY = ref(0);
  const isMouseOver = ref(false);

  function handleKeyPress(data: unknown) {
    const eventData = data as KeyboardEventData;
    console.log('handleKeyPress called with:', eventData);
    const { eventType, isDoubleTap, pressCount, key } = eventData;

    console.log('事件类型:' + eventType, '是否双击:' + isDoubleTap, '点击次数:' + pressCount, '键:' + key);
    if (eventType === 'keyup') {
      state.value = defined;
      return;
    }
    if (eventType === 'keydown') {
      if (isDoubleTap) {
        // 双击事件
        console.log('Double Tap:', key);
        switch (key) {
          case 'LeftArrow':
            state.value = run.l;
            return;
          case 'RightArrow':
            // 向右跑
            state.value = run.r;
            return;
          default: state.value = att; state.value.key = key;
        }
      }
      switch (key) {
        case 'C':
          state.value = C;
          return;
        // 单击事件
        case 'LeftArrow':
          state.value = move.l;
          return;
        case 'RightArrow':
          state.value = move.r;
          return;
        case 'DownArrow':
          state.value = move.d;
          return;
        case 'UpArrow':
          state.value = move.u;
          return;
        default: state.value = att; state.value.key = key;
      }
    }


    // currentGif.value = gifList[currentIndex.value];
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

    // console.log('Drag Move:');
    // console.log('  Current Mouse: (', event.screenX, ', ', event.screenY, ')');
    // console.log('  Delta: (', deltaX, ', ', deltaY, ')');
    // console.log('  New Window: (', newWindowX, ', ', newWindowY, ')');

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
    eventBus.on('keyboard-event', handleKeyPress);
  });

  onUnmounted(() => {
    eventBus.off('keyboard-event', handleKeyPress);
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
      :src="state.gif"
      alt="GIF"
      class="gif-image"
    />
    <div class="info-panel">
      <div class="key-hint">
        <span class="key">{{ state.key }}</span>
        <span class="fff">:</span>
        <span class="description">{{ state.description }}</span>
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
    top: 55px;
    left: 57%;
    width: 100%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 5px;
    align-items: center;
    padding: 10px 0;
  }

  .key-hint {
    position: relative;
    display: flex;
    align-items: center;
    width: 200px;
    justify-content: center;
  }

  .key {
    position: absolute;
    right: calc(50% + 8px);
    white-space: nowrap;
  }

  .fff {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
  }

  .description {
    position: absolute;
    left: calc(50% + 8px);
    white-space: nowrap;
  }

  /* .indicator {
    color: white;
    font-size: 14px;
    font-weight: 500;
  } */

  .key,
  .description {
    font-size: 14px;
    font-weight: 600;
  }

  .key {
    color: rgba(255, 255, 255, 0.8);
  }

  .description {
    color: #4ade80;
  }

  .fff {
    color: white;
  }
</style>