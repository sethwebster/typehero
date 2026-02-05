class Middleware {
  constructor() {
    this.stack = [];
  }

  use(fn) {
    this.stack.push(fn);
    return this;
  }

  async execute(context) {
    let index = 0;

    const next = async () => {
      if (index >= this.stack.length) return;
      const fn = this.stack[index++];
      await fn(context, next);
    };

    await next();
  }
}

// Usage:
// const mw = new Middleware();
// mw.use(async (ctx, next) => { console.log('before 1'); await next(); console.log('after 1'); });
// mw.use(async (ctx, next) => { console.log('before 2'); await next(); console.log('after 2'); });
// await mw.execute({ data: 'test' });
