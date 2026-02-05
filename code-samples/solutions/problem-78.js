function jsonPath(obj, path) {
  const segments = path.split('.').flatMap(seg => {
    // Handle array notation: users[0].name
    const match = seg.match(/^([^\[]+)(?:\[(\d+)\])?$/);
    return match[2] !== undefined ? [match[1], parseInt(match[2])] : [seg];
  });

  return segments.reduce((current, segment) => {
    if (current === undefined || current === null) return undefined;

    if (segment === '*') {
      return Array.isArray(current)
        ? current
        : Object.values(current);
    }

    if (segment === '**') {
      // Deep search - collect all matching values
      const collect = (obj, results = []) => {
        if (Array.isArray(obj)) {
          obj.forEach(item => collect(item, results));
        } else if (typeof obj === 'object' && obj !== null) {
          Object.values(obj).forEach(val => {
            results.push(val);
            collect(val, results);
          });
        }
        return results;
      };
      return collect(current);
    }

    return current[segment];
  }, obj);
}
