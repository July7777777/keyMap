<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';
  import { eventBus } from '../utils/eventBus';
  import { invoke } from '@tauri-apps/api/core';
  import type { KeyboardEventData } from '../utils/keyboardListener';

  import { Asura } from '../assets/skillmap.ts';



  // const move = {
  //   l: {
  //     gif: left,
  //     key: '←',
  //     description: '向左走'
  //   },
  //   d: {
  //     gif: down,
  //     key: '↓',
  //     description: '向下走'
  //   },
  //   r: {
  //     gif: right,
  //     key: '→',
  //     description: '向右走'
  //   },
  //   u: {
  //     gif: up,
  //     key: '↑',
  //     description: '向上走'
  //   }
  // }

  // const run = {
  //   r: {
  //     gif: 'https://pic.rmb.bdstatic.com/0bee242d564e26ba033343e166e34d9f.gif',
  //     key: '→',
  //     description: '向右跑'
  //   },
  //   l: {
  //     gif: 'https://pic.rmb.bdstatic.com/bjh/down/fe7e393653abab669185921289a87937.gif',
  //     key: '←',
  //     description: '向左跑'
  //   },
  // }

  const isDragging = ref(false);
  const initialMouseX = ref(0);
  const initialMouseY = ref(0);
  const initialWindowX = ref(0);
  const initialWindowY = ref(0);
  const isMouseOver = ref(false);

  const Role = ref(Asura)
  const lastKeyPressTime = ref(0);
  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  const defined = {
    gif: Role.value.DEFINE.gif,
    key: 'defined',
    name: '站立',
    delay: '0',
  }
  // const att = {
  //   gif: 'https://news-vod.voc.com.cn/9/2023/08/30/e14372b528a2a242b6c81ac489d1bf99446688ac1693397303.jpg?pid=6177335',
  //   key: 'x',
  //   name: '攻击',
  //   delay: '0',
  // }

  const state = ref({
    gif: defined.gif,
    key: defined.key,
    name: defined.name,
    delay: defined.delay,
  });

  function resetToDefault() {
    state.value = defined;
    resetTimer = null;
  }

  function handleKeyPress(data: unknown) {
    const eventData = data as KeyboardEventData;
    console.log('handleKeyPress called with:', eventData);
    const { eventType, isDoubleTap, pressCount, key: originalKey } = eventData;
    let key = originalKey ? originalKey.toUpperCase() : originalKey;

    const keyMap: Record<string, string> = {
      ' ': 'SPACE',
      'SPACE': 'SPACE',
      'ALT': 'ALT',
      'ALTLEFT': 'ALT',
      'ALTRIGHT': 'ALT',
      'CTRL': 'CTRL',
      'CTRLLEFT': 'CTRL',
      'CTRLRIGHT': 'CTRL',
      'SHIFT': 'SHIFT',
      'SHIFTLEFT': 'SHIFTLEFT',
      'SHIFTRIGHT': 'SHIFTRIGHT',
    };

    if (key && key in keyMap) {
      key = keyMap[key];
    }

    console.log('事件类型:' + eventType, '是否双击:' + isDoubleTap, '点击次数:' + pressCount, '键:' + key);

    const currentTime = Date.now();
    const currentDelay = parseFloat(state.value.delay) * 1000;

    if (currentDelay > 0 && currentTime - lastKeyPressTime.value < currentDelay) {
      console.log('Delayed, ignoring key:', key);
      return;
    }

    if (eventType === 'keyup') {
      resetToDefault();
      return;
    }

    lastKeyPressTime.value = currentTime;

    if (resetTimer) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }
    if (eventType === 'keydown') {
      if (key === 'C') return;
      if (isDoubleTap) {
        // 双击事件
        console.log('Double Tap:', key);
        // switch (key) {
        //   case 'LeftArrow':
        //     state.value = run.l;
        //     return;
        //   case 'RightArrow':
        //     // 向右跑
        //     state.value = run.r;
        //     return;
        //   default: state.value = att; state.value.key = key;
        // }
      }
      if (key) {
        console.log('Checking key:', key, 'in Role:', Object.keys(Role.value));
        if (key in Role.value) {
          console.log('Skill found:', Role.value[key]);
          state.value = Role.value[key];
          const skillDelay = parseFloat(state.value.delay) * 1000;
          console.log('Setting skill delay:', skillDelay);
          if (skillDelay > 0) {
            resetTimer = setTimeout(resetToDefault, skillDelay);
          }
          return;
        }
        console.log('Skill not found, using att:', key);
        state.value = defined;
        const attDelay = parseFloat(state.value.delay) * 1000;
        if (attDelay > 0) {
          resetTimer = setTimeout(resetToDefault, attDelay);
        }
      }
      // switch (key) {
      //   case 'C':
      //     state.value = C;
      //     return;
      //   // 单击事件
      //   case 'LeftArrow':
      //     state.value = move.l;
      //     return;
      //   case 'RightArrow':
      //     state.value = move.r;
      //     return;
      //   case 'DownArrow':
      //     state.value = move.d;
      //     return;
      //   case 'UpArrow':
      //     state.value = move.u;
      //     return;

      //   default: state.value = att; state.value.key = key;
      // }
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
    if (resetTimer) {
      clearTimeout(resetTimer);
    }
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
        <span class="description">{{ state.name }}</span>
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
    bottom: 8px;
    left: 50%;
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