function parseQueryString(query: string): Record<string, string> {
  return Object.fromEntries(
    new URLSearchParams(query.startsWith('?') ? query.slice(1) : query)
  );
}

// Usage: parseQueryString('?foo=bar&baz=qux') // { foo: 'bar', baz: 'qux' }
