static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

fn solve_for_length(input: &str, msg_length: usize) -> i32 {
    assert!(input.len() > msg_length);

    'outer: for i in msg_length..input.len() {
        // yeah, I'm setting up the table so that the branch tests against 0 in order to ... 
        // save an instruction byte over testing against a non-zero value
        // I get my thrills in weird ways.
        let mut table: [u8; 127] = [2; 127];
        let word = input[i - msg_length..i].as_bytes();

        for ord in word.iter() {
            let ord = *ord as usize;

            table[ord] -= 1;
            if table[ord] == 0 {
                // duplicate found in set
                continue 'outer;
            }
        }
        return i as i32;
    }
    panic!("no solution found");
}

fn part1(input: &str) -> i32 {
    solve_for_length(input, 4)
}


fn part2(input: &str) -> i32 {
    solve_for_length(input, 14)
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_TESTS: [(&str, i32, i32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn test_part_1() {
        for test in EXAMPLE_TESTS {
            assert_eq!(part1(test.0), test.1);
        }
    }

    #[test]
    fn test_part_2() {
        for test in EXAMPLE_TESTS {
            assert_eq!(part2(test.0), test.2);
        }
    }
}
