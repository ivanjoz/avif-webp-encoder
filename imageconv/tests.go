package imageconv

import (
	"fmt"
	"log"
	"os"

	"github.com/ivanjoz/avif-webp-encoder/binaries"
)

// avif conversion based on https://github.com/xiph/rav1e
func Test() {
	binaries.Hello()

	input := ImageConvertInput{
		ImagePath:    "/test_files/demo2.webp",
		Resolutions:  []uint16{340, 820},
		UseWebp:      true,
		UseAvif:      true,
		UseDebugLogs: true,
		UseThumbhash: 1,
	}

	images, err := Convert(input)
	if err != nil {
		panic(err)
	}
	fmt.Println("Images converted:: ", len(images))
	wd, _ := os.Getwd()

	for _, e := range images {
		outputFileName := wd + "/test_outputs/" + e.Name
		fmt.Println("Saving image::", outputFileName)
		f, err := os.Create(outputFileName)
		if err != nil {
			log.Fatal(err)
		}

		defer f.Close()
		_, err = f.Write(e.Content)

		if err != nil {
			log.Fatal(err)
		}
	}
}
