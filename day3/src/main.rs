use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let answer = solve_p1(&input);
    println!("{}", answer);
    Ok(())
}

fn solve_p1(input: &str) -> u32 {
    let mut answer: u32 = 0;
    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut max_index = 0;
        for i in 0..len - 1 {
            if chars[max_index] < chars[i] {
                max_index = i;
            }
        }
        let mut s_max_index = max_index + 1;
        for i in max_index + 1..len {
            if chars[s_max_index] < chars[i] {
                s_max_index = i;
            }
        }
        let ans_str: String = [chars[max_index], chars[s_max_index]].iter().collect();
        let value: u32 = ans_str.parse().expect("Oops Got wrong string");

        answer += value;
    }
    return answer;
}
