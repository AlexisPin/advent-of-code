pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter_map(|c| c.to_digit(10));

            let first = nums.next().unwrap();
            match nums.last() {
                Some(n) => format!("{first}{n}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum::<u32>();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");

            let mut nums = line.chars().filter_map(|c| c.to_digit(10));

            let first = nums.next().unwrap();
            match nums.last() {
                Some(n) => format!("{first}{n}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "281");
    }
}
