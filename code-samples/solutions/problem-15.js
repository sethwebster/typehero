function uniqueValues(arr, key) {
  return [...new Set(arr.map(obj => obj[key]))];
}

// Usage:
// uniqueValues([{id: 1}, {id: 2}, {id: 1}], 'id') -> [1, 2]
