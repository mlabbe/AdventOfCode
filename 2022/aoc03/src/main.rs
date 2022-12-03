const MAX_PRIORITY: usize = 53;

type PriorityCount = [u8; MAX_PRIORITY];

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

fn priority_for_char(ch: char) -> u8 {    
    match ch {
        'a'..='z' => {ch as u8 - 'a' as u8 + 1},
        'A'..='Z' => {ch as u8 - 'A' as u8 + 27},
        _ => panic!("invalid char"),
    }
}

fn priority_count_for_compartment(compartment: &str) -> PriorityCount {
    let mut priority_count: PriorityCount = [0; MAX_PRIORITY];
    for ch in compartment.chars()    {
        let priority = priority_for_char(ch) as usize;
        priority_count[priority] += 1;
    }

    priority_count
}

fn find_first_similar_item_in_both_compartments(pc0: &PriorityCount, pc1: &PriorityCount) -> u8 {
    for i in 1..MAX_PRIORITY {
        if pc0[i] > 0 && pc1[i] > 0 {
            return i as u8;
        }
    }

    panic!("no similar item found in both compartments");
}

fn part1(input: &str) -> i32 {
    let mut sum = 0i32;

    for line in input.lines() {
        let line = line.trim();

        assert!(line.len() % 2 == 0);
        let halflen = line.len() / 2;
        
        let compartment0 = &line[0..halflen];
        let compartment1 = &line[halflen..];

        let priority_count_0 = priority_count_for_compartment(compartment0);    
        let priority_count_1 = priority_count_for_compartment(compartment1);

        let priority_ord = find_first_similar_item_in_both_compartments(&priority_count_0,
            &priority_count_1);
        sum += priority_ord as i32;
    }

    sum
}

fn main() {
    println!("part 1: {}", part1(&INPUT));
}


#[cfg(test)]
mod tests {
    use super::part1;

    
    static EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), 157);
    }

    /*/
    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), 12);
    }
    */
}
