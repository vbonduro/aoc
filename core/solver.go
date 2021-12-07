package core

import (
	"bufio"
	"log"
	"os"
)

type solver interface {
	Input(line string)
	Solution()
}

func Solve(input_file string, calculator solver) {
	f, err := os.Open(input_file)

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		calculator.Input(scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	calculator.Solution()
}
