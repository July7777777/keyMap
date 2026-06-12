import { listen } from '@tauri-apps/api/event';
import { eventBus } from './eventBus';

export interface KeyboardEventData {
  key: string;
  eventType: 'keydown' | 'keyup';
  isDoubleTap: boolean;
  pressCount: number;
}

export function initKeyboardListener() {
  console.log('Initializing keyboard listener...');

  // 监听来自 Rust 的全局键盘事件
  listen<KeyboardEventData>('keyboard-event', (event) => {
    const data = event.payload;
    console.log('Received keyboard event from Rust:', data);
    eventBus.emit('keyboard-event', data);
  });

  // 添加前端键盘监听作为补充（当窗口有焦点时）
  // window.addEventListener('keydown', (event) => {
  //   // 防止默认行为（如页面滚动）
  //   event.preventDefault();
  //   // 将字母键转换为大写，与 Rust 发送的格式保持一致
  //   const key = event.key.toUpperCase();
  //   console.log('Received keyboard event from frontend:', key);
  //   eventBus.emit('keyboard-event', {
  //     key,
  //     eventType: 'keydown' as const,
  //     isDoubleTap: false,
  //     pressCount: 1,
  //   });
  // });

  // window.addEventListener('keyup', (event) => {
  //   event.preventDefault();
  //   const key = event.key.toUpperCase();
  //   console.log('Received keyup event from frontend:', key);
  //   eventBus.emit('keyboard-event', {
  //     key,
  //     eventType: 'keyup' as const,
  //     isDoubleTap: false,
  //     pressCount: 1,
  //   });
  // });
}