package main

import (
	"fmt"

	"github.com/ivanjoz/avif-webp-encoder/binaries"
)

func test() {
	binaries.Hello()
	input := ImageConvertInput{
		ImagePath:    "/test_files/demo.webp",
		Resolutions:  []uint16{300, 700},
		UseWebp:      true,
		UseAvif:      true,
		useDebugLogs: true,
	}

	images, err := Convert(input)
	if err != nil {
		panic(err)
	}
	fmt.Println("Images converted:: ", len(images))
	for _, e := range images {
		fmt.Printf("Image Converted: Name: %v, Size: %v, Format: %v, Resolution: %v\n", e.Name, len(e.Content), e.Format, e.Resolution)
	}
}
