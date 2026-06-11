type Callback = (data: unknown) => void;

const listeners: Record<string, Set<Callback>> = {};

export const eventBus = {
  on(event: string, callback: Callback) {
    if (!listeners[event]) {
      listeners[event] = new Set();
    }
    listeners[event].add(callback);
    console.log(`Added listener for event: ${event}, total: ${listeners[event].size}`);
  },

  emit(event: string, data: unknown) {
    console.log(`Emitting event: ${event}, listeners: ${listeners[event]?.size || 0}`);
    listeners[event]?.forEach(callback => {
      try {
        callback(data);
      } catch (error) {
        console.error('Error in event callback:', error);
      }
    });
  },

  off(event: string, callback: Callback) {
    listeners[event]?.delete(callback);
  }
};