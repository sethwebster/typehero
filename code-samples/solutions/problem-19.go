package main

import "strings"

func padString(s string, minLen int, padChar rune) string {
	if len(s) >= minLen {
		return s
	}
	padding := strings.Repeat(string(padChar), minLen-len(s))
	return padding + s
}

// Usage:
// padString("42", 5, '0') -> "00042"
