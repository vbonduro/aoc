pub mod api;
pub mod input;

#[cfg(test)]
mod tests {
    use crate::api;
    #[test]
    fn it_works() {
        let aoc_api = api::Client::new(2021, 1);
        println!("{}", aoc_api.get_input().unwrap());
    }

    #[test]
    fn send_solution() {
        let aoc_api = api::Client::new(2021, 1);
        aoc_api.send_answer(api::PuzzleId::PartOne, 5).unwrap();
    }
}
