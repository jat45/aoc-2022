fn main() {
    let sections = include_str!("input.txt").lines().map(parse_line).collect::<Vec<_>>();
    let results = sections
    .iter()
    .map(|(x,y)| {
        let contains = x.contains(y) || y.contains(x);
        let overlaps = x.overlaps(y);
        (contains, overlaps)
    })
    .collect::<Vec<_>>();


    println!("contains {}", results.iter().filter(|(x,_)| *x).count());
    println!("overlaps {}", results.iter().filter(|(_,y)| *y).count());
}

#[derive(Debug)]
struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Section) -> bool {
        !(other.end < self.start || other.start > self.end)
    }
}

fn parse_line(line: &str) -> (Section, Section) {
    let mut sections = line.split(",").map(parse_section);
    (sections.next().expect("no first section"), sections.next().expect("no second section"))
}

fn parse_section(section: &str) -> Section {
    let mut numbers = section.split("-").map(|s| s.parse::<usize>().expect("wasn't a number"));
    Section { start: numbers.next().expect("no start found"), end: numbers.next().expect("no end found") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Section { start: 1, end: 4}, Section{start:2, end: 3}, true)]
    #[test_case(Section { start: 1, end: 4}, Section{start:1, end: 4}, true)]
    #[test_case(Section { start: 3, end: 4}, Section{start:3, end: 3}, true)]
    #[test_case(Section { start: 3, end: 4}, Section{start:2, end: 3}, false)]
    #[test_case(Section { start: 3, end: 4}, Section{start:4, end: 5}, false)]
    #[test_case(Section { start: 3, end: 4}, Section{start:2, end: 5}, false)]
    fn test_contains(outer: Section, inner: Section, expected: bool) {
        assert_eq!(outer.contains(&inner), expected)
    }

    #[test_case(Section { start: 2, end: 4}, Section{start:6, end: 8}, false)]
    #[test_case(Section { start: 2, end: 3}, Section{start:4, end: 5}, false)]
    #[test_case(Section { start: 5, end: 7}, Section{start:7, end: 9}, true)]
    #[test_case(Section { start: 2, end: 8}, Section{start:3, end: 7}, true)]
    #[test_case(Section { start: 6, end: 6}, Section{start:4, end: 6}, true)]
    fn test_overlaps(x: Section, y: Section, expected: bool) {
        assert_eq!(x.overlaps(&y), expected)
    }
}
