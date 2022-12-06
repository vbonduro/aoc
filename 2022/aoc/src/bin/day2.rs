use aoc::input;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

type Err = ();

impl FromStr for Shape {
    type Err = ();

    fn from_str(coded_shape: &str) -> Result<Shape, Self::Err> {
        match coded_shape {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn from_strategy(opponent: &Shape, strategy: &str) -> Result<Shape, Err> {
        match strategy {
            "X" => Ok(Shape::losing_shape(opponent)),
            "Y" => Ok(opponent.clone()),
            "Z" => Ok(Shape::winning_shape(opponent)),
            _ => Err(()),
        }
    }

    fn winning_shape(other: &Shape) -> Shape {
        match other {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn losing_shape(other: &Shape) -> Shape {
        match other {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn beats(&self, other: &Shape) -> bool {
        match *self {
            Shape::Rock => other == &Shape::Scissors,
            Shape::Paper => other == &Shape::Rock,
            Shape::Scissors => other == &Shape::Paper,
        }
    }

    fn value(&self) -> u64 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn score_round(you: &Shape, opponent: &Shape) -> u64 {
    if you == opponent {
        return you.value() + 3;
    } else if you.beats(opponent) {
        return you.value() + 6;
    } else {
        return you.value();
    }
}

fn decode_part1(encoded_play: &str) -> u64 {
    let plays: Vec<Shape> = encoded_play
        .split(" ")
        .map(|shape| Shape::from_str(shape).unwrap())
        .collect();
    return score_round(&plays[1], &plays[0]);
}

fn decode_part2(encoded_play: &str) -> u64 {
    let encoded_shapes: Vec<&str> = encoded_play.split(" ").collect();
    let opponent_shape = Shape::from_str(encoded_shapes[0]).unwrap();
    let my_shape = Shape::from_strategy(&opponent_shape, encoded_shapes[1]).unwrap();
    return score_round(&my_shape, &opponent_shape);
}

fn total_score(strategy_guide: String) -> u64 {
    return strategy_guide
        .lines()
        .into_iter()
        .map(|encoded_play| decode_part2(encoded_play))
        .sum();
}

fn main() {
    println!("{}", total_score(input::get(2022, 2).unwrap()));
}

#[cfg(test)]
mod tests {
    use crate::decode;
    use crate::Shape;
    #[test]
    fn rock_beats_scissors() {
        let mine = Shape::Rock;
        let theirs = Shape::Scissors;
        assert!(mine.beats(&theirs));
    }

    #[test]
    fn rock_loses_to_paper() {
        let mine = Shape::Rock;
        let theirs = Shape::Paper;
        assert!(!mine.beats(&theirs));
    }

    #[test]
    fn rock_doesnt_beat_rock() {
        let mine = Shape::Rock;
        let theirs = Shape::Rock;
        assert!(!mine.beats(&theirs));
    }

    #[test]
    fn decoded_rock_wins_score() {
        let encoded = "C X";
        assert!(7 == decode(encoded));
    }
}
