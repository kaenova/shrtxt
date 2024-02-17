package datainput

import "os"

func EnvDefault(key, value string) string {
	v := os.Getenv(key)
	if v == "" {
		return value
	}
	return v
}
