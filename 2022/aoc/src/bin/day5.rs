use aoc::puzzle;
use regex::Regex;
use std::collections::LinkedList;

type Crate = char;
type Stack = LinkedList<Crate>;

struct LoadingDock {
    stacks: Vec<Stack>,
}

impl LoadingDock {
    pub fn from_drawing(layout: &Vec<&str>) -> Self {
        let mut dock = LoadingDock { stacks: Vec::new() };
        layout.into_iter().for_each(|line| dock.from_line(line));
        dock
    }

    pub fn rearrange(&mut self, ncrates: usize, from_stack: usize, to_stack: usize) {
        for _ in 0..ncrates {
            let crate_id = self.stacks[from_stack - 1].pop_back().unwrap();
            self.stacks[to_stack - 1].push_back(crate_id);
        }
    }

    pub fn rearrange_9001(&mut self, ncrates: usize, from_stack: usize, to_stack: usize) {
        let split_location = self.stacks[from_stack - 1].len() - ncrates;
        let mut moved_crates = self.stacks[from_stack - 1].split_off(split_location);
        self.stacks[to_stack - 1].append(&mut moved_crates);
    }

    pub fn peek(self) -> String {
        self.stacks
            .into_iter()
            .filter(|stack| !stack.is_empty())
            .map(|stack| *stack.back().unwrap())
            .collect()
    }

    fn from_line(&mut self, line: &str) {
        line.chars()
            .into_iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(stack_id, crate_id)| self.add_crate(stack_id, crate_id));
    }

    fn add_crate(&mut self, stack_id: usize, crate_id: Crate) {
        if self.stacks.len() < stack_id + 1 {
            self.stacks.push(Stack::new());
        }
        if !crate_id.is_whitespace() {
            self.stacks[stack_id].push_front(crate_id);
        }
    }
}

fn top_of_stacks(instructions: &String) -> String {
    let instruction_set = instructions.lines().collect::<Vec<&str>>();
    let drawing_end = instruction_set.iter().position(|&x| x.is_empty()).unwrap();

    let mut loading_dock = LoadingDock::from_drawing(&instruction_set[..drawing_end - 1].to_vec());

    let rearrangement_expr = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    instruction_set[drawing_end + 1..]
        .iter()
        .for_each(|instruction| {
            println!("{}", instruction);
            let parsed_instruction = rearrangement_expr.captures(instruction).unwrap();
            loading_dock.rearrange_9001(
                parsed_instruction.get(1).unwrap().as_str().parse().unwrap(),
                parsed_instruction.get(2).unwrap().as_str().parse().unwrap(),
                parsed_instruction.get(3).unwrap().as_str().parse().unwrap(),
            );
        });

    loading_dock.peek()
}

fn main() {
    let dry_run = false;
    //puzzle::Puzzle::new(2022, 5, puzzle::Id::PartOne).solve(top_of_stacks, dry_run);
    puzzle::Puzzle::new(2022, 5, puzzle::Id::PartTwo).solve(top_of_stacks, dry_run);
}
