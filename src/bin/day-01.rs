use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::{env, io};

fn main() {
    let lines = load_input("day-01.txt");

    let values: Vec<u32> = lines
        .iter()
        .map(|line| get_calibration_value(line))
        .collect();

    let sum: u32 = values.iter().sum();

    println!("part1 result: {sum}");

    let values2: Vec<u32> = lines
        .iter()
        .map(|line| get_calibration_value2(line))
        .collect();

    let sum2: u32 = values2.iter().sum();

    println!("part2 result: {sum2}");
}

fn load_input(name: &str) -> Vec<String> {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let path = exe_path
        .parent()
        .expect("Failed to get parent directory")
        .join("../../data/")
        .join(name);

    // println!("{:?}", path.display());

    let file = File::open(path).expect(format!("Reading file '{}' failed", name).as_str());
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.expect("failed to read line"))
        .collect();

    lines
}

fn get_calibration_value(s: &str) -> u32 {
    let first_numeric = s.chars().find(|c| c.is_numeric()).unwrap();
    let last_numeric = s.chars().rev().find(|c| c.is_numeric()).unwrap();
    let a = first_numeric.to_digit(10).unwrap();
    let b = last_numeric.to_digit(10).unwrap();

    10 * a + b
}

fn string_to_u32(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => s.parse::<u32>().ok(),
    }
}

static PATTERN: &str = r"[0-9]|one|t(?:wo|hree)|f(?:our|ive)|s(?:ix|even)|eight|nine";

fn get_calibration_value2(s: &str) -> u32 {
    let matches: Vec<_> = find_overlapping_matches(s, PATTERN);
    let first_match = matches.first().expect("string didnt match");
    let last_match = matches.last().expect("string didnt match");

    let a = string_to_u32(first_match).expect("failed to parse");
    let b = string_to_u32(last_match).expect("failed to parse");

    println!("input: {s:<40} matches: {first_match:>7} / {last_match:<7} | {a}{b}");

    10 * a + b
}

fn find_overlapping_matches<'a>(text: &'a str, pattern: &str) -> Vec<&'a str> {
    let regex = Regex::new(pattern).expect("Invalid regex pattern");
    let mut matches = Vec::new();

    let mut start = 0;
    while let Some(mat) = regex.find_at(text, start) {
        let match_start = mat.start();
        let match_end = mat.end();

        if match_start == match_end {
            // Prevent infinite loop in case of zero-width match
            start += 1;
        } else {
            matches.push(&text[match_start..match_end]);
            start = match_start + 1;
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calibration_values() {
        let pairs = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (s, v) in pairs {
            assert_eq!(
                get_calibration_value(s),
                v,
                "calibration value for string '{}' should be {}",
                s,
                v
            );
        }
    }

    #[test]
    fn test_calibration_values2() {
        let pairs = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("1threeight", 18),
        ];

        for (s, v) in pairs {
            assert_eq!(
                get_calibration_value2(s),
                v,
                "calibration value for string '{}' should be {}",
                s,
                v
            );
        }
    }
}
