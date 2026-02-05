package main

import (
	"sync"
	"sync/atomic"
)

type Singleton struct {
	data string
}

var (
	instance *Singleton
	once     sync.Once
	initFlag uint32
)

func GetInstance() *Singleton {
	// Fast path: already initialized
	if atomic.LoadUint32(&initFlag) == 1 {
		return instance
	}

	// Slow path: double-checked locking
	once.Do(func() {
		instance = &Singleton{
			data: "initialized",
		}
		atomic.StoreUint32(&initFlag, 1)
	})

	return instance
}

// Alternative: simpler sync.Once only (no atomic optimization)
func GetInstanceSimple() *Singleton {
	once.Do(func() {
		instance = &Singleton{data: "initialized"}
	})
	return instance
}
