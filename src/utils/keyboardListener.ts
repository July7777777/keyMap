import { listen } from '@tauri-apps/api/event';
import { eventBus } from './eventBus';

export function initKeyboardListener() {
  console.log('Initializing keyboard listener...');

  // 监听来自 Rust 的全局键盘事件
  listen<string>('keyboard-event', (event) => {
    const key = event.payload;
    console.log('Received keyboard event from Rust:', key);
    eventBus.emit('key-press', key);
  });

  // 添加前端键盘监听作为补充（当窗口有焦点时）
  window.addEventListener('keydown', (event) => {
    // 防止默认行为（如页面滚动）
    event.preventDefault();
    // 将字母键转换为大写，与 Rust 发送的格式保持一致
    const key = event.key.toUpperCase();
    console.log('Received keyboard event from frontend:', key);
    eventBus.emit('key-press', key);
  });
}