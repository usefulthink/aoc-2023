use lazy_regex::regex_captures;
use std::cmp::max;

type Draw = (i32, i32, i32);
type Game = (i32, Vec<Draw>);

static INPUT: &str = include_str!("../../data/day-02.txt");

fn main() {
    let res = part1(INPUT);
    println!("Part 1 result: {}", res);

    let res = part2(INPUT);
    println!("Part 2 result: {}", res);
}

fn part1(input: &str) -> i32 {
    let games = parse_input(input);
    let max = (12, 13, 14);

    let filtered = games.iter().filter(|game| is_game_possible(&game, &max));

    filtered.map(|game| game.0).sum()
}

fn part2(input: &str) -> i32 {
    let games = parse_input(input);

    games
        .iter()
        .map(|game| get_min_cubes(&game))
        .map(|min| get_power(&min))
        .sum()
}

fn get_power(min_cubes: &Draw) -> i32 {
    min_cubes.0 * min_cubes.1 * min_cubes.2
}

fn get_min_cubes(game: &Game) -> Draw {
    let (_, draws) = game;
    draws.iter().fold((0, 0, 0), |acc, draw| {
        (max(acc.0, draw.0), max(acc.1, draw.1), max(acc.2, draw.2))
    })
}

fn is_game_possible(game: &Game, max: &Draw) -> bool {
    let (_, draws) = game;

    draws.iter().all(|draw| is_draw_possible(draw, max))
}

fn is_draw_possible(draw: &Draw, max: &Draw) -> bool {
    draw.0 <= max.0 && draw.1 <= max.1 && draw.2 <= max.2
}

fn parse_input(input: &str) -> Vec<(i32, Vec<Draw>)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> (i32, Vec<Draw>) {
    let (_, game_id_str, draws_str) =
        regex_captures!(r#"^Game\s*(\d+):\s*(.*)$"#, line).expect("failed to parse line");

    let game_id = game_id_str.parse::<i32>().expect("failed to parse game_id");
    let draws = draws_str
        .split(";")
        .map(|draw| parse_draw(draw.trim()))
        .collect();

    (game_id, draws)
}

fn parse_draw(draw: &str) -> Draw {
    let parts = draw.split(",");

    let mut parsed = (0, 0, 0);

    for p in parts {
        let (_, count_str, color) = regex_captures!(r#"(\d+) (red|green|blue)"#, p,).unwrap();
        let count = count_str.parse::<i32>().expect("failed to parse");

        match color {
            "red" => parsed.0 = count,
            "green" => parsed.1 = count,
            "blue" => parsed.2 = count,
            _ => panic!("invalid color value"),
        }
    }

    parsed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_draw() {
        let tests = [
            ("3 blue, 4 red", (4, 0, 3)),
            ("1 red, 2 green, 6 blue", (1, 2, 6)),
            ("2 green", (0, 2, 0)),
        ];

        for (s, expected) in tests {
            let res = parse_draw(s);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_parse_line() {
        let tests = [
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                (2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                (3, vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                (4, vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                (5, vec![(6, 3, 1), (1, 2, 2)]),
            ),
        ];

        for (s, expected) in tests {
            let res: (i32, Vec<Draw>) = parse_line(s);

            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_parse_input() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n\
        ";

        let expected = vec![
            (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
            (2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
            (3, vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
            (4, vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
            (5, vec![(6, 3, 1), (1, 2, 2)]),
        ];

        let res = parse_input(input);

        assert_eq!(res, expected);
    }

    #[test]
    fn test_is_draw_possible() {
        let tests = [
            ((1, 2, 3), (10, 10, 10), true),
            ((10, 1, 10), (10, 10, 10), true),
            ((1, 2, 3), (1, 1, 1), false),
        ];

        for (draw, max, expected) in tests {
            assert_eq!(is_draw_possible(&draw, &max), expected);
        }
    }

    #[test]
    fn test_is_game_possible() {
        let tests = [
            ((1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]), (4, 2, 6), true),
            ((1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]), (4, 2, 5), false),
        ];
        for (game, max, expected) in tests {
            assert_eq!(is_game_possible(&game, &max), expected);
        }
    }

    #[test]
    fn test_get_min_cubes() {
        let tests = [
            ((1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]), (4, 2, 6)),
            ((2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]), (1, 3, 4)),
        ];

        for (game, expected) in tests {
            assert_eq!(get_min_cubes(&game), expected);
        }
    }
}
