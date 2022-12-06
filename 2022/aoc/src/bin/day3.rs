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

// fn priority_sum_elf_groups(rucksacks: String) -> u64 {
//     return rucksacks
//         .lines()
//         .array_chunks()
//         .into_iter()
//         .map(|group| calculate_elf_group_priority(group))
//         .sum();
// }

fn main() {
    println!("{}", priority_sum(input::get(2022, 3).unwrap()));
}

mod tests {
    use crate::calculate_priority;
    #[test]
    fn priority_calculation_sample() {
        let rucksack = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        println!("{}", calculate_priority(rucksack));
    }
}
