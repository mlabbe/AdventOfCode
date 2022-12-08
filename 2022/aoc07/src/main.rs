static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

#[derive(Debug)]
enum TerminalOutput {
    CommandCd(String),
    CommandLs,

    ResponseDir(),
    ResponseSize(usize),
}

fn parse_terminal_output(input: &str) -> Vec<TerminalOutput> {
    input
        .lines()
        .map(|line| {
            if let Some(line) = line.strip_prefix('$') {
                // parse command

                let args: Vec<&str> = line.split_whitespace().collect();
                match args[0] {
                    "ls" => TerminalOutput::CommandLs,
                    "cd" => TerminalOutput::CommandCd(args[1].to_string()),
                    _ => unreachable!("bogus terminal output {}", args[0]),
                }
            } else {
                // parse response

                let args: Vec<&str> = line[0..].split_whitespace().collect();
                match args[0] {
                    "dir" => TerminalOutput::ResponseDir(),
                    _ => TerminalOutput::ResponseSize(args[0].parse().expect("expected size")),
                }
            }
        })
        .collect()
}

fn find_total_dir_size(
    index: &mut usize,
    output: &Vec<TerminalOutput>,
    dir_sizes: &mut Vec<usize>,
) -> usize {
    let mut total = 0usize;
    while *index < output.len() {
        let current_output = &output[*index];
        *index += 1;
        match current_output {
            TerminalOutput::ResponseSize(sz) => total += sz,
            TerminalOutput::CommandCd(new_dir) => {
                if new_dir != ".." {
                    //  println!("entering subdir {}", new_dir);
                    total += find_total_dir_size(index, output, dir_sizes);
                } else {
                    // println!("leaving subdir with total {total}");
                    dir_sizes.push(total);
                    return total;
                }
            }
            _ => (),
        }
    }
    //    println!("Finishing sweep of dir with {total}");
    dir_sizes.push(total);
    total
}

fn part1(input: &str) -> usize {
    let terminal_output = parse_terminal_output(input);
    let mut dir_sizes = vec![];

    find_total_dir_size(&mut 0, &terminal_output, &mut dir_sizes);

    let mut count = 0;
    for sz in dir_sizes {
        if sz <= 100000 {
            count += sz;
        }
    }

    count
}

fn part2(input: &str) -> usize {
    const TOTAL_DISK_SPACE: usize = 70000000;
    const REQUIRED_DISK_SPACE: usize = 30000000;

    let terminal_output = parse_terminal_output(input);
    let mut dir_sizes = vec![];

    let total_used_disk_space = find_total_dir_size(&mut 0, &terminal_output, &mut dir_sizes);
    let at_least = total_used_disk_space - (TOTAL_DISK_SPACE - REQUIRED_DISK_SPACE);

    let mut acceptable_sizes = Vec::new();
    for sz in dir_sizes {
        if sz >= at_least {
            acceptable_sizes.push(sz);
        }
    }

    *acceptable_sizes.iter().min().unwrap()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), 24933642);
    }
}
