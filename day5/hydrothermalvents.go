package main

import (
	"fmt"
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

type hydrothermalVents struct {
	grid      map[coordinate]uint
	nOverlaps uint
}

func newHydrothermalVents() *hydrothermalVents {
	return &hydrothermalVents{make(map[coordinate]uint), 0}
}

func (oceanFloor *hydrothermalVents) markTheSpot(x coordinate) {
	oceanFloor.grid[x] += 1
	if oceanFloor.grid[x] == 2 {
		oceanFloor.nOverlaps++
	}
}

func (oceanFloor *hydrothermalVents) draw(line hydrothermalLine) {
	xIncrement := 0
	yIncrement := 0

	if line.from.x < line.to.x {
		xIncrement = 1
	} else if line.from.x > line.to.x {
		xIncrement = -1
	}
	if line.from.y < line.to.y {
		yIncrement = 1
	} else if line.from.y > line.to.y {
		yIncrement = -1
	}

	location := line.from
	for {
		oceanFloor.markTheSpot(location)
		location.x += uint(xIncrement)
		location.y += uint(yIncrement)
		if location == line.to {
			oceanFloor.markTheSpot(location)
			return
		}
	}
}

func (oceanFloor *hydrothermalVents) Input(line string) {
	oceanFloor.draw(makeLine(line))
}

func (oceanFloor *hydrothermalVents) Solution() {
	fmt.Println(oceanFloor.nOverlaps)
}

func main() {
	core.Solve("input.txt", newHydrothermalVents())
}
