type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

class ResultType<T, E> {
  constructor(private result: Result<T, E>) {}

  static ok<T, E>(value: T): ResultType<T, E> {
    return new ResultType({ ok: true, value });
  }

  static err<T, E>(error: E): ResultType<T, E> {
    return new ResultType({ ok: false, error });
  }

  map<U>(fn: (value: T) => U): ResultType<U, E> {
    return this.result.ok
      ? ResultType.ok(fn(this.result.value))
      : ResultType.err(this.result.error);
  }

  flatMap<U>(fn: (value: T) => ResultType<U, E>): ResultType<U, E> {
    return this.result.ok ? fn(this.result.value) : ResultType.err(this.result.error);
  }
}
