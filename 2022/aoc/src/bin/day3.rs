use aoc::input;

fn priority(item: char) -> u64 {
    let unicode = item as u64;
    if item.is_uppercase() {
        return unicode - 64 + 26;
    } else {
        return unicode - 96;
    }
}

// Part 1

fn calculate_priority(rucksack: &str) -> u64 {
    let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);
    let common_item = compartment1
        .chars()
        .find(|item| compartment2.contains(*item))
        .unwrap();
    return priority(common_item);
}

fn priority_sum(rucksacks: String) -> u64 {
    return rucksacks
        .lines()
        .into_iter()
        .map(|rucksack| calculate_priority(rucksack))
        .sum();
}

// Part 2

fn calculate_elf_group_priority(group: &[&str]) -> u64 {
    let mut elf_group = group.to_vec();
    elf_group.sort_by(|a, b| b.len().cmp(&a.len()));
    let common_item = elf_group[0]
        .chars()
        .find(|item| elf_group[1].contains(*item) && elf_group[2].contains(*item))
        .unwrap();
    return priority(common_item);
}

fn priority_sum_elf_groups(rucksacks: String) -> u64 {
    return rucksacks
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(calculate_elf_group_priority)
        .sum();
}

fn main() {
    println!("Part 1: {}", priority_sum(input::get(2022, 3).unwrap()));
    println!(
        "Part 2: {}",
        priority_sum_elf_groups(input::get(2022, 3).unwrap())
    );
}

mod tests {
    use crate::calculate_elf_group_priority;
    use crate::calculate_priority;
    #[test]
    fn priority_calculation_sample() {
        let rucksack = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        println!("{}", calculate_priority(rucksack));
    }
    #[test]
    fn test_calculate_elf_group_priority() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        println!(
            "{}",
            calculate_elf_group_priority(rucksack.lines().collect::<Vec<&str>>().as_slice())
        );
    }
}
