package main

import (
	f "cloner_go/pkg"
	"fmt"
	"log"
	"os"
	"time"
)

func main() {
	start := time.Now()
	// accept comand line arguments source & destination
	source := os.Args[1]
	destination := os.Args[2]

	err := f.RecursiveCopy(source, destination)

	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	elapsed := time.Since(start)
	log.Printf("Binomial took %s", elapsed)
}
