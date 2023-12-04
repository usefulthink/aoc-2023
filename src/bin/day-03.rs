use lazy_regex::regex;
use regex::Match;
use std::cmp::{max, min};

static INPUT: &str = include_str!("../../data/day-03.txt");

static TEST_INPUT: &str = "\
    467..114..\n\
    ...*......\n\
    ..35..633.\n\
    ......#...\n\
    617*......\n\
    .....+.58.\n\
    ..592.....\n\
    ......755.\n\
    ...$.*....\n\
    .664.598..\
";

#[derive(Debug, Clone)]
struct PartNumber {
    postion: (usize, usize),
    len: usize,
    value: i32,
}

#[derive(Debug, Clone)]
struct ParsedInput<'a> {
    lines: Vec<&'a str>,
    part_numbers: Vec<PartNumber>,
}

fn main() {
    let res = part1(INPUT);
    println!("Part 1 result: {}", res);

    let res = part2(INPUT);
    println!("Part 2 result: {}", res);
}

fn part1(input: &str) -> i32 {
    let parsed_input = parse_input(input);

    let part_numbers = parsed_input
        .part_numbers
        .iter()
        .filter(|part_number| has_adjacent_symbol(&part_number, &parsed_input.lines));

    part_numbers.map(|pn| pn.value).sum()
}

fn part2(input: &str) -> i32 {
    let rx_gear = regex!(r"\*");

    let stuff = parse_input(input);
    let lines: Vec<&str> = stuff.lines;
    let gear_locations: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| rx_gear.find_iter(line).map(move |m| (m.start(), y)))
        .collect();

    let filtered = gear_locations.iter().filter(|pos| {
        let selected = select_bbox(&lines, get_bbox(&lines, pos, 1));

        let num_matches = selected
            .iter()
            .fold(0, |count, s| count + regex!(r"\d+").find_iter(s).count());

        num_matches == 2
    });

    let mut sum = 0;
    for pos in filtered {
        println!("gear at {pos:?}");
        sum += find_gear_ratio(&lines, *pos);
    }

    sum
}

fn find_gear_ratio(lines: &[&str], gear_pos: (usize, usize)) -> i32 {
    let (x, y) = gear_pos;
    let w = lines[0].len();
    let h = lines.len();

    let x0 = x.saturating_sub(3);
    let y0 = y.saturating_sub(1);
    let y1 = (y + 2).min(h);
    let x1 = (x + 4).min(w);

    let selection = select_bbox(&lines, ((x0, y0), (x1, y1)));

    println!("selection: {:#?}", selection);

    let rx_number = regex!(r"\d+");
    let mut res = 1;
    for line in selection {
        for m in rx_number.find_iter(line) {
            // 012*456
            // 467..11
            if m.start() > 4 || m.end() < 3 {
                continue;
            }

            println!("matched number {:#?}", m);
            res *= m.as_str().parse::<i32>().expect("failed to parse");
        }
    }

    res
}

fn has_adjacent_symbol(part_number: &PartNumber, lines: &Vec<&str>) -> bool {
    let bbox = get_bbox(&lines, &part_number.postion, part_number.len);
    let selected = select_bbox(lines, bbox);

    let rx_symbol = regex!(r"[^0-9\.]");

    selected.iter().any(|s| rx_symbol.is_match(s))
}

fn get_bbox(lines: &[&str], pos: &(usize, usize), len: usize) -> ((usize, usize), (usize, usize)) {
    let (x, y) = pos;
    let w = lines[0].len();
    let h = lines.len();

    let x0 = x.saturating_sub(1);
    let y0 = y.saturating_sub(1);
    let y1 = (y + 2).min(h);
    let x1 = (x + len + 1).min(w);

    ((x0, y0), (x1, y1))
}

fn select_bbox<'a>(lines: &'a [&str], bbox: ((usize, usize), (usize, usize))) -> Vec<&'a str> {
    let ((x0, y0), (x1, y1)) = bbox;

    let selected_lines = lines[y0..y1].to_vec();

    selected_lines.iter().map(|line| &line[x0..x1]).collect()
}

fn parse_input<'a>(input: &'a str) -> ParsedInput<'a> {
    let lines: Vec<&str> = input.lines().map(|s| s.trim()).collect();

    let part_numbers: Vec<PartNumber> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| get_part_numbers_from_line(y, line))
        .collect();

    ParsedInput {
        lines,
        part_numbers,
    }
}

fn get_part_numbers_from_line(y: usize, line: &str) -> Vec<PartNumber> {
    let rx = regex!(r"\d+");

    rx.find_iter(line)
        .map(|digits| PartNumber {
            postion: (digits.start(), y),
            len: digits.len(),
            value: digits.as_str().parse::<i32>().unwrap(),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\
        ";

        let res = parse_input(input);
        println!("parse_input result: {:#?}", res);
    }

    #[test]
    fn test_has_adjacent_symbol() {
        #[rustfmt::skip]
        let lines = vec![
            "467..114..", 
            "...*......", 
            "..35..633.", 
            "......#..."
        ];

        let part_number = PartNumber {
            postion: (2, 2),
            len: 2,
            value: 35,
        };

        assert!(has_adjacent_symbol(&part_number, &lines));

        let part_number = PartNumber {
            postion: (5, 0),
            len: 3,
            value: 114,
        };

        assert!(!has_adjacent_symbol(&part_number, &lines));
    }
}
