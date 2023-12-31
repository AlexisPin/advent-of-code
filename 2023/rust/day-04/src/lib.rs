use std::collections::{BTreeMap, HashMap};

pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let mut it = line.split(" | ");
            let winning_numbers = it
                .next()
                .unwrap()
                .split(": ")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<&str>>();

            let numbers = it.next().unwrap().split_ascii_whitespace();

            let mut total = 0;
            numbers.for_each(|n| {
                if winning_numbers.contains(&n) {
                    if total == 0 {
                        total += 1;
                    } else {
                        total *= 2;
                    }
                }
            });
            total
        })
        .sum::<u32>();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut map = BTreeMap::new();
    let mut instances: HashMap<String, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let mut it = line.split(" | ");
        let winning_numbers = it
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();

        let numbers = it.next().unwrap().split_ascii_whitespace();

        let mut total = 0;
        numbers.for_each(|n| {
            if winning_numbers.contains(&n) {
                total += 1;
            }
        });
        let card_id = line
            .split(": ")
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();
        instances.insert(card_id.to_string(), 1);
        map.insert(card_id, total);
    });

    dbg!(&map);

    map.iter().for_each(|(card_id, count)| {
        let instance_count = *instances.get(*card_id).unwrap();

        for i in 1..=*count {
            let next = (i + card_id.parse::<u32>().unwrap()).to_string();
            instances.entry(next).and_modify(|e| *e += instance_count);
        }
    });
    instances.values().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "31");
    }
}
