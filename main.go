package main

import (
	"os"

	"github.com/ivanjoz/avif-webp-encoder/imageconv"
)

func main() {
	for _, args := range os.Args {
		if args == "test" {
			imageconv.Test()
		}
	}
}
