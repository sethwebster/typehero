async function runConcurrent(tasks, maxConcurrency) {
  const results = [];
  const executing = [];

  for (const [index, task] of tasks.entries()) {
    const promise = Promise.resolve().then(() => task()).then(
      result => ({ index, result, status: 'fulfilled' }),
      error => ({ index, error, status: 'rejected' })
    );

    results.push(promise);
    executing.push(promise);

    const cleanup = promise.then(() => {
      executing.splice(executing.indexOf(cleanup), 1);
    });

    if (executing.length >= maxConcurrency) {
      await Promise.race(executing);
    }
  }

  return Promise.all(results);
}
