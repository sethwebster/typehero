type Table<T> = {
  name: string;
  columns: T;
};

type Query<T, Selected extends keyof T = keyof T> = {
  select<K extends keyof T>(...keys: K[]): Query<T, K>;
  where(condition: Partial<T>): Query<T, Selected>;
  build(): string;
  _phantom?: Pick<T, Selected>;
};

function table<T>(name: string): Table<T> & { query: () => Query<T> } {
  return {
    name,
    columns: {} as T,
    query: () => createQuery<T>(name),
  };
}

function createQuery<T, Selected extends keyof T = keyof T>(
  tableName: string,
  selected: Selected[] = [],
  conditions: Partial<T> = {}
): Query<T, Selected> {
  return {
    select<K extends keyof T>(...keys: K[]) {
      return createQuery<T, K>(tableName, keys, conditions);
    },
    where(condition: Partial<T>) {
      return createQuery<T, Selected>(tableName, selected, { ...conditions, ...condition });
    },
    build() {
      const cols = selected.length ? selected.join(', ') : '*';
      const where = Object.entries(conditions)
        .map(([k, v]) => `${k} = '${v}'`)
        .join(' AND ');
      return `SELECT ${cols} FROM ${tableName}${where ? ` WHERE ${where}` : ''}`;
    },
  };
}

// Usage with type inference
type User = { id: number; name: string; email: string };
const users = table<User>('users');

const query = users.query().select('id', 'name').where({ email: 'test@example.com' });
const sql = query.build(); // Type-safe!
