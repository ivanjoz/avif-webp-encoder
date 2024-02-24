//go:build linux && arm64

package binaries

import (
	_ "embed"
	"fmt"
)

//go:embed linux-arm64
var binaryExec []byte

func init() {
	fmt.Println("Using Binary:", "linux-arm64")
}
