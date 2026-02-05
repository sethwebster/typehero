package main

import (
	"encoding/base64"
	"encoding/json"
	"errors"
	"strings"
)

func validateJWTStructure(token string) error {
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		return errors.New("invalid JWT: must have 3 parts")
	}

	for i := 0; i < 2; i++ {
		decoded, err := base64.RawURLEncoding.DecodeString(parts[i])
		if err != nil {
			return err
		}
		var js json.RawMessage
		if err := json.Unmarshal(decoded, &js); err != nil {
			return errors.New("invalid JSON in JWT part")
		}
	}
	return nil
}
