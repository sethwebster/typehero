package main

import "sync"

type WorkerPool struct {
	workers   int
	taskQueue chan func()
	wg        sync.WaitGroup
}

func NewWorkerPool(workers int) *WorkerPool {
	pool := &WorkerPool{
		workers:   workers,
		taskQueue: make(chan func(), 100),
	}
	pool.start()
	return pool
}

func (p *WorkerPool) start() {
	for i := 0; i < p.workers; i++ {
		go func() {
			for task := range p.taskQueue {
				task()
				p.wg.Done()
			}
		}()
	}
}

func (p *WorkerPool) Submit(task func()) {
	p.wg.Add(1)
	p.taskQueue <- task
}

func (p *WorkerPool) Wait() {
	p.wg.Wait()
}

func (p *WorkerPool) Close() {
	close(p.taskQueue)
}
