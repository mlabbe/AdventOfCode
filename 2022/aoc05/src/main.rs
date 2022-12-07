use std::fmt;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

// last element is highest on the stack
type CrateStacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Command {
    quantity: usize,
    src: usize,
    dst: usize,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.quantity, self.src, self.dst
        )
    }
}

fn find_first_command_line(input: &str) -> usize {
    for (i, line) in input.lines().enumerate() {
        if line.split_whitespace().count() == 0 {
            return i + 1;
        }
    }

    panic!("command lines not found");
}

fn count_columns(input: &str) -> usize {
    // first line containing whitsepace separated numbers is the column count
    for line in input.lines() {
        let v = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        if v.is_empty() {
            continue;
        }

        return *v.last().unwrap();
    }

    panic!("Column count not found");
}

fn parse_crates(input: &str) -> CrateStacks {
    let last_row = find_first_command_line(input) - 2;
    let num_columns = count_columns(input);

    let mut crate_columns: CrateStacks = Vec::with_capacity(num_columns);
    for _ in 0..num_columns {
        crate_columns.push(Vec::new());
    }

    for (i, line) in input.lines().enumerate() {
        if i > last_row {
            break;
        }

        let columns: Vec<&str> = line.split("").collect();
        for index in (2..columns.len()).step_by(4) {
            let crate_id = columns[index].chars().next().unwrap();
            if !crate_id.is_alphabetic() {
                continue;
            }
            let column_index = (index - 2) / 4;
            crate_columns[column_index].insert(0, crate_id);
        }
    }

    crate_columns
}

fn parse_cmdbuf(input: &str) -> Vec<Command> {
    let command_line_row = find_first_command_line(input);
    let mut cmdbuf = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i < command_line_row {
            continue;
        }

        let cmd_vec = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        assert!(cmd_vec.len() == 3);
        assert!(cmd_vec[1] >= 1);
        assert!(cmd_vec[2] >= 1);

        let cmd = Command {
            quantity: cmd_vec[0],
            src: cmd_vec[1],
            dst: cmd_vec[2],
        };

        assert!(cmd.src != cmd.dst);
        cmdbuf.push(cmd);
    }

    cmdbuf
}

fn part1(input: &str) -> String {
    let mut crates = parse_crates(input);
    let cmdbuf = parse_cmdbuf(input);

    for cmd in cmdbuf {
        //println!("{}", cmd);
        for _ in 0..cmd.quantity {
            let id = crates[cmd.src - 1].pop().unwrap();
            crates[cmd.dst - 1].push(id);
        }
    }

    crates.iter().map(|i| i.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let mut crates = parse_crates(input);
    let cmdbuf = parse_cmdbuf(input);

    for cmd in cmdbuf {
        let mut id_set = Vec::<char>::with_capacity(cmd.quantity);

        for _ in 0..cmd.quantity {
            let id = crates[cmd.src - 1].pop().unwrap();
            id_set.push(id);
        }

        id_set.reverse();
        crates[cmd.dst - 1].extend(id_set);
    }

    crates.iter().map(|i| i.last().unwrap()).collect()
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
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), "MCD");
    }
}
