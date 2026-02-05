function sortBy(arr, keys) {
  return arr.sort((a, b) => {
    for (const { key, dir = 'asc' } of keys) {
      const mult = dir === 'asc' ? 1 : -1;
      if (a[key] < b[key]) return -1 * mult;
      if (a[key] > b[key]) return 1 * mult;
    }
    return 0;
  });
}
