use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let answer = solve_p2(&input);
    println!("{}", answer);
    Ok(())
}

fn solve_p1(input: &str) -> u32 {
    let mut pos: i32 = 50;
    let mut zeroes: u32 = 0;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (dir, dist_str) = line.split_at(1);
        let dist: i32 = dist_str.parse().expect("Invalid Distance");

        match dir {
            "L" => {
                let d = dist % 100;
                pos = (pos - d).rem_euclid(100);
            }
            "R" => {
                let d = dist % 100;
                pos = (pos + d).rem_euclid(100);
            }
            _ => {
                panic!("Invalid direction : {}", dir);
            }
        }

        if pos == 0 {
            zeroes += 1;
        }
    }

    return zeroes;
}

fn solve_p2(input: &str) -> u32 {
    let mut pos: i32 = 50;
    let mut zeroes: u32 = 0;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (dir, dist_str) = line.split_at(1);
        let dist: i32 = dist_str.parse().expect("Invalid Distance");

        match dir {
            "L" => {
                let start = (100 - pos) % 100;
                zeroes += ((start + dist) / 100) as u32;
                let d = dist % 100;
                pos = (pos - d).rem_euclid(100);
            }
            "R" => {
                zeroes += ((pos + dist) / 100) as u32;
                let d = dist % 100;
                pos = (pos + d).rem_euclid(100);
            }
            _ => {
                panic!("Invalid direction : {}", dir);
            }
        }
    }

    return zeroes;
}
