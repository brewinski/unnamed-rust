package main

import (
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

	err := RecursePrintDirectory(source, destination)

	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	elapsed := time.Since(start)
	log.Printf("Binomial took %s", elapsed)
}

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
