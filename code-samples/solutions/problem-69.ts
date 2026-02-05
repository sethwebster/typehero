class Observable<T> {
  private subscribers: ((value: T) => void)[] = [];

  subscribe(fn: (value: T) => void): () => void {
    this.subscribers.push(fn);
    return () => {
      const idx = this.subscribers.indexOf(fn);
      if (idx > -1) this.subscribers.splice(idx, 1);
    };
  }

  next(value: T): void {
    this.subscribers.forEach(fn => fn(value));
  }
}
