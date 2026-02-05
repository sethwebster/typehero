type Success<T> = { type: 'success'; value: T };
type Error<E> = { type: 'error'; error: E };
type Loading = { type: 'loading' };

type Result<T, E = string> = Success<T> | Error<E> | Loading;

function match<T, E, R>(
  result: Result<T, E>,
  handlers: {
    success: (value: T) => R;
    error: (error: E) => R;
    loading: () => R;
  }
): R {
  switch (result.type) {
    case 'success':
      return handlers.success(result.value);
    case 'error':
      return handlers.error(result.error);
    case 'loading':
      return handlers.loading();
  }
}

// Usage enforces exhaustive matching
const result: Result<number> = { type: 'success', value: 42 };

const output = match(result, {
  success: (val) => `Got ${val}`,
  error: (err) => `Error: ${err}`,
  loading: () => 'Loading...',
});
