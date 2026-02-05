package main

import "container/list"

type LRUCache struct {
	capacity int
	cache    map[string]*list.Element
	order    *list.List
}

type entry struct {
	key   string
	value interface{}
}

func NewLRU(capacity int) *LRUCache {
	return &LRUCache{
		capacity: capacity,
		cache:    make(map[string]*list.Element),
		order:    list.New(),
	}
}

func (c *LRUCache) Get(key string) (interface{}, bool) {
	if elem, ok := c.cache[key]; ok {
		c.order.MoveToFront(elem)
		return elem.Value.(*entry).value, true
	}
	return nil, false
}

func (c *LRUCache) Set(key string, value interface{}) {
	if elem, ok := c.cache[key]; ok {
		c.order.MoveToFront(elem)
		elem.Value.(*entry).value = value
		return
	}

	if c.order.Len() >= c.capacity {
		oldest := c.order.Back()
		if oldest != nil {
			c.order.Remove(oldest)
			delete(c.cache, oldest.Value.(*entry).key)
		}
	}

	elem := c.order.PushFront(&entry{key, value})
	c.cache[key] = elem
}
