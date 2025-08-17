package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"

	"github.com/ivanjoz/avif-webp-encoder/imageconv"
)

func main() {
	for _, args := range os.Args {
		if args == "test" {
			execTest()
			break
		}
	}
}

func execTest() {
	// Determine the project root, assuming the script is run from the directory
	// that contains 'rust-src' and 'binaries' folders.
	projectRoot, err := os.Getwd()
	if err != nil {
		fmt.Printf("Error getting current directory: %v\n", err)
		os.Exit(1)
	}

	rustSrcPath := filepath.Join(projectRoot, "rust-src")
	binariesPath := filepath.Join(projectRoot, "binaries")

	// Determine the correct executable name based on the operating system
	var executableName string
	if runtime.GOOS == "windows" {
		executableName = "avif-converter.exe"
	} else {
		executableName = "avif-converter"
	}

	// Construct the full path to the built executable
	builtExecutableSrcPath := filepath.Join(rustSrcPath, "target", "release", executableName)

	// Construct the full path for the destination file
	destinationPath := filepath.Join(binariesPath, "avif-converter-local-debug")

	fmt.Printf("Entering %s and executing 'cargo build --release'...\n", rustSrcPath)

	// Execute 'cargo build' within the 'rust-src' directory
	cmd := exec.Command("cargo", "build", "--release")
	cmd.Dir = rustSrcPath
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin

	if err := cmd.Run(); err != nil {
		fmt.Printf("Error during cargo build: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("Cargo build completed successfully.")

	// Verify that the compiled executable exists
	if _, err := os.Stat(builtExecutableSrcPath); os.IsNotExist(err) {
		fmt.Printf("Compiled executable not found at: %s\n", builtExecutableSrcPath)
		os.Exit(1)
	}

	fmt.Printf("Copying \"%s\" to \"%s\"...\n", builtExecutableSrcPath, destinationPath)

	// Copy the compiled file to the binaries folder with the specified new name
	if err := copyFile(builtExecutableSrcPath, destinationPath); err != nil {
		fmt.Printf("Error copying file: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("File copied successfully.")
	imageconv.Test()
}

// copyFile copies a file from src to dst
func copyFile(src, dst string) error {
	sourceFile, err := os.Open(src)
	if err != nil {
		return err
	}
	defer sourceFile.Close()

	destFile, err := os.Create(dst)
	if err != nil {
		return err
	}
	defer destFile.Close()

	_, err = destFile.ReadFrom(sourceFile)
	return err
}
