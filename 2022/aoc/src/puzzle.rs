use crate::api::Client;

pub use crate::api::PuzzleId as Id;

pub struct Puzzle {
    client: Client,
    part: Id,
}

impl Puzzle {
    pub fn new(year: u16, day: u8, puzzleid: Id) -> Self {
        Puzzle {
            client: Client::new(year, day),
            part: puzzleid,
        }
    }

    pub fn solve(&self, solver: fn(&String) -> String, dry_run: bool) {
        let answer = solver(&self.client.get_input().unwrap());
        if dry_run {
            println!("Answer: {}", answer);
        } else {
            match self.client.send_answer(self.part, answer.clone()) {
                Ok(()) => println!("🎅 You got the right answer and have earned another star! ⭐"),
                Err(error) => println!(
                    "😢 Your answer {} was not accepted. Here's your error: {}",
                    answer, error
                ),
            }
        }
    }
}
