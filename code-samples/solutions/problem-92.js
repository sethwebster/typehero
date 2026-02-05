class ReactiveStream {
  constructor(bufferSize = 10) {
    this.subscribers = [];
    this.buffer = [];
    this.bufferSize = bufferSize;
    this.paused = false;
  }

  subscribe(handler) {
    this.subscribers.push(handler);
    return () => {
      this.subscribers = this.subscribers.filter(h => h !== handler);
    };
  }

  async next(value) {
    if (this.buffer.length >= this.bufferSize) {
      this.paused = true;
      await this.waitForDrain();
    }

    this.buffer.push(value);
    await this.emit(value);
  }

  async emit(value) {
    await Promise.all(this.subscribers.map(async handler => {
      await handler(value);
      this.buffer.shift();

      if (this.buffer.length < this.bufferSize / 2 && this.paused) {
        this.paused = false;
      }
    }));
  }

  waitForDrain() {
    return new Promise(resolve => {
      const check = setInterval(() => {
        if (!this.paused) {
          clearInterval(check);
          resolve();
        }
      }, 10);
    });
  }
}

module.exports = ReactiveStream;
