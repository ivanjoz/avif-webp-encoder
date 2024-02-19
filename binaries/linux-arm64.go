//go:build linux && arm64

package binaries

import (
	_ "embed"
)

//go:embed linux-arm64
var binaryExec []byte

func init() {

}
