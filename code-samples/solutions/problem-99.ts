interface Repository<T> {
  findById(id: string): Promise<T | null>;
  findAll(filter?: Partial<T>): Promise<T[]>;
  create(data: Omit<T, 'id'>): Promise<T>;
  update(id: string, data: Partial<T>): Promise<T>;
  delete(id: string): Promise<void>;
}

type QueryBuilder<T> = {
  where<K extends keyof T>(key: K, value: T[K]): QueryBuilder<T>;
  orderBy<K extends keyof T>(key: K, direction: 'asc' | 'desc'): QueryBuilder<T>;
  limit(count: number): QueryBuilder<T>;
  execute(): Promise<T[]>;
};

class GenericRepository<T extends { id: string }> implements Repository<T> {
  constructor(private db: Map<string, T>) {}

  async findById(id: string): Promise<T | null> {
    return this.db.get(id) || null;
  }

  async findAll(filter?: Partial<T>): Promise<T[]> {
    const items = Array.from(this.db.values());

    if (!filter) return items;

    return items.filter(item =>
      Object.entries(filter).every(([key, value]) => item[key as keyof T] === value)
    );
  }

  async create(data: Omit<T, 'id'>): Promise<T> {
    const id = Math.random().toString(36);
    const item = { id, ...data } as T;
    this.db.set(id, item);
    return item;
  }

  async update(id: string, data: Partial<T>): Promise<T> {
    const existing = await this.findById(id);
    if (!existing) throw new Error('Not found');

    const updated = { ...existing, ...data };
    this.db.set(id, updated);
    return updated;
  }

  async delete(id: string): Promise<void> {
    this.db.delete(id);
  }

  query(): QueryBuilder<T> {
    let filters: Partial<T> = {};
    let ordering: { key: keyof T; dir: 'asc' | 'desc' } | null = null;
    let limitCount: number | null = null;

    return {
      where: (key, value) => {
        filters[key] = value;
        return this;
      },
      orderBy: (key, direction) => {
        ordering = { key, dir: direction };
        return this;
      },
      limit: (count) => {
        limitCount = count;
        return this;
      },
      execute: async () => {
        let results = await this.findAll(filters);

        if (ordering) {
          results.sort((a, b) => {
            const aVal = a[ordering!.key];
            const bVal = b[ordering!.key];
            const cmp = aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
            return ordering!.dir === 'asc' ? cmp : -cmp;
          });
        }

        if (limitCount) {
          results = results.slice(0, limitCount);
        }

        return results;
      },
    } as QueryBuilder<T>;
  }
}
