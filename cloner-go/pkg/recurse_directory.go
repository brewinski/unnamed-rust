package files

import (
	"fmt"
	"os"
	"sync"
)

func RecursiveCopy(source string, destination string) error {
	sourceInfo, err := os.Stat(source)
	if err != nil {
		return fmt.Errorf("failed to get source file/directory information: %w", err)
	}

	if !sourceInfo.IsDir() {
		return CopyFile(source, destination)
	}

	err = os.MkdirAll(destination, sourceInfo.Mode())
	if err != nil {
		return fmt.Errorf("failed to create destination directory: %w", err)
	}

	sourceDir, err := os.Open(source)
	if err != nil {
		return fmt.Errorf("failed to open source directory: %w", err)
	}
	defer sourceDir.Close()

	sourceFiles, err := sourceDir.Readdir(-1)
	if err != nil {
		return fmt.Errorf("failed to read source directory: %w", err)
	}

	wg := sync.WaitGroup{}
	wg.Add(len(sourceFiles))

	for _, file := range sourceFiles {
		sourcePath := source + "/" + file.Name()
		destinationPath := destination + "/" + file.Name()

		if file.IsDir() {
			go func(src string, dest string) {
				defer wg.Done()
				_ = RecursiveCopy(sourcePath, destinationPath)
			}(sourcePath, destinationPath)
		} else {
			go func(src string, dest string) {
				defer wg.Done()
				_ = CopyFile(sourcePath, destinationPath)
			}(sourcePath, destinationPath)
		}
	}

	wg.Wait()
	return nil
}
