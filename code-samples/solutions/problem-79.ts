async function* raceToCompletion<T>(
  promises: Promise<T>[]
): AsyncGenerator<T, void, unknown> {
  const pending = new Map(promises.map((p, i) => [i, p]));

  while (pending.size > 0) {
    const wrapped = Array.from(pending.entries()).map(([index, promise]) =>
      promise.then(
        value => ({ index, value, status: 'fulfilled' as const }),
        error => ({ index, error, status: 'rejected' as const })
      )
    );

    const result = await Promise.race(wrapped);
    pending.delete(result.index);

    if (result.status === 'fulfilled') {
      yield result.value;
    }
  }
}
