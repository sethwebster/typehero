async function retry<T>(
  fn: () => Promise<T>,
  attempts: number,
  delay: number
): Promise<T> {
  try {
    return await fn();
  } catch (err) {
    if (attempts <= 1) throw err;
    await new Promise(resolve => setTimeout(resolve, delay));
    return retry(fn, attempts - 1, delay);
  }
}
