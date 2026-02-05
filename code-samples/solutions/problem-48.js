class EventEmitter {
  constructor() {
    this.events = {};
  }

  on(event, handler) {
    (this.events[event] = this.events[event] || []).push(handler);
    return () => this.off(event, handler);
  }

  off(event, handler) {
    if (!this.events[event]) return;
    this.events[event] = this.events[event].filter(h => h !== handler);
  }

  emit(event, ...args) {
    (this.events[event] || []).forEach(handler => handler(...args));
  }
}
