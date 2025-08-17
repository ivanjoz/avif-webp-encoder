package imageconv

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"strings"

	"github.com/amenzhinsky/go-memexec"
	"github.com/ivanjoz/avif-webp-encoder/binaries"
)

func Convert(args ImageConvertInput) ([]Image, error) {
	if args.UseDebugLogs {
		fmt.Println("binary size::", len(*binaries.BinaryExec))
	}

	if len(args.Image) == 0 || len(args.ImagePath) > 0 {
		wd, _ := os.Getwd()
		if args.ImagePath[0:1] == "/" {
			args.ImagePath = "/" + args.ImagePath
		}

		filePath := wd + args.ImagePath

		if _, err := os.Stat(filePath); err != nil {
			return nil, fmt.Errorf("error file not found: %v", filePath)
		}

		var err error
		args.Image, err = os.ReadFile(filePath)
		if err != nil {
			return nil, fmt.Errorf("error reading image file: %v", err)
		}
	}

	resolutions := []string{}
	for _, e := range args.Resolutions {
		resolutions = append(resolutions, fmt.Sprintf("%d", e))
	}

	argv := []string{
		"-image-stdin",
		"-resolutions=" + strings.Join(resolutions, ","),
	}

	if args.UseAvif || args.AvifQuality > 0 || args.AvifSpeed > 0 {
		argv = append(argv, "-avif")
	}
	if args.UseWebp || args.WebpQuality > 0 || args.WebpMethod > 0 {
		argv = append(argv, "-webp")
	}
	if args.UseThumbhash > 0 {
		argv = append(argv, fmt.Sprintf("-thumbhash=%v", args.UseThumbhash))
	}

	var cmd *exec.Cmd

	if len(args.TempDirOfExecution) > 0 {
		binaryPath := args.TempDirOfExecution + "imageconv"
		fmt.Println("Creating binary on path...", binaryPath)

		err := os.WriteFile(binaryPath, *binaries.BinaryExec, 0777)
		if err != nil {
			return nil, fmt.Errorf("error writing binary: %v", err)
		}

		err = os.Chmod(binaryPath, 0777)
		if err != nil {
			return nil, fmt.Errorf("error setting permissions on binary: %v", err)
		}

		cmd = exec.Command(binaryPath, argv...)
	} else {
		fmt.Println("Executing binary on memory...")

		exe, err := memexec.New(*binaries.BinaryExec)
		if err != nil {
			return nil, fmt.Errorf("error reading image file: %v", err)
		}

		defer exe.Close()
		cmd = exe.Command(argv...)
	}

	outputImages := []Image{}

	cmd.Stdin = bytes.NewReader(args.Image)

	stdoutPipe, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("error creating stdout pipe: %v", err)
	}

	err = cmd.Start()
	if err != nil {
		return nil, fmt.Errorf("error starting execution binary: %v", err)
	}

	// Read the output using a ReadCloser
	if args.StdoutBufferSize == 0 {
		args.StdoutBufferSize = 1024 * 48
	}
	buf := make([]byte, args.StdoutBufferSize)
	jsonBytes := []byte{}

	for {
		n, err := stdoutPipe.Read(buf)

		if n > 0 {
			msg := buf[:n]

			if len(msg) > 2 && strings.Contains(string(msg[len(msg)-3:]), `"}`) {
				jsonBytes = append(jsonBytes, msg...)

				rec := imageStdOutput{}
				err := json.Unmarshal(jsonBytes, &rec)
				if err != nil {
					if len(jsonBytes) > 200 {
						fmt.Println(string(jsonBytes[:100]), "...", string(jsonBytes[len(jsonBytes)-100:]))
					}
					return nil, fmt.Errorf("error parsing JSON output from binary: %v", err)
				}

				if args.UseDebugLogs {
					fmt.Printf("Image Converted: Name: %v, Size: %v, Format: %v, Resolution: %v\n", rec.Name, int(float32(len(rec.ImageBase64)-1)*0.75), rec.Format, rec.Resolution)
				}

				jsonBytes = []byte{}

				image := Image{
					Name:       rec.Name,
					Format:     rec.Format,
					Resolution: rec.Resolution,
				}

				image.Content, err = base64.StdEncoding.DecodeString(rec.ImageBase64)
				if err != nil {
					return nil, fmt.Errorf("error decoding output image from binary: %v", err)
				}

				outputImages = append(outputImages, image)
			} else if len(jsonBytes) > 0 ||
				(len(msg) > 10 && string(msg[0:10]) == `{"image":"`) {
				jsonBytes = append(jsonBytes, msg...)
			} else {
				if args.UseDebugLogs {
					fmt.Println(string(msg))
				}
			}
		}

		if err == io.EOF {
			break
		}

		if err != nil {
			return nil, fmt.Errorf("error converting image: %v", err)
		}
	}

	// Wait for the command to finish
	err = cmd.Wait()
	if err != nil {
		return nil, fmt.Errorf("error closing binary execution: %v", err)
	}

	return outputImages, nil
}

type ImageConvertInput struct {
	Image              []byte   // Image as binary
	ImagePath          string   // Path or name of the image
	Resolutions        []uint16 // Slice of resolutions. Example 800 mean 800x800px density
	UseWebp            bool     // Not necesary if WebpQuality or WebpMethod is configured
	WebpQuality        uint8    // From 1 - 100
	WebpMethod         uint8    // From 1 - 6; default 6 (best quality, slowest)
	UseAvif            bool     // Not necesary if AvifSpeed or AvifQuality is configured
	AvifSpeed          uint8    // From 1 - 11; default 2 (more speed, less quality)
	AvifQuality        uint8    // From 1 - 100
	StdoutBufferSize   int      // Default 1024*1000
	OutputDirectory    string   // If want to save the images to directory
	UseDebugLogs       bool     // Default false
	TempDirOfExecution string
	UseThumbhash       uint8 // 1 = normal tumbhash | 2 = force square tumbhash
}

type Image struct {
	Content    []byte
	Name       string
	Resolution int32
	Format     string
}

type imageStdOutput struct {
	ImageBase64 string `json:"image"`
	Name        string `json:"name"`
	Resolution  int32  `json:"resolution"`
	Format      string `json:"format"`
}
