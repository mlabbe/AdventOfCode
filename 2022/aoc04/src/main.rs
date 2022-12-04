static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

#[derive(Debug)]
struct Span {
    min: i8,
    max: i8,
}

impl Span {
    fn contains(&self, rhs: &Span) -> bool {
        rhs.min >= self.min && rhs.max <= self.max
    }

    fn overlaps(&self, rhs: &Span) -> bool {
        self.min <= rhs.max && self.max >= rhs.min
    }
}

fn parse_spans(line: &str) -> (Span, Span) {
    let spans = line
        .split(|c| c == '-' || c == ',');

    let mut parsed_numbers: Vec<i8> = Vec::with_capacity(4);

    for span_str in spans {
        parsed_numbers.push(span_str.parse::<i8>().unwrap());        
    }

    assert!(parsed_numbers.len() == 4);

    
    (Span{
        min: parsed_numbers[0],
        max: parsed_numbers[1],
    },
    Span{
        min: parsed_numbers[2],
        max: parsed_numbers[3],
    })
}


fn part1(input: &str) -> i32 {
    let mut contain_count = 0i32;

    for line in input.lines() {
        let line = line.trim();

        let (first_span, second_span) = parse_spans(line);
        if first_span.contains(&second_span) || second_span.contains(&first_span) {
            contain_count += 1;
        }
    }

    contain_count
}

fn part2(input: &str) -> i32 {
    let mut overlap_count = 0i32;

    for line in input.lines() {
        let line = line.trim();

        let (first_span, second_span) = parse_spans(line);
        if first_span.overlaps(&second_span) {
            overlap_count += 1;
        }
    }

    overlap_count
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}


#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), 4);
    }

}