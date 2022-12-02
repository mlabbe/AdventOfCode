use itertools::Itertools;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

#[derive(Debug, Clone, Copy)]
enum PlayKind {
    Rock = 1,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl PlayKind {
    fn from_char(ch: char) -> PlayKind {
        match ch {
            'A' => PlayKind::Rock,
            'B' => PlayKind::Paper,
            'C' => PlayKind::Scissors,

            'X' => PlayKind::Rock,
            'Y' => PlayKind::Paper,
            'Z' => PlayKind::Scissors,

            _ => panic!("invalid opponent char"),
        }
    }

    fn respond_to_opponent(&self, opponent: PlayKind) -> Outcome {
        match opponent {
            PlayKind::Rock => match *self {
                PlayKind::Rock => Outcome::Draw,
                PlayKind::Paper => Outcome::Won,
                PlayKind::Scissors => Outcome::Lost,
            },

            PlayKind::Paper => match *self {
                PlayKind::Rock => Outcome::Lost,
                PlayKind::Paper => Outcome::Draw,
                PlayKind::Scissors => Outcome::Won,
            },

            PlayKind::Scissors => match *self {
                PlayKind::Rock => Outcome::Won,
                PlayKind::Paper => Outcome::Lost,
                PlayKind::Scissors => Outcome::Draw,
            },
        }
    }
}

impl Outcome {
    fn pick_kind_to_match_outcome(&self, opponent: PlayKind) -> PlayKind {
        match opponent {
            PlayKind::Rock => match *self {
                Outcome::Lost => PlayKind::Scissors,
                Outcome::Draw => PlayKind::Rock,
                Outcome::Won => PlayKind::Paper,
            },

            PlayKind::Paper => match *self {
                Outcome::Lost => PlayKind::Rock,
                Outcome::Draw => PlayKind::Paper,
                Outcome::Won => PlayKind::Scissors,
            },

            PlayKind::Scissors => match *self {
                Outcome::Lost => PlayKind::Paper,
                Outcome::Draw => PlayKind::Scissors,
                Outcome::Won => PlayKind::Rock,
            },
        }
    }

    fn from_char(ch: char) -> Outcome {
        match ch {
            'X' => Outcome::Lost,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Won,

            _ => panic!("invalid outcome char"),
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut total_score = 0i32;

    for line in input.lines() {
        let line = line.trim();

        let (opponent, response) = line
            .split(' ')
            .map(|x| PlayKind::from_char(x.chars().next().unwrap()))
            .next_tuple()
            .unwrap();

        let outcome = response.respond_to_opponent(opponent);
        let round_score = outcome as i32 + response as i32;

        total_score += round_score;
    }

    total_score
}

fn part2(input: &str) -> i32 {
    let mut total_score = 0i32;

    for line in input.lines() {
        let line = line.trim();

        let play_strings = line.split(' ').collect::<Vec<&str>>();
        let opponent = PlayKind::from_char(play_strings[0].chars().next().unwrap());
        let outcome = Outcome::from_char(play_strings[1].chars().next().unwrap());

        let response = outcome.pick_kind_to_match_outcome(opponent);

        let round_score = outcome as i32 + response as i32;
        total_score += round_score;
    }

    total_score
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static EXAMPLE: &str = r#"A Y
B X
C Z
"#;
    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), 12);
    }
}
