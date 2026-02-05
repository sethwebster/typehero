package main

import (
	"context"
	"time"
)

type ContextManager struct {
	timeout time.Duration
}

func NewContextManager(timeout time.Duration) *ContextManager {
	return &ContextManager{timeout: timeout}
}

func (cm *ContextManager) WithTimeout(parent context.Context) (context.Context, context.CancelFunc) {
	return context.WithTimeout(parent, cm.timeout)
}

func (cm *ContextManager) WithCancel(parent context.Context) (context.Context, context.CancelFunc) {
	return context.WithCancel(parent)
}

func (cm *ContextManager) WithDeadline(parent context.Context, deadline time.Time) (context.Context, context.CancelFunc) {
	return context.WithDeadline(parent, deadline)
}

// Usage:
// cm := NewContextManager(5 * time.Second)
// ctx, cancel := cm.WithTimeout(context.Background())
// defer cancel()
//
// select {
// case <-time.After(10 * time.Second):
//     // Work done
// case <-ctx.Done():
//     // Timeout or cancelled
// }
