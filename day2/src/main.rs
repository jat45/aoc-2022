use std::io::{BufRead, BufReader};
use std::str::FromStr;
use strum_macros::EnumString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("./input.txt")?;

    let shape_scores = input.iter().map(|round| shape_score(round.us));
    let round_scores = input
        .iter()
        .map(|round| round.outcome())
        .map(|outcome| outcome_score(outcome));

    // println!("shape scores {:?}", shape_scores);
    // println!("round scores {:?}", round_scores);
    let part_1 = shape_scores.sum::<i64>() + round_scores.sum::<i64>();
    let part_2 = input.iter().map(|round| round.part_2_score()).collect::<Vec<i64>>();

    // println!("round scores {:?}", part_2);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2.iter().sum::<i64>());
    Ok(())
}

#[derive(Debug, PartialEq, EnumString, Copy, Clone)]
enum Shape {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

fn shape_score(shape: Shape) -> i64 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent: Shape,
    us: Shape,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

fn outcome_score(outcome: Outcome) -> i64 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (self.opponent, self.us) {
            // losses
            (Shape::Rock, Shape::Scissors) => Outcome::Loss,
            (Shape::Paper, Shape::Rock) => Outcome::Loss,
            (Shape::Scissors, Shape::Paper) => Outcome::Loss,
            // draws
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
            // wins
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
        }
    }

    fn part_2_score(&self) -> i64 {
        let base_score = match self.us {
            Shape::Rock => 0,     // lose
            Shape::Paper => 3,    // draw
            Shape::Scissors => 6, // win
        };

        let we_should_play = match (self.opponent, self.us) {
            (Shape::Rock, Shape::Rock) => Shape::Scissors,
            (Shape::Rock, Shape::Paper) => Shape::Rock,
            (Shape::Rock, Shape::Scissors) => Shape::Paper,

            (Shape::Paper, Shape::Rock) => Shape::Rock,
            (Shape::Paper, Shape::Paper) => Shape::Paper,
            (Shape::Paper, Shape::Scissors) => Shape::Scissors,

            (Shape::Scissors, Shape::Rock) => Shape::Paper,
            (Shape::Scissors, Shape::Paper) => Shape::Scissors,
            (Shape::Scissors, Shape::Scissors) => Shape::Rock,
        };

        base_score + shape_score(we_should_play)
    }
}

fn parse_round(line: String) -> Result<Round, Box<dyn std::error::Error>> {
    let parse = line.split(" ").collect::<Vec<&str>>();
    let opponent = Shape::from_str(parse[0])?;
    let us = Shape::from_str(parse[1])?;
    Ok(Round { opponent, us })
}

fn read_input(filepath: &str) -> Result<Vec<Round>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .map(|line| parse_round(line).unwrap())
        .collect())
}

// fn parse(input: Vec<String>) -> Vec<Elf> {
//     let chunks = input.into_iter().fold(Vec::new(), |mut acc, x| {
//         if x.len() == 0 || acc.is_empty() {
//             acc.push(Vec::new());
//         }
//         if x.len() > 0 {
//             acc.last_mut().unwrap().push(x.parse::<i64>().unwrap());
//         }
//         acc
//     });
//     chunks.into_iter().map(|items| Elf { items }).collect()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("A A".to_string(), Round { opponent: Shape::Rock, us: Shape::Rock})]
    #[test_case("B X".to_string(), Round { opponent: Shape::Paper, us: Shape::Rock})]
    #[test_case("C X".to_string(), Round { opponent: Shape::Scissors, us: Shape::Rock})]
    #[test_case("A X".to_string(), Round { opponent: Shape::Rock, us: Shape::Rock})]
    #[test_case("A Y".to_string(), Round { opponent: Shape::Rock, us: Shape::Paper})]
    #[test_case("A Z".to_string(), Round { opponent: Shape::Rock, us: Shape::Scissors})]
    fn test_parse_round(line: String, expected: Round) {
        if let Ok(round) = parse_round(line) {
            assert_eq!(round, expected)
        } else {
            panic!("got an error")
        }
    }

    #[test_case(Shape::Rock, 1)]
    #[test_case(Shape::Paper, 2)]
    #[test_case(Shape::Scissors, 3)]
    fn test_score_shape(shape: Shape, expected: i64) {
        let score = shape_score(shape);
        assert_eq!(score, expected)
    }

    #[test_case(Round { opponent: Shape::Paper, us: Shape::Rock}, Outcome::Loss)]
    #[test_case(Round { opponent: Shape::Rock, us: Shape::Rock}, Outcome::Draw)]
    #[test_case(Round { opponent: Shape::Rock, us: Shape::Paper}, Outcome::Win)]
    fn test_outcome(round: Round, expected: Outcome) {
        assert_eq!(round.outcome(), expected)
    }
}
