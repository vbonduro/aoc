use aoc::input;
use std::cmp;

fn calories_per_elf(inventory: String) -> Vec<u64> {
    let mut elves_calories = vec![];
    let mut elf_calories: u64 = 0;
    for line in inventory.lines() {
        if line.is_empty() {
            elves_calories.push(elf_calories);
            elf_calories = 0;
        } else {
            elf_calories += line.parse::<u64>().unwrap();
        }
    }
    elves_calories
}

fn total_top_3(inventory: String) -> u64 {
    let mut elves_calories = calories_per_elf(inventory);
    elves_calories.sort();
    return elves_calories[elves_calories.len() - 3..].iter().sum();
}

fn max_calories(inventory: String) -> u64 {
    let mut max: u64 = 0;
    let mut elf_calories: u64 = 0;
    for line in inventory.lines() {
        if line.is_empty() {
            elf_calories = 0;
        } else {
            elf_calories += line.parse::<u64>().unwrap();
            max = cmp::max(max, elf_calories);
        }
    }
    return max;
}

fn main() {
    println!("{}", total_top_3(input::get(2022, 1).unwrap()));
}
