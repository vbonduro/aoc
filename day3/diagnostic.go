package main

import (
	"fmt"
	"strconv"

	"github.com/vbonduro/aoc/core"
)

type power struct {
	gamma_rate, epsilon_rate uint64
}

type context struct {
	ones_count     [12]uint
	readings_count uint
}

type diagnostics struct {
	power_consumption power
	calculations      context
}

func (power_consumption *diagnostics) Input(line string) {
	for i, c := range line {
		bit, _ := strconv.Atoi(string(c))
		power_consumption.calculations.ones_count[i] += uint(bit)
	}
	power_consumption.calculations.readings_count++
}

func (diag *diagnostics) Solution() {
	var gamma_bitstr string
	var epsilon_bitstr string
	for _, ones := range diag.calculations.ones_count {
		if ones > (diag.calculations.readings_count / 2) {
			gamma_bitstr += "1"
			epsilon_bitstr += "0"
		} else {
			gamma_bitstr += "0"
			epsilon_bitstr += "1"
		}
	}
	diag.power_consumption.gamma_rate, _ = strconv.ParseUint(gamma_bitstr, 2, 32)
	diag.power_consumption.epsilon_rate, _ = strconv.ParseUint(epsilon_bitstr, 2, 32)
}

func main() {
	var diag diagnostics
	core.Solve("input.txt", &diag)
	fmt.Println(diag.power_consumption.gamma_rate * diag.power_consumption.epsilon_rate)
}
