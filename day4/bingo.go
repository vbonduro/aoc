package main

import (
	"errors"
	"strconv"
)

type board struct {
	score  uint
	grid   [5][5]string
	solved bool
}

func (bingo *board) AddRow(line [5]string, row uint) {
	bingo.grid[row] = line
	for _, value_str := range bingo.grid[row] {
		value, _ := strconv.Atoi(string(value_str))
		bingo.score += uint(value)
	}
}

func (bingo board) find(value uint) (int, int, error) {
	for i, row := range bingo.grid {
		for j, bingo_value_str := range row {
			bingo_value, err := strconv.Atoi(string(bingo_value_str))
			if err == nil && uint(bingo_value) == value {
				return i, j, nil
			}
		}
	}
	return 0, 0, errors.New("Could not find value")
}

func (bingo *board) updateSolved(i int, j int) {
	for x := 0; x < len(bingo.grid); x++ {
		if bingo.grid[x][j] != "x" {
			return
		}
		if bingo.grid[i][x] != "x" {
			return
		}
	}
	bingo.solved = true
}

func (bingo *board) Mark(value uint) {
	i, j, err := bingo.find(value)

	if err == nil {
		value, _ := strconv.Atoi(string(bingo.grid[i][j]))
		bingo.grid[i][j] = "x"
		bingo.score -= uint(value)
		bingo.updateSolved(i, j)
	}
}

type bingo_subsystem struct {
	draw_numbers []uint
	boards       []board
}
