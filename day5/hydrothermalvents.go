package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/vbonduro/aoc/core"
)

type coordinate struct {
	x, y uint
}

func makeCoordinate(line string) coordinate {
	coordStrs := strings.Split(line, ",")
	coordX, _ := strconv.Atoi(coordStrs[0])
	coordY, _ := strconv.Atoi(coordStrs[1])

	var coord coordinate
	coord.x = uint(coordX)
	coord.y = uint(coordY)
	return coord
}

type hydrothermalLine struct {
	from, to coordinate
}

func makeLine(line string) hydrothermalLine {
	var coordX string
	var coordY string
	fmt.Sscanf(line, "%s -> %s", &coordX, &coordY)

	var convertedLine hydrothermalLine
	convertedLine.from = makeCoordinate(coordX)
	convertedLine.to = makeCoordinate(coordY)
	return convertedLine
}

type straightHydrothermalVents struct {
	grid      map[coordinate]uint
	nOverlaps uint
}

func newStraightHydrothermalVents() *straightHydrothermalVents {
	return &straightHydrothermalVents{make(map[coordinate]uint), 0}
}

func (oceanFloor *straightHydrothermalVents) markTheSpot(x coordinate) {
	oceanFloor.grid[x] += 1
	if oceanFloor.grid[x] == 2 {
		oceanFloor.nOverlaps++
	}
}

func (oceanFloor *straightHydrothermalVents) drawVerticalLine(line hydrothermalLine) {
	y_coords := []int{int(line.from.y), int(line.to.y)}
	sort.Ints(y_coords)
	x := line.from.x
	for y := y_coords[0]; y <= y_coords[1]; y++ {
		oceanFloor.markTheSpot(coordinate{x, uint(y)})
	}
}

func (oceanFloor *straightHydrothermalVents) drawHorrizontalLine(line hydrothermalLine) {
	x_coords := []int{int(line.from.x), int(line.to.x)}
	sort.Ints(x_coords)
	y := line.from.y
	for x := x_coords[0]; x <= x_coords[1]; x++ {
		oceanFloor.markTheSpot(coordinate{uint(x), y})
	}
}

func (oceanFloor *straightHydrothermalVents) Input(line string) {
	hydrothermalVentLine := makeLine(line)
	if hydrothermalVentLine.from.x == hydrothermalVentLine.to.x {
		oceanFloor.drawVerticalLine(hydrothermalVentLine)
	} else if hydrothermalVentLine.from.y == hydrothermalVentLine.to.y {
		oceanFloor.drawHorrizontalLine(hydrothermalVentLine)
	}
}

func (oceanFloor *straightHydrothermalVents) Solution() {
	fmt.Println(oceanFloor)
	fmt.Println(oceanFloor.nOverlaps)
}

func main() {
	core.Solve("input.txt", newStraightHydrothermalVents())
}
