function curry(fn) {
  return function curried(...args) {
    if (args.length >= fn.length) {
      return fn.apply(this, args);
    }
    return (...nextArgs) => curried.apply(this, [...args, ...nextArgs]);
  };
}

// Example: const add = (a, b, c) => a + b + c;
// const curriedAdd = curry(add);
// curriedAdd(1)(2)(3) === curriedAdd(1, 2)(3) === 6
