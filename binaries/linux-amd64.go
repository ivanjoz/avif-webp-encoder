//go:build linux && amd64

package binaries

import (
	_ "embed"
)

//go:embed avif-converter
var binaryExec []byte

func init() {

}
