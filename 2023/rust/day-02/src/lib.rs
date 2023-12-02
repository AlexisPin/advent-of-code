#[derive(PartialEq)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    game_id: u32,
    bags: Vec<Bag>,
}

impl Bag {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.red > other.red || self.green > other.green || self.blue > other.blue {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let base_bag = Bag::new(12, 13, 14);

    let games = input.lines().map(|line| {
        let mut it = line.split(": ");
        let game_id = it
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let cubes_sets = it.next().unwrap().split("; ");
        let bags = cubes_sets
            .map(|cubes_set| {
                let cubes = cubes_set.split(", ");
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                cubes.for_each(|cube| {
                    let mut it = cube.split(" ");
                    let count = it.next().unwrap().parse::<u32>().unwrap();
                    let color = it.next().unwrap();
                    match color {
                        "red" => red += count,
                        "green" => green += count,
                        "blue" => blue += count,
                        _ => panic!("Unknown color"),
                    }
                });
                Bag::new(red, green, blue)
            })
            .collect::<Vec<_>>();

        Game { game_id, bags }
    });

    let res = games
        .map(|game| {
            if game.bags.iter().all(|bag| bag <= &base_bag) {
                Some(game.game_id)
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum::<u32>();

    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    let games = input.lines().map(|line| {
        let mut it = line.split(": ");
        let game_id = it
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let cubes_sets = it.next().unwrap().split("; ");
        let bags = cubes_sets
            .map(|cubes_set| {
                let cubes = cubes_set.split(", ");
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                cubes.for_each(|cube| {
                    let mut it = cube.split(" ");
                    let count = it.next().unwrap().parse::<u32>().unwrap();
                    let color = it.next().unwrap();
                    match color {
                        "red" => red += count,
                        "green" => green += count,
                        "blue" => blue += count,
                        _ => panic!("Unknown color"),
                    }
                });
                Bag::new(red, green, blue)
            })
            .collect::<Vec<_>>();

        (game_id, bags)
    });

    let res = games
        .map(|(_, bags)| {
            let mut max_blue = 0;
            let mut max_green = 0;
            let mut max_red = 0;
            bags.iter().for_each(|bag| {
                if bag.blue > max_blue {
                    max_blue = bag.blue;
                }
                if bag.green > max_green {
                    max_green = bag.green;
                }
                if bag.red > max_red {
                    max_red = bag.red;
                }
            });
            max_blue * max_green * max_red
        })
        .sum::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "2286");
    }
}
