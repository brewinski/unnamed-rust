package main

import (
	f "cloner_go/pkg"
	"fmt"
	"io"
	"log"
	"os"
	"time"
)

func main() {
	start := time.Now()
	// accept comand line arguments source & destination
	source := os.Args[1]
	destination := os.Args[2]

	err := f.RecursePrintDirectory(source, destination)

	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	elapsed := time.Since(start)
	log.Printf("Binomial took %s", elapsed)
}

func CopyFile(source string, destination string) error {
	// fmt.Println("Copying: ", source, " to ", destination)
	// open the source file
	sourceFile, err := os.Open(source)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}
	// close the source file when the function completes execution
	defer sourceFile.Close()
	// create a new file in the destination
	destinationFile, err := os.Create(destination)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}
	// close the destination file when the function completes execution
	defer func() {
		cerr := destinationFile.Close()
		if err == nil {
			err = cerr
		}
	}()

	// copy the content of the source file to the destination file
	_, err = io.Copy(destinationFile, sourceFile)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}
	// set the file permission mode
	err = os.Chmod(destination, 0777)
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}
	// copy the file permissions
	err = destinationFile.Sync()
	if err != nil {
		return fmt.Errorf("CopyFile: %w", err)
	}

	return nil
}