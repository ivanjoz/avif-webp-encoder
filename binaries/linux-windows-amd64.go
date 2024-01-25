//go:build windows && amd64

package binaries

import (
	_ "embed"
)

//go:embed linux-amd64
var binaryExec []byte

func init() {

}
