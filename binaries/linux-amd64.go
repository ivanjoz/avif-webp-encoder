//go:build linux && amd64

package binaries

import (
	_ "embed"
	"fmt"
)

//go:embed avif-converter
var binaryExec []byte

func init() {
	fmt.Println("Using Binary:", "avif-converter")
}
