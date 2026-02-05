function isPalindrome(str: string): boolean {
  const cleaned = str.toLowerCase().replace(/\s/g, '');
  return cleaned === cleaned.split('').reverse().join('');
}
