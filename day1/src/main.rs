use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("./input.txt").unwrap();
    let elves = parse(input);
    let mut calories_carried = elves
        .into_iter()
        .map(|elf| elf.calories_carried())
        .collect::<Vec<i64>>();
    calories_carried.sort();
    calories_carried.reverse();
    let max: i64 = calories_carried.iter().take(1).sum();
    let top_three : i64 = calories_carried.iter().take(3).sum();

    println!("Part 1: {}", max);
    println!("Part 2: {}", top_three);
}

pub trait CaloriesCarried {
    fn calories_carried(&self) -> i64;
}
#[derive(Debug, PartialEq)]
struct Elf {
    items: Vec<i64>,
}

impl CaloriesCarried for Elf {
    fn calories_carried(&self) -> i64 {
        self.items.iter().sum()
    }
}

fn read_input(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect())
}

fn parse(input: Vec<String>) -> Vec<Elf> {
    let chunks = input.into_iter().fold(Vec::new(), |mut acc, x| {
        if x.len() == 0 || acc.is_empty() {
            acc.push(Vec::new());
        }
        if x.len() > 0 {
            acc.last_mut().unwrap().push(x.parse::<i64>().unwrap());
        }
        acc
    });
    chunks.into_iter().map(|items| Elf { items }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![], 0)]
    #[test_case(vec![0], 0)]
    #[test_case(vec![1], 1)]
    #[test_case(vec![1,2], 3)]
    fn test_calories_counted(items: Vec<i64>, expected: i64) {
        let elf = Elf { items };
        let calories_carried = elf.calories_carried();

        assert_eq!(calories_carried, expected)
    }

    #[test_case(vec!["1".to_string()], vec![Elf{ items:vec![1]}])]
    #[test_case(vec!["1".to_string(), "".to_string(), "2".to_string()], vec![Elf{ items:vec![1]}, Elf{ items:vec![2]}])]
    #[test_case(vec!["1".to_string(), "2".to_string(), "".to_string(), "3".to_string()], vec![Elf{ items:vec![1,2]}, Elf{ items:vec![3]}])]
    fn test_parse(input: Vec<String>, expected: Vec<Elf>) {
        let elves = parse(input);
        assert_eq!(elves, expected)
    }
}
