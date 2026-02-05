function groupBy(arr, key) {
  return arr.reduce((acc, obj) => {
    const value = obj[key];
    (acc[value] ??= []).push(obj);
    return acc;
  }, {});
}

// Usage: groupBy([{type: 'a', v: 1}, {type: 'b', v: 2}, {type: 'a', v: 3}], 'type')
