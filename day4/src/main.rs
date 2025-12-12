use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let answer = solve_p2(&input);
    println!("{}", answer);
    Ok(())
}

#[warn(dead_code)]
fn solve_p1(input: &str) -> u32 {
    let lines: Vec<&str> = input.trim().lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut accessible_count: u32 = 0;

    for r in 0..rows{
        for c in 0..cols{
            if grid[r][c] != '@' {
                continue;
            }

            let mut neighbors = 0;
            for (dr, dc) in directions.iter().copied() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < rows as isize && nc>=0 && nc<cols as isize{
                    if grid[nr as usize][nc as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                accessible_count += 1;
            }
        }

    }
    return accessible_count;
}
fn solve_p2(input: &str) -> u32 {
    let lines: Vec<&str> = input.trim().lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut answer: u32 = 0;

    for _ in 0..rows*cols{
        let mut accessible_count: u32 = 0;

        for r in 0..rows{
            for c in 0..cols{
                if grid[r][c] != '@' {
                    continue;
                }

                let mut neighbors = 0;
                for (dr, dc) in directions.iter().copied() {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < rows as isize && nc>=0 && nc<cols as isize{
                        if grid[nr as usize][nc as usize] == '@' {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors < 4 {
                    accessible_count += 1;
                    grid[r][c] = 'x';
                }
            }

        }
        answer += accessible_count;
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn solve_p1_test() {assert_eq!(solve_p1(EXAMPLE_INPUT), 13)}

    #[test]
    fn solve_p2_test() {assert_eq!(solve_p2(EXAMPLE_INPUT), 43)}
}