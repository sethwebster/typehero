function zip<T, U>(arr1: T[], arr2: U[]): [T, U][] {
  const length = Math.min(arr1.length, arr2.length);
  return Array.from({ length }, (_, i) => [arr1[i], arr2[i]]);
}

// Example: zip([1, 2, 3], ['a', 'b', 'c']) === [[1, 'a'], [2, 'b'], [3, 'c']]
