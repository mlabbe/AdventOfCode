static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

type CrateColumns = Vec<char>;

fn find_first_command_line(input: &str) -> usize {
    for (i, line) in input.lines().enumerate() {
        if line.split_whitespace().count() == 0 {
            return i+1;
        }
    }

    panic!("command lines not found");
}

fn count_columns(input: &str) -> usize {
    // first line containing whitsepace separated numbers is the column count
    for line in input.lines() {
        let v = line.split_whitespace().
            filter_map(|s| { s.parse::<usize>().ok() }).
            collect::<Vec<usize>>();
        
        if v.is_empty() {
            continue;
        }

        return *v.last().unwrap();
    }
    
    panic!("Column count not found");
}

fn parse_crates(input: &str) -> usize {

    
    let last_row  = find_first_command_line(input) - 3; 
    let num_columns = count_columns(input);

    let crate_columns: CrateColumns = Vec::with_capacity(num_columns);

    for (i, line) in input.lines().enumerate() {
        if i > last_row {
            break;
        }

        let columns: Vec<&str> = line.split("").collect();
        //assert!(columns.len() % 4 == 0);
        for index in (2..columns.len()).step_by(4) {
            let crate_id = columns[index];
            if !crate_id.chars().next().unwrap().is_alphabetic() {
                continue;
            }
            let crate_index = (index - 2) / 4;
            // todo: CrateColumns needs 2d, then return it
            //println!("{}: {}", crate_id, crate_index);
        }
        //println!("{:?}", columns);
    }

    0
}

fn part1(input: &str) -> String {
    parse_crates(input);

    "NO".to_string()
}

fn part2(input: &str) -> String {
    "NO".to_string()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}


#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_common() {
        assert_eq!(count_columns(EXAMPLE), 3);
        assert_eq!(count_columns(INPUT), 9);
        assert_eq!(find_first_command_line(EXAMPLE), 5);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), "CMZ");
    }

    //#[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), "UNKNOWN");
    }

}
