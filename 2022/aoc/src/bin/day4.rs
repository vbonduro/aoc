use aoc::api;

#[derive(Copy, Clone, Debug)]
struct SectionRange {
    start: u64,
    end: u64,
}

impl SectionRange {
    fn contains(&self, other: SectionRange) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: SectionRange) -> bool {
        return other.start <= self.end && self.start <= other.end;
    }

    fn from_str(range_str: &str) -> Self {
        let range: Vec<u64> = range_str
            .split("-")
            .into_iter()
            .map(|int_str| int_str.parse::<u64>().unwrap())
            .collect();
        SectionRange {
            start: range[0],
            end: range[1],
        }
    }
}

// Part1

fn has_redundancy(cleanup_assignment: &str) -> u64 {
    let ranges = cleanup_assignment
        .split(",")
        .into_iter()
        .map(|section_range| SectionRange::from_str(section_range))
        .collect::<Vec<SectionRange>>();
    println!("{:?}", ranges);
    return (ranges[0].contains(ranges[1]) || ranges[1].contains(ranges[0])) as u64;
}

fn total_redundant_cleanups(section_assignments: String) -> u64 {
    section_assignments
        .lines()
        .into_iter()
        .map(|cleanup_assignment| has_redundancy(cleanup_assignment))
        .sum()
}

// Part2

fn has_overlap(cleanup_assignment: &str) -> u64 {
    let ranges = cleanup_assignment
        .split(",")
        .into_iter()
        .map(|section_range| SectionRange::from_str(section_range))
        .collect::<Vec<SectionRange>>();
    println!("{:?}", ranges);
    return ranges[0].overlaps(ranges[1]) as u64;
}

fn total_overlaps(section_assignments: String) -> u64 {
    section_assignments
        .lines()
        .into_iter()
        .map(|cleanup_assignment| has_overlap(cleanup_assignment))
        .sum()
}

fn main() {
    let aoc_client = api::Client::new(2022, 4);
    // Part1
    // let redundant_cleanups = total_redundant_cleanups(aoc_client.get_input().unwrap());
    // println!("{}", redundant_cleanups);
    // aoc_client
    //     .send_answer(api::PuzzleId::PartOne, redundant_cleanups)
    //     .unwrap();
    // Part2
    let overlaps = total_overlaps(aoc_client.get_input().unwrap());
    println!("{}", overlaps);
    aoc_client
        .send_answer(api::PuzzleId::PartTwo, overlaps.to_string())
        .unwrap();
}

mod tests {
    use crate::has_redundancy;
    #[test]
    fn test_has_redundancy() {
        let assignment = "2-8,3-9";
        println!("{}", has_redundancy(assignment));
    }
}
