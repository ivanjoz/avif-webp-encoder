package binaries

import (
	"embed"
	"fmt"
)

var Box embed.FS

var BinaryExec *[]byte

func init() {
	BinaryExec = &binaryExec
}

func Hello() {
	fmt.Println("Hello! Binary size is: ", len(*BinaryExec))
}
