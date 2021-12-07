package main

import (
	"fmt"
	"strconv"

	"github.com/vbonduro/aoc/core"
)

type context struct {
	ones_count     [12]uint
	readings_count uint
}

type power_diagnostics struct {
	gamma_rate, epsilon_rate uint64
	calculations             context
}

func (power_consumption *power_diagnostics) Input(line string) {
	for i, c := range line {
		bit, _ := strconv.Atoi(string(c))
		power_consumption.calculations.ones_count[i] += uint(bit)
	}
	power_consumption.calculations.readings_count++
}

func (diag *power_diagnostics) Solution() {
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
	diag.gamma_rate, _ = strconv.ParseUint(gamma_bitstr, 2, 32)
	diag.epsilon_rate, _ = strconv.ParseUint(epsilon_bitstr, 2, 32)
}

type life_support_rating struct {
	oxygen_rating, co2_scrubber_rating  uint64
	diagnostics_ones, diagnostics_zeros []string
}

func (life_support *life_support_rating) Input(line string) {
	if line[0] == '0' {
		life_support.diagnostics_zeros = append(life_support.diagnostics_zeros, line)
	} else {
		life_support.diagnostics_ones = append(life_support.diagnostics_ones, line)
	}
}

func SplitOnesAndZeros(list []string, index uint) ([]string, []string) {
	var ones []string
	var zeros []string
	for _, item := range list {
		if item[index] == '0' {
			zeros = append(zeros, item)
		} else {
			ones = append(ones, item)
		}
	}
	return ones, zeros
}

func FindOxygenRating(search_set []string, index uint) string {
	var ones []string
	var zeros []string

	if len(search_set) == 1 {
		return search_set[0]
	}

	ones, zeros = SplitOnesAndZeros(search_set, index)

	index++
	if len(zeros) > len(ones) {
		return FindOxygenRating(zeros, index)
	} else {
		return FindOxygenRating(ones, index)
	}
}

func FindCo2Rating(search_set []string, index uint) string {
	var ones []string
	var zeros []string

	if len(search_set) == 1 {
		return search_set[0]
	}

	ones, zeros = SplitOnesAndZeros(search_set, index)

	index++
	if len(zeros) > len(ones) {
		return FindCo2Rating(ones, index)
	} else {
		return FindCo2Rating(zeros, index)
	}
}

func (life_support *life_support_rating) Solution() {
	var oxygen_rating_bitstring string
	var co2_rating_bitstring string
	if len(life_support.diagnostics_zeros) > len(life_support.diagnostics_ones) {
		oxygen_rating_bitstring = FindOxygenRating(life_support.diagnostics_zeros, 1)
		co2_rating_bitstring = FindCo2Rating(life_support.diagnostics_ones, 1)
	} else {
		oxygen_rating_bitstring = FindOxygenRating(life_support.diagnostics_ones, 1)
		co2_rating_bitstring = FindCo2Rating(life_support.diagnostics_zeros, 1)
	}
	life_support.oxygen_rating, _ = strconv.ParseUint(oxygen_rating_bitstring, 2, 32)
	life_support.co2_scrubber_rating, _ = strconv.ParseUint(co2_rating_bitstring, 2, 32)
}

func main() {
	var diag power_diagnostics
	core.Solve("input.txt", &diag)
	fmt.Println(diag.gamma_rate * diag.epsilon_rate)

	var life_support life_support_rating
	core.Solve("input.txt", &life_support)
	fmt.Println(life_support.oxygen_rating * life_support.co2_scrubber_rating)
}
