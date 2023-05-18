package files

import (
	"fmt"
	"os"
)

func RecursePrintDirectory(source string, destination string) error {
	// read a file from source path
	sourceDir, err := os.Open(source)
	if err != nil {
		return fmt.Errorf("RecursePrintDirectory: %w", err)
	}
	// close the source directory when the function completes execution
	defer sourceDir.Close()
	// defer fmt.Println("Closing: ", source)

	// check source dir is a directory
	sourceDirInfo, err := sourceDir.Readdir(-1)
	if err != nil {
		return fmt.Errorf("RecursePrintDirectory: %w", err)
	}

	dirErr := os.MkdirAll(destination, os.ModePerm)
	if dirErr != nil {
		return fmt.Errorf("RecursePrintDirectory: %w", dirErr)
	}

	// read the content of the file
	for _, file := range sourceDirInfo {
		if !file.IsDir() {
			CopyFile(source+"/"+file.Name(), destination+"/"+file.Name())
		}
		// check if the file is a directory
		if file.IsDir() {
			RecursePrintDirectory(source+"/"+file.Name(), destination+"/"+file.Name())
		}
	}

	return nil
}
