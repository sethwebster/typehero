type JSONValue = string | number | boolean | null | JSONObject | JSONArray;
interface JSONObject { [key: string]: JSONValue; }
interface JSONArray extends Array<JSONValue> {}

type Schema =
  | { type: 'string'; minLength?: number; maxLength?: number }
  | { type: 'number'; min?: number; max?: number }
  | { type: 'boolean' }
  | { type: 'null' }
  | { type: 'array'; items: Schema }
  | { type: 'object'; properties: Record<string, Schema>; required?: string[] };

function validate(value: JSONValue, schema: Schema): boolean {
  switch (schema.type) {
    case 'string':
      if (typeof value !== 'string') return false;
      if (schema.minLength && value.length < schema.minLength) return false;
      if (schema.maxLength && value.length > schema.maxLength) return false;
      return true;

    case 'number':
      if (typeof value !== 'number') return false;
      if (schema.min !== undefined && value < schema.min) return false;
      if (schema.max !== undefined && value > schema.max) return false;
      return true;

    case 'boolean':
      return typeof value === 'boolean';

    case 'null':
      return value === null;

    case 'array':
      if (!Array.isArray(value)) return false;
      return value.every(item => validate(item, schema.items));

    case 'object':
      if (typeof value !== 'object' || value === null || Array.isArray(value)) return false;
      const obj = value as JSONObject;

      if (schema.required) {
        for (const key of schema.required) {
          if (!(key in obj)) return false;
        }
      }

      for (const [key, val] of Object.entries(obj)) {
        const propSchema = schema.properties[key];
        if (propSchema && !validate(val, propSchema)) return false;
      }

      return true;
  }
}
