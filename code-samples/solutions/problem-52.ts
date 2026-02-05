function longestCommonPrefix(strs: string[]): string {
  if (strs.length === 0) return "";

  for (let i = 0; i < strs[0].length; i++) {
    const char = strs[0][i];
    if (!strs.every(s => s[i] === char)) {
      return strs[0].substring(0, i);
    }
  }

  return strs[0];
}

// Example: longestCommonPrefix(["flower", "flow", "flight"]) === "fl"
