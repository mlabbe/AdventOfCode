use std::io;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

fn read_calorie_count_into_sorted_vec(buf: &str) -> Vec<i32> {
    // this approach disposes of the context that only the top 3 are
    // needed.  It is general purpose rather than efficient.

    let mut calorie_count = Vec::new();
    let mut current_count = 0i32;

    for line in buf.lines() {
        let line = line.trim();

        if line.is_empty() {
            calorie_count.push(current_count);
            current_count = 0;
            continue;
        }

        current_count += line.parse::<i32>().unwrap();
    }

    calorie_count.sort();

    calorie_count
}

fn part1(buf: &str) -> i32 {
    let calorie_count = read_calorie_count_into_sorted_vec(buf);
    *calorie_count.last().unwrap()
}

fn part2(buf: &str) -> i32 {
    let calorie_count = read_calorie_count_into_sorted_vec(buf);
    calorie_count[calorie_count.len() - 3..].iter().sum()
}

fn main() -> io::Result<()> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static EXAMPLE: &str = r#"
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000

"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), 45000);
    }
}
