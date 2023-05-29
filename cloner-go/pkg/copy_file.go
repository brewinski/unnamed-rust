package files

import (
	"bytes"
	"fmt"
	"os"
)

func CopyFile(source, destination string) error {
	sourceFile, err := os.ReadFile(source)

	if err != nil {
		return fmt.Errorf("failed to open source file: %w", err)
	}

	sourceFileRep := bytes.ReplaceAll(sourceFile, []byte("m"), []byte("<Z>"))

	err = os.WriteFile(destination, sourceFileRep, 0644) // os.Create(destination)
	if err != nil {
		return fmt.Errorf("failed to create destination file: %w", err)
	}

	return nil
}
