function flattenToDot(obj, prefix = '') {
  return Object.entries(obj).reduce((acc, [key, val]) => {
    const newKey = prefix ? `${prefix}.${key}` : key;
    if (val && typeof val === 'object' && !Array.isArray(val)) {
      Object.assign(acc, flattenToDot(val, newKey));
    } else {
      acc[newKey] = val;
    }
    return acc;
  }, {});
}
