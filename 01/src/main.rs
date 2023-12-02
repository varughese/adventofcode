use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn parse_number(n: &str) -> i64 {
    let p = n.parse::<i64>();

    match p {
        Ok(n) => n,
        Err(_) => match n {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        },
    }
}

fn solve(lines: Vec<&str>) -> i64 {
    let re =
        Regex::new("(one|two|three|four|five|six|seven|eight|nine|ten|1|2|3|4|5|6|7|8|9)").unwrap();

    lines
        .iter()
        .map(|l| {
            let c = re
                .captures_iter(l)
                .map(|cap| parse_number(cap.extract::<1>().0))
                .collect::<Vec<_>>();

            format!("{}{}", c.first().unwrap(), c.last().unwrap())
                .parse::<i64>()
                .unwrap()
        })
        .sum::<i64>()
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("/Users/matv/Code/learning/adventofcode/src/1.txt")?;
    let reader = io::BufReader::new(file);
    let mut grand_total = 0;
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // Iterate through lines
    for lineee in reader.lines() {
        // Unwrap the line or handle any potential errors
        let linee = lineee?;
        let line = re.replace_all(&linee, |caps: &regex::Captures| {
            match &caps[0] {
                "one" => "1one",
                "two" => "2two",
                "three" => "3three",
                "four" => "4four",
                "five" => "5five",
                "six" => "6six",
                "seven" => "7seven",
                "eight" => "8eight",
                "nine" => "9nine",
                _ => &caps[0], // If not found in the mapping, keep the original word
            }.to_string()
        });

        let digits: String = line.chars()
            .filter(|c| c.is_digit(10))
            .collect();
        
        let first = digits.chars().next().expect("oops").to_digit(10).expect("not a digit");
        let last = digits.chars().last().expect("oops").to_digit(10).expect("not a digit");

        let total = 10 * first + last;

        println!("{} {}", linee, total);

        grand_total += total;
    }

    let str = include_str!("../src/1.txt");

    println!("{} {}",grand_total, solve(str.lines().collect::<Vec<_>>()));

    Ok(())
}