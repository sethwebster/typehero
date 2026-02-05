package main

import "strings"

func removeWhitespace(s string) string {
	return strings.ReplaceAll(s, " ", "")
}
