use lazy_regex::{regex, regex_captures};
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../data/day-04.txt");
static TEST_INPUT: &str = "\
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
";

#[derive(Debug, PartialEq)]
struct ScratchCard {
    winning_numbers: HashSet<i32>,
    scratched_numbers: HashSet<i32>,
}

fn main() {
    let res = part1(INPUT);
    println!("Part 1 result: {}", res);

    let res = part2(INPUT);
    println!("Part 2 result: {}", res);
}

fn part1(input: &str) -> i32 {
    let cards = parse_input(input);

    cards.iter().fold(0, |acc, card| {
        let n = get_winning_number_count(card);
        let score = if n > 0 { 1 << (n - 1) } else { 0 };

        // println!("card: {:?}, num_winning: {}, score: {}", card, n, score);

        acc + score
    })
}

fn part2(input: &str) -> i32 {
    let cards = parse_input(input);
    let mut card_counts: Vec<usize> = vec![1; cards.len()];

    cards
        .iter()
        .enumerate()
        .fold(0, |total_cards, (idx, card)| -> usize {
            let n = get_winning_number_count(card);
            let count = card_counts[idx];

            // println!("card: {:?}, num_winning: {}", card, n);
            for i in idx + 1..(idx + 1 + n) {
                card_counts[i] = card_counts[i] + count;
                // println!(
                //     "  update count at index {}: count is now {}",
                //     i, card_counts[i]
                // );
            }

            total_cards + count
        }) as i32
}

fn get_winning_number_count(card: &ScratchCard) -> usize {
    card.winning_numbers
        .intersection(&card.scratched_numbers)
        .count()
}

fn parse_input(input: &str) -> Vec<ScratchCard> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> ScratchCard {
    let mut card = ScratchCard {
        winning_numbers: HashSet::new(),
        scratched_numbers: HashSet::new(),
    };

    let (_, winning_numbers_str, scratched_numbers_str) = regex_captures!(
        r"Card\s+\d+:\s*((?:\d+\s+)*\d+)\s*\|\s*((?:\d+\s+)*\d+)",
        line
    )
    .expect("failed to parse line");

    for s in winning_numbers_str.split(" ") {
        let n = s.parse::<i32>();
        if n.is_ok() {
            card.winning_numbers.insert(n.unwrap());
        }
    }

    for s in scratched_numbers_str.split(" ") {
        let n = s.parse::<i32>();
        if n.is_ok() {
            card.scratched_numbers.insert(n.unwrap());
        }
    }

    card
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let data = parse_input(TEST_INPUT);

        assert_eq!(data.len(), 6);
        assert_eq!(data[1].winning_numbers, HashSet::from([13, 32, 20, 16, 61]))
    }

    #[test]
    fn test_parse_line() {
        let tests = [(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            ScratchCard {
                winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
                scratched_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            },
        )];

        for (line, expected) in tests {
            assert_eq!(parse_line(line), expected);
        }
    }
}
