function allSatisfy<T>(arr: T[], predicate: (item: T) => boolean): boolean {
  return arr.every(predicate);
}

// Usage:
// allSatisfy([2, 4, 6], n => n % 2 === 0) -> true
// allSatisfy([2, 3, 6], n => n % 2 === 0) -> false
