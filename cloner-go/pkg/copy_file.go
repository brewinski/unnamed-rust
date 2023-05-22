package files

import (
	"bytes"
	"fmt"
	"os"
)

func CopyFile(source, destination string) error {
	sourceFile, err := os.ReadFile(source) // os.Open(source)
	if err != nil {
		return fmt.Errorf("failed to open source file: %w", err)
	}

	sourceFileRep := bytes.ReplaceAll(sourceFile, []byte("m"), []byte("<Z>"))

	err = os.WriteFile(destination, sourceFileRep, 0644) // os.Create(destination)
	if err != nil {
		return fmt.Errorf("failed to create destination file: %w", err)
	}

	// _, err = io.Copy(destinationFile, sourceFile)
	// if err != nil {
	// 	return fmt.Errorf("failed to copy file contents: %w", err)
	// }

	// // Set the file permissions (e.g., 0644)
	// err = destinationFile.Chmod(0644)
	// if err != nil {
	// 	return fmt.Errorf("failed to set file permissions: %w", err)
	// }

	return nil
}
