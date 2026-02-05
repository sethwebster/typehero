package main

import (
	"sync"
	"time"
)

type RateLimiter struct {
	mu         sync.Mutex
	timestamps []time.Time
	maxRequests int
	window     time.Duration
}

func NewRateLimiter(maxRequests int, window time.Duration) *RateLimiter {
	return &RateLimiter{
		timestamps:  make([]time.Time, 0),
		maxRequests: maxRequests,
		window:     window,
	}
}

func (rl *RateLimiter) Allow() bool {
	rl.mu.Lock()
	defer rl.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-rl.window)

	// Remove expired timestamps
	valid := 0
	for _, ts := range rl.timestamps {
		if ts.After(cutoff) {
			rl.timestamps[valid] = ts
			valid++
		}
	}
	rl.timestamps = rl.timestamps[:valid]

	if len(rl.timestamps) < rl.maxRequests {
		rl.timestamps = append(rl.timestamps, now)
		return true
	}

	return false
}
