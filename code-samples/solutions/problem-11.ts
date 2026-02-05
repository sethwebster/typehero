function takeFirst<T>(arr: T[], n: number): T[] {
  return arr.slice(0, n);
}

// Usage:
// takeFirst([1, 2, 3, 4, 5], 3) -> [1, 2, 3]
// takeFirst([1, 2], 5) -> [1, 2]
