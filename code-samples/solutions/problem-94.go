package main

import (
	"errors"
	"sync"
	"time"
)

type Connection struct {
	ID      int
	Healthy bool
}

type ConnectionPool struct {
	mu          sync.Mutex
	conns       chan *Connection
	factory     func() (*Connection, error)
	healthCheck func(*Connection) bool
	maxSize     int
}

func NewPool(size int, factory func() (*Connection, error), healthCheck func(*Connection) bool) *ConnectionPool {
	pool := &ConnectionPool{
		conns:       make(chan *Connection, size),
		factory:     factory,
		healthCheck: healthCheck,
		maxSize:     size,
	}

	for i := 0; i < size; i++ {
		if conn, err := factory(); err == nil {
			pool.conns <- conn
		}
	}

	go pool.monitor()
	return pool
}

func (p *ConnectionPool) Get() (*Connection, error) {
	select {
	case conn := <-p.conns:
		if p.healthCheck(conn) {
			return conn, nil
		}
		return p.reconnect()
	case <-time.After(5 * time.Second):
		return nil, errors.New("timeout waiting for connection")
	}
}

func (p *ConnectionPool) Put(conn *Connection) {
	select {
	case p.conns <- conn:
	default:
	}
}

func (p *ConnectionPool) reconnect() (*Connection, error) {
	return p.factory()
}

func (p *ConnectionPool) monitor() {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for range ticker.C {
		select {
		case conn := <-p.conns:
			if !p.healthCheck(conn) {
				if newConn, err := p.reconnect(); err == nil {
					p.conns <- newConn
				}
			} else {
				p.conns <- conn
			}
		default:
		}
	}
}
