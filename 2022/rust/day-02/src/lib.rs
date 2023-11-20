use std::{cmp::Ordering, str::FromStr};

#[derive(Clone, Copy, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("Invalid move: {}", s)),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();

            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                None => panic!("Invalid move"),
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();

            let opponent_move = moves[0].parse::<Move>().unwrap();
            match moves[1] {
                "X" => match opponent_move {
                    Move::Rock => Move::Scissors as u32,
                    Move::Paper => Move::Rock as u32,
                    Move::Scissors => Move::Paper as u32,
                },
                "Y" => 3 + opponent_move as u32,
                "Z" => {
                    6 + match opponent_move {
                        Move::Rock => Move::Paper as u32,
                        Move::Paper => Move::Scissors as u32,
                        Move::Scissors => Move::Rock as u32,
                    }
                }
                _ => panic!("Invalid move"),
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
