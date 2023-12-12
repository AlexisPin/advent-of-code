use std::collections::BTreeMap;

enum Instruction {
    Right,
    Left,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

pub fn process_part1(input: &str) -> String {
    let mut it = input.split("\n\n");

    let instructions = it
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<_>>();

    let nodes = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nodes = line.split(" = ");
            let name = nodes.next().unwrap();
            let mut values = nodes.next().unwrap().split(", ");

            let left = values
                .next()
                .unwrap()
                .chars()
                .skip(1)
                .take(3)
                .collect::<String>();
            let right = values.next().unwrap().chars().take(3).collect::<String>();
            let node = Node::new(left, right);

            (name, node)
        })
        .collect::<BTreeMap<_, _>>();

    let mut starting_node = nodes.get("AAA").unwrap();
    let mut nb_instructions: u64 = 0;

    for instruction in instructions.iter().cycle() {
        nb_instructions += 1;
        let next_value;
        match instruction {
            Instruction::Left => {
                next_value = starting_node.left.as_str();
            }
            Instruction::Right => {
                next_value = starting_node.right.as_str();
            }
        }
        if next_value == "ZZZ" {
            break;
        }
        starting_node = nodes.get(next_value).unwrap();
    }

    nb_instructions.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut it = input.split("\n\n");

    let instructions = it
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<_>>();

    let nodes = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nodes = line.split(" = ");
            let name = nodes.next().unwrap();
            let mut values = nodes.next().unwrap().split(", ");

            let left = values
                .next()
                .unwrap()
                .chars()
                .skip(1)
                .take(3)
                .collect::<String>();
            let right = values.next().unwrap().chars().take(3).collect::<String>();
            let node = Node::new(left, right);

            (name, node)
        })
        .collect::<BTreeMap<_, _>>();

    let starting_nodes: Vec<&str> = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    let results = starting_nodes
        .iter()
        .map(|node| {
            let mut current_node = *node;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = nodes
                        .get(current_node)
                        .unwrap();
                    let next_node = match instruction {
                        Instruction::Left => options.left.as_str(),
                        Instruction::Right => options.right.as_str(),
                    };
                    if next_node.ends_with("Z") {
                        Some(index + 1)
                    } else {
                        current_node = next_node;
                        None
                    }
                })
                .unwrap()
        })
        .collect::<Vec<usize>>();

    let min_cycle = lcm(&results);

    min_cycle.to_string()
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "6");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "6");
    }
}
