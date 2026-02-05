class Semaphore {
  constructor(maxConcurrent) {
    this.maxConcurrent = maxConcurrent;
    this.current = 0;
    this.queue = [];
  }

  async acquire() {
    if (this.current < this.maxConcurrent) {
      this.current++;
      return;
    }

    await new Promise(resolve => this.queue.push(resolve));
  }

  release() {
    this.current--;

    if (this.queue.length > 0) {
      this.current++;
      const resolve = this.queue.shift();
      resolve();
    }
  }

  async run(fn) {
    await this.acquire();
    try {
      return await fn();
    } finally {
      this.release();
    }
  }
}

// Usage:
// const sem = new Semaphore(3);
// await Promise.all(
//   tasks.map(task => sem.run(() => task()))
// );
