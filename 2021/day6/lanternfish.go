package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/vbonduro/aoc/core"
)

type lanternFish struct {
	spawnTimer uint
	count      uint
}

func newLanternFish() *lanternFish {
	babyFish := lanternFish{8, 1}
	return &babyFish
}

func (fish *lanternFish) advanceDay() bool {
	if fish.spawnTimer == 0 {
		fish.spawnTimer = 6
		return true
	} else {
		fish.spawnTimer--
	}
	return false
}

type school struct {
	fishes []lanternFish
}

func (model *school) advanceDay() {
	var babyFish *lanternFish
	babyFish = nil
	for fishi := range model.fishes {
		fish := &model.fishes[fishi]
		spawnFish := fish.advanceDay()
		if spawnFish {
			if babyFish == nil {
				babyFish = newLanternFish()
				babyFish.count = fish.count
			} else {
				babyFish.count += fish.count
			}
		}
	}
	if babyFish != nil {
		model.fishes = append(model.fishes, *babyFish)
	}
}

func (model *school) count() uint {
	nfishes := 0
	for _, fish := range model.fishes {
		nfishes += int(fish.count)
	}
	return uint(nfishes)
}

func (model *school) Input(line string) {
	fishyStrings := strings.Split(line, ",")
	for _, fishyString := range fishyStrings {
		spawnTimer, _ := strconv.Atoi(fishyString)
		model.fishes = append(model.fishes, lanternFish{uint(spawnTimer), 1})
	}
}

func (model *school) Solution() {
	for i := 0; i < 256; i++ {
		model.advanceDay()
		fmt.Println(model.count())
	}
	fmt.Println(model.count())
}

func main() {
	model := school{}
	core.Solve("input.txt", &model)
}
