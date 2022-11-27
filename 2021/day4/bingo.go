package main

import (
	"errors"
	"fmt"
	"strconv"
	"strings"

	"github.com/vbonduro/aoc/core"
)

type board struct {
	score  uint
	card   [][]string
	solved bool
}

func (bingo *board) AddRow(line []string) {
	bingo.card = append(bingo.card, line)
	row := len(bingo.card) - 1
	for _, value_str := range bingo.card[row] {
		value, _ := strconv.Atoi(string(value_str))
		bingo.score += uint(value)
	}
}

func (bingo board) find(value uint) (int, int, error) {
	for i, row := range bingo.card {
		for j, bingo_value_str := range row {
			bingo_value, err := strconv.Atoi(string(bingo_value_str))
			if err == nil && uint(bingo_value) == value {
				return i, j, nil
			}
		}
	}
	return 0, 0, errors.New("could not find value")
}

func (bingo board) checkSolvedRow(col int) bool {
	for row := 0; row < len(bingo.card); row++ {
		if bingo.card[row][col] != "x" {
			return false
		}
	}
	return true
}

func (bingo board) checkSolvedColumn(row int) bool {
	for col := 0; col < len(bingo.card); col++ {
		if bingo.card[row][col] != "x" {
			return false
		}
	}
	return true
}

func (bingo board) checkSolved(i int, j int) bool {
	return bingo.checkSolvedRow(j) || bingo.checkSolvedColumn(i)
}

func (bingo *board) Mark(value uint) {
	i, j, err := bingo.find(value)

	if err == nil {
		bingo.card[i][j] = "x"
		bingo.score = bingo.score - value
		bingo.solved = bingo.checkSolved(i, j)
	}
}

type bingo_subsystem struct {
	draw_numbers []uint
	boards       []board
}

func (squid_game *bingo_subsystem) parseDrawNumbers(line string) {
	draw_num_strs := strings.Split(line, ",")
	for _, draw_num_str := range draw_num_strs {
		draw_num, _ := strconv.Atoi(draw_num_str)
		squid_game.draw_numbers = append(squid_game.draw_numbers, uint(draw_num))
	}
}

func (squid_game *bingo_subsystem) Input(line string) {
	if len(squid_game.draw_numbers) == 0 {
		squid_game.parseDrawNumbers(line)
	} else {
		if len(line) == 0 {
			var bingo board
			squid_game.boards = append(squid_game.boards, bingo)
		} else {
			row_values := strings.Fields(line)
			squid_game.boards[len(squid_game.boards)-1].AddRow(row_values)
		}
	}
}

func (squid_game *bingo_subsystem) Solution() {
	for _, drawn_number := range squid_game.draw_numbers {
		for i := range squid_game.boards {
			bingo := &squid_game.boards[i]
			if !bingo.solved {
				bingo.Mark(drawn_number)
				if bingo.solved {
					fmt.Println(bingo.score * drawn_number)
				}
			}
		}
	}
}

func main() {
	var squid_game bingo_subsystem
	core.Solve("inputs.txt", &squid_game)
}
