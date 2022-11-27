package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const horrizontal = 0
const depth = 1
const aim = 2

func update_positions_part1(positions *[2]int, direction string, n_moves int) {
	switch direction {
	case "forward":
		positions[horrizontal] += n_moves
	case "down":
		positions[depth] += n_moves
	case "up":
		positions[depth] -= n_moves
	}
}

func update_positions_part2(positions *[3]int, direction string, n_moves int) {
	switch direction {
	case "forward":
		positions[horrizontal] += n_moves
		positions[depth] += n_moves * positions[aim]
	case "down":
		positions[aim] += n_moves
	case "up":
		positions[aim] -= n_moves
	}
}

func main() {
	f, err := os.Open("input")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var positions [3]int

	for scanner.Scan() {
		instruction_set := strings.Split(scanner.Text(), " ")
		direction := instruction_set[0]
		if n_moves, err := strconv.Atoi(instruction_set[1]); err != nil {
			log.Fatal(err)
		} else {
			update_positions_part2(&positions, direction, n_moves)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("%d\n", positions[horrizontal]*positions[depth])
}
