function camelToSnake(str) {
  return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

// Example: camelToSnake("myVariableName") === "my_variable_name"
