function pipe<T>(...fns: Array<(arg: any) => any>) {
  return (initial: T) => fns.reduce((acc, fn) => fn(acc), initial);
}
