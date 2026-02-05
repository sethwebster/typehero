package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"
)

type ShutdownHandler struct {
	timeout time.Duration
	hooks   []func(context.Context) error
	mu      sync.Mutex
}

func NewShutdownHandler(timeout time.Duration) *ShutdownHandler {
	return &ShutdownHandler{
		timeout: timeout,
		hooks:   make([]func(context.Context) error, 0),
	}
}

func (h *ShutdownHandler) RegisterHook(hook func(context.Context) error) {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.hooks = append(h.hooks, hook)
}

func (h *ShutdownHandler) Wait() {
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)

	<-sigChan
	log.Println("Shutdown signal received, cleaning up...")

	ctx, cancel := context.WithTimeout(context.Background(), h.timeout)
	defer cancel()

	h.mu.Lock()
	hooks := h.hooks
	h.mu.Unlock()

	var wg sync.WaitGroup
	errChan := make(chan error, len(hooks))

	for _, hook := range hooks {
		wg.Add(1)
		go func(fn func(context.Context) error) {
			defer wg.Done()
			if err := fn(ctx); err != nil {
				errChan <- err
			}
		}(hook)
	}

	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
		log.Println("Graceful shutdown completed")
	case <-ctx.Done():
		log.Println("Shutdown timeout exceeded, forcing exit")
	}

	close(errChan)
	for err := range errChan {
		log.Printf("Cleanup error: %v", err)
	}
}

// Usage
func main() {
	handler := NewShutdownHandler(30 * time.Second)

	handler.RegisterHook(func(ctx context.Context) error {
		log.Println("Closing database connections...")
		time.Sleep(2 * time.Second)
		return nil
	})

	handler.RegisterHook(func(ctx context.Context) error {
		log.Println("Flushing metrics...")
		time.Sleep(1 * time.Second)
		return nil
	})

	handler.Wait()
}
