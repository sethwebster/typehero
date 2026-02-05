class Parser {
  private pos = 0;
  private text: string;

  constructor(text: string) {
    this.text = text.replace(/\s+/g, '');
  }

  parse(): number {
    return this.parseExpression();
  }

  private parseExpression(): number {
    let result = this.parseTerm();
    while (this.pos < this.text.length && (this.text[this.pos] === '+' || this.text[this.pos] === '-')) {
      const op = this.text[this.pos++];
      const right = this.parseTerm();
      result = op === '+' ? result + right : result - right;
    }
    return result;
  }

  private parseTerm(): number {
    let result = this.parseFactor();
    while (this.pos < this.text.length && (this.text[this.pos] === '*' || this.text[this.pos] === '/')) {
      const op = this.text[this.pos++];
      const right = this.parseFactor();
      result = op === '*' ? result * right : result / right;
    }
    return result;
  }

  private parseFactor(): number {
    if (this.text[this.pos] === '(') {
      this.pos++;
      const result = this.parseExpression();
      this.pos++; // skip ')'
      return result;
    }
    let num = '';
    while (this.pos < this.text.length && /[0-9.]/.test(this.text[this.pos])) {
      num += this.text[this.pos++];
    }
    return parseFloat(num);
  }
}
