function groupByParity(numbers) {
  return numbers.reduce((acc, num) => {
    acc[num % 2 === 0 ? 'even' : 'odd'].push(num);
    return acc;
  }, { even: [], odd: [] });
}
