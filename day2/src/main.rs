use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let answer = solve_p1(&input);
    println!("{}", answer);
    Ok(())
}

fn solve_p1(input: &str) -> u64 {
    let mut answer: u64 = 0;
    for line in input.split(",").map(str::trim).filter(|l| !l.is_empty()) {
        let range: Vec<&str> = line.split("-").collect();
        let start: u64 = range[0].trim().parse().expect("Invalid Start Range");
        let end: u64 = range[1].trim().parse().expect("Invalid End Range");

        for i in start..=end {
            let num_str: String = i.to_string();
            let len = num_str.len();
            if len % 2 != 0 {
                continue;
            }
            let (p1, p2) = num_str.split_at(len / 2);
            if p1 == p2 {
                answer += i;
            }
        }
    }

    return answer;
}
