static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

type HeightGrid = Vec<Vec<i8>>;
type VisGrid = HeightGrid;
type ScoreGrid = Vec<Vec<i32>>;

const LEFT: i8 = 0x01;
const RIGHT: i8 = 0x02;
const TOP: i8 = 0x04;
const BOTTOM: i8 = 0x08;

fn new_from_heightgrid(height_grid: &HeightGrid) -> VisGrid {
    vec![vec![0; height_grid.len()]; height_grid[0].len()]
}

fn parse(input: &str) -> HeightGrid {
    let mut height_grid = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        let height_line: Vec<i8> = line
            .chars()
            .map(|c| c.to_string().parse::<i8>().unwrap())
            .collect();

        height_grid.push(height_line);
    }

    height_grid
}

#[allow(dead_code)]
fn assert_grid(
    height_grid: &HeightGrid,
    vis_grid: &VisGrid,
    x: usize,
    y: usize,
    num: i8,
    vis_flags: i8,
) {
    println!(
        "{x}x{y} == num {} testing, hoping for {:b}b",
        vis_grid[y][x], vis_flags
    );

    assert!(height_grid[y][x] == num);
    assert!(vis_grid[y][x] == vis_flags);

    println!("{x}x{y} == {:b}b passed\n", vis_flags);
}

fn count_visible_trees(height_grid: &HeightGrid) -> VisGrid {
    let mut vis_grid = new_from_heightgrid(height_grid);

    let height = height_grid.len();
    let width = height_grid[0].len();

    // left to right
    for y in 0..height {
        let mut highest_tree = -1i8;
        for x in 0..width {
            let tree = height_grid[y][x];
            if tree > highest_tree {
                highest_tree = tree;
                vis_grid[y][x] |= LEFT;
            }
        }
    }

    // right to left
    for y in 0..height {
        let mut highest_tree = -1i8;
        for x in (0..width).rev() {
            let tree = height_grid[y][x];
            if tree > highest_tree {
                highest_tree = tree;
                vis_grid[y][x] |= RIGHT;
            }
        }
    }

    // top to bottom
    for x in 0..width {
        let mut highest_tree = -1i8;
        for y in 0..height {
            let tree = height_grid[y][x];
            if tree > highest_tree {
                highest_tree = tree;
                vis_grid[y][x] |= TOP;
            }
        }
    }

    // bottom to top
    for x in 0..width {
        let mut highest_tree = -1i8;
        for y in (0..height).rev() {
            let tree = height_grid[y][x];
            if tree > highest_tree {
                highest_tree = tree;
                vis_grid[y][x] |= BOTTOM;
            }
        }
    }

    /*
        if true {
            assert_grid(height_grid, &vis_grid, 1, 1, 5, LEFT | TOP); // top-left 5
            assert_grid(height_grid, &vis_grid, 2, 1, 5, TOP | RIGHT);
            assert_grid(height_grid, &vis_grid, 3, 1, 1, 0); // top-right 1
            assert_grid(height_grid, &vis_grid, 1, 2, 5, RIGHT); // left-middle 5
            assert_grid(height_grid, &vis_grid, 2, 2, 3, 0); // center 3
            assert_grid(height_grid, &vis_grid, 3, 2, 3, RIGHT); // right-middle 3
    }
        */
    vis_grid
}

fn score(start: (usize, usize), height_grid: &HeightGrid) -> i32 {
    let height = height_grid.len();
    let width = height_grid[0].len();

    let mut score_right = 0;
    let mut score_left = 0;
    let mut score_up = 0;
    let mut score_down = 0;

    let original_tree = height_grid[start.1][start.0];
    for x in start.0 + 1..width {
        let tree = height_grid[start.1][x];
        score_right += 1;
        if tree >= original_tree {
            break;
        }
    }

    // origin to left
    if start.0 > 0 {
        for x in (0..start.0).rev() {
            let tree = height_grid[start.1][x];
            score_left += 1;
            if tree >= original_tree {
                break;
            }
        }
    }

    // origin down
    for y in start.1 + 1..height {
        let tree = height_grid[y][start.0];
        score_down += 1;
        if tree >= original_tree {
            break;
        }
    }

    // origin up
    if start.1 > 0 {
        for y in (0..start.1).rev() {
            let tree = height_grid[y][start.0];
            score_up += 1;
            if tree >= original_tree {
                break;
            }
        }
    }

    score_right * score_left * score_up * score_down
}

fn score_each_position(height_grid: &HeightGrid) -> ScoreGrid {
    let mut score_grid: ScoreGrid = vec![vec![0; height_grid.len()]; height_grid[0].len()];

    let height = height_grid.len();
    let width = height_grid[0].len();

    for x in 0..width {
        for y in 0..height {
            score_grid[y][x] = score((x, y), height_grid);
        }
    }

    score_grid
}

fn part1(input: &str) -> usize {
    let height_grid = parse(input);
    let vis_grid = count_visible_trees(&height_grid);

    vis_grid.iter().flatten().filter(|&&n| n != 0).count()
}

fn part2(input: &str) -> i32 {
    let height_grid = parse(input);
    let grid_score = score_each_position(&height_grid);

    *grid_score.iter().flatten().max().unwrap()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE), 8);
    }
}
