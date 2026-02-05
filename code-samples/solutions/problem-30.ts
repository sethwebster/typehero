function lastOrDefault<T>(arr: T[], defaultValue: T): T {
  return arr.length > 0 ? arr[arr.length - 1] : defaultValue;
}
