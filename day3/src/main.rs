fn main() {
    println!("Hello, world!");
}
se std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let part1 = include_str!("input.txt")
    .lines()
    .map(line_parse)
    .map(|(a,b)| overlap_score(a,b))
    .collect::<Vec<u32>>();
    println!("part 1: {part1:?}");
    println!("part 1: {}", part1.iter().sum::<u32>());

    let part2 = include_str!("input.txt")
    .lines()
    .map(line_parse)
    .chunks(3).into_iter()
    .map(|i| i.fold(None, |acc: Option<HashSet<char>>, (a,b)| {
        let u : HashSet<char> = HashSet::from_iter(a.union(&b).map(|c| *c));
        if let Some(intersection) = acc {
            Some(HashSet::from_iter(intersection.intersection(&u).map(|c| *c)))
        } else {
            Some(u)
        }
    }))
    .map(|x| {
        if let Some(badge) = x {
            if badge.len() == 1 {
                badge.iter().map(|c| score(*c)).sum()
            } else { 
            panic!("badge didn't have exactly one element {badge:?}");
            0
            }
        } else {
            panic!("no badges found!");
            0
        }
    })
    .collect::<Vec<_>>();
    println!("part 2: {part2:?}");
    println!("part 2: {}", part2.iter().sum::<u32>());

}

fn line_parse(line: &str) -> (HashSet<char>, HashSet<char>) {
    let halfway = line.len() / 2;
    let a = HashSet::from_iter(line.chars().take(halfway));
    let b= HashSet::from_iter(line.chars().skip(halfway));
    (a, b)
}

fn overlap_score(a: HashSet<char>, b: HashSet<char>) -> u32 {
    let intersection = a.intersection(&b);
    intersection.fold(0, |acc, c| {
        acc + score(*c)
    })    
}

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

// "vJrwpWtwJgWrhcsFMMfFFhFp"
// "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
// "PmmdzqPrVvPwwTWBwg"
// "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
// "ttgJtRGJQctTZtZT"
// "CrZsJsPPZsGzwwsLwLmpwMDw"


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ascore() {
        let result = score('a');
        assert_eq!(result, 1);
    }
    #[test]
    fn A_ascore() {
        let result = score('A');
        assert_eq!(result, 27);
    }
    #[test]
    fn z_ascore() {
        let result = score('z');
        assert_eq!(result, 26);
    }
    #[test]
    fn Z_ascore() {
        let result = score('Z');
        assert_eq!(result, 52);
    }
}
