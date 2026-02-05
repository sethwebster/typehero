type Validator<T> = (value: T) => boolean | string;

class Schema<T> {
  private validators: Validator<T>[] = [];

  test(fn: Validator<T>): this {
    this.validators.push(fn);
    return this;
  }

  validate(value: T): { valid: boolean; errors: string[] } {
    const errors: string[] = [];

    for (const validator of this.validators) {
      const result = validator(value);
      if (result !== true) {
        errors.push(typeof result === 'string' ? result : 'Validation failed');
      }
    }

    return { valid: errors.length === 0, errors };
  }
}

// Constraint builders
const string = () => new Schema<string>()
  .test(v => typeof v === 'string' || 'Must be string');

const number = () => new Schema<number>()
  .test(v => typeof v === 'number' || 'Must be number');

const min = (n: number) => (v: number) => v >= n || `Must be >= ${n}`;
const max = (n: number) => (v: number) => v <= n || `Must be <= ${n}`;
const minLength = (n: number) => (v: string) => v.length >= n || `Min length ${n}`;
const pattern = (regex: RegExp) => (v: string) => regex.test(v) || 'Invalid format';

// Usage:
// const schema = string().test(minLength(3)).test(pattern(/^[a-z]+$/));
// schema.validate('abc'); // { valid: true, errors: [] }
