package main

import (
	"bytes"
	"fmt"
	"io"
	"os"

	"github.com/amenzhinsky/go-memexec"
	"github.com/ivanjoz/avif-webp-encoder/binaries"
)

func main() {
	for _, args := range os.Args {
		if args == "test" {
			Test()
		}
	}
}

func Test() {
	binaries.Hello()

	exe, err := memexec.New(*binaries.BinaryExec)
	if err != nil {
		panic(err)
	}

	defer exe.Close()

	wd, _ := os.Getwd()
	filePath := wd + "/test_files/demo.webp"

	fmt.Println("Filepath::", filePath)

	if _, err := os.Stat(filePath); err != nil {
		panic(err)
	}

	file, err := os.ReadFile(filePath)
	if err != nil {
		panic(err)
	}

	argv := []string{
		"-image-stdin",
		"-avif",
		"-webp",
		"-resolutions=300",
	}

	cmd := exe.Command(argv...)
	cmd.Stdin = bytes.NewReader(file)

	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		panic(err)
	}

	// Start the command
	err = cmd.Start()
	if err != nil {
		panic(err)
	}

	// Read the output using a ReadCloser
	output := make([]byte, 0)
	buf := make([]byte, 4096) // Adjust the buffer size as needed

	for {
		n, err := stdoutPipe.Read(buf)
		if n > 0 {
			fmt.Println(string(output))
			output = append(output, buf[:n]...)
		}

		if err == io.EOF {
			break
		}

		if err != nil {
			panic(err)
		}
	}

	// Wait for the command to finish
	err = cmd.Wait()
	if err != nil {
		panic(err)
	}
	/*
		output, err := cmd.Output()

		if err != nil {
			panic(err)
		}

		fmt.Println(string(output))
	*/
}
