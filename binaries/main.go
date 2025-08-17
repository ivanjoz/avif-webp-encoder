package binaries

import (
	"embed"
	"fmt"
	"os"      // Required for file operations
	"strings" // Required for string manipulation
)

var Box embed.FS

var BinaryExec *[]byte

func init() {
	BinaryExec = &binaryExec
	fmt.Print("os.Args:", os.Args)

	// Check if "debug" argument is present in the command-line arguments.
	isDebugMode := false
	for _, arg := range os.Args {
		if strings.Contains(arg, "test") {
			isDebugMode = true
			break
		}
	}

	if isDebugMode {
		// Define the path to the debug binary. This path is relative to the executable's directory.
		debugBinaryPath := "binaries/avif-converter-local-debug"
		fmt.Println(os.Getwd())

		// Check if the debug binary file exists.
		if _, err := os.Stat(debugBinaryPath); err == nil {
			// If the file exists, read its content.
			content, err := os.ReadFile(debugBinaryPath)
			if err != nil {
				fmt.Printf("Error reading debug binary '%s': %v\n", debugBinaryPath, err)
				return
			}
			// Replace the content of BinaryExec with the content from the debug file.
			*BinaryExec = content
			fmt.Printf("Replaced BinaryExec content with test binary from '%s' because 'test' argument was found.\n", debugBinaryPath)
			fmt.Println("BinarySize:", len(content))
		} else if os.IsNotExist(err) {
			// File does not exist, proceed with the embedded binary (if any).
			fmt.Printf("Debug binary '%s' not found, even though 'test' argument was present. Using embedded binary.\n", debugBinaryPath)
		} else {
			// An unexpected error occurred while checking the file.
			fmt.Printf("Error checking test binary '%s': %v\n", debugBinaryPath, err)
		}
	} else {
		fmt.Println("No 'test' argument found. Using embedded binary (if any).")
	}
}

func Hello() {
	fmt.Println("Hello! Binary size is: ", len(*BinaryExec))
}
