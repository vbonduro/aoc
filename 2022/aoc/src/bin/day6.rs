use aoc::puzzle;

use std::collections::HashMap;
/*
lsi=3
seen=mjq
mjqjpqmgbljsphdztnvjfqwrcgsmlb
...
 */
fn start_of_frame_location(datastream: &String) -> String {
    let mut seen_chars = Vec::new();
    for (i, c) in datastream.chars().enumerate() {
        println!("i={} c={} seen_chars={:?}", i, c, seen_chars);
        let maybe_index = seen_chars.iter().position(|&r| r == c);
        if let Some(index) = maybe_index {
            seen_chars.drain(..index + 1);
        }
        seen_chars.push(c);
        if seen_chars.len() == 14 {
            return (i + 1).to_string();
        }
    }
    return (-1).to_string();
}

fn main() {
    let dry_run = false;
    //puzzle::Puzzle::new(2022, 6, puzzle::Id::PartOne).solve(start_of_frame_location, dry_run);
    puzzle::Puzzle::new(2022, 6, puzzle::Id::PartTwo).solve(start_of_frame_location, dry_run);
}

mod tests {
    use crate::start_of_frame_location;
    #[test]
    fn test_sample_1() {
        let code = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let sof = start_of_frame_location(&code);
        assert_eq!(sof, 7.to_string());
    }

    #[test]
    fn test_sample_2() {
        let code = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let sof = start_of_frame_location(&code);
        assert_eq!(sof, 5.to_string());
    }
}
