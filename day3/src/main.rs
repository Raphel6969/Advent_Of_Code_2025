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
    let mut answer: u32 = 0;
    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut cbest: u32 = 0;
        for i in 0..len {
            for j in i + 1..len {
                let d1 = chars[i].to_digit(10).expect("err");
                let d2 = chars[j].to_digit(10).expect("err");
                let value = 10 * d1 + d2;
                if value > cbest {
                    cbest = value;
                }
            }
        }
        answer += cbest;
    }
    return answer;
}

fn solve_p2(input: &str) -> u64 {
    let mut answer: u64 = 0;
    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let chars: Vec<char> = line.chars().collect();
        let n = chars.len();
        let k = 12;
        let mut chosen = String::new();
        let mut remaining = k;
        let mut start = 0;
        for _ in 0..k {
            let end = n - remaining;
            let mut best_index = start;
            for i in start..=end {
                if chars[best_index] < chars[i] {
                    best_index = i;
                }
            }
            chosen.push(chars[best_index]);
            start = best_index + 1;
            remaining -= 1;
        }
        let value: u64 = chosen.parse().expect("parse failed");
        answer += value;
    }

    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "\
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    ";

    #[test]
    fn part1_example() {
        assert_eq!(solve_p1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_p2(EXAMPLE_INPUT), 3_121_910_778_619);
    }
}

// fn solve_p1(input: &str) -> u32 {
//     let mut answer: u32 = 0;
//     for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
//         let chars: Vec<char> = line.chars().collect();
//         let len = chars.len();
//         let mut max_index = 0;
//         for i in 0..len - 1 {
//             if chars[max_index] < chars[i] {
//                 max_index = i;
//             }
//         }
//         let mut s_max_index = max_index + 1;
//         for i in max_index + 1..len {
//             if chars[s_max_index] < chars[i] {
//                 s_max_index = i;
//             }
//         }
//         let ans_str: String = [chars[max_index], chars[s_max_index]].iter().collect();
//         let value: u32 = ans_str.parse().expect("Oops Got wrong string");

//         answer += value;
//     }
//     return answer;
// }
