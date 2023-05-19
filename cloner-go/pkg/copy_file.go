package files

import (
	"fmt"
	"io"
	"os"
)

func CopyFile(source string, destination string) error {
	sourceFile, err := os.Open(source)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}
	defer sourceFile.Close()

	destinationFile, err := os.Create(destination)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}
	defer func() {
		cerr := destinationFile.Close()
		if err == nil {
			err = cerr
		}
	}()

	_, err = io.Copy(destinationFile, sourceFile)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}

	err = destinationFile.Chmod(0777)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}

	return nil
}
