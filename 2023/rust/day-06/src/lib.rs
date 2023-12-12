pub fn process_part1(input: &str) -> String {
    let mut it = input.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<_>>()
    });
    let times = it.next().unwrap();
    let distances = it.next().unwrap();
    let result = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            (0..time)
                .into_iter()
                .filter_map(|speed| {
                    let possible_distance = (time - speed) * speed;
                    (possible_distance > distance).then_some(possible_distance)
                })
                .count()
        })
        .product::<usize>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut it = input.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .map(|n: u32| n.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });

    let times = it.next().unwrap();

    let distances = it.next().unwrap();

    let result = (0..times)
        .into_iter()
        .filter_map(|speed| {
            let possible_distance = (times - speed) * speed;
            (possible_distance > distances).then_some(possible_distance)
        })
        .count();

    result.to_string()
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

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "71503");
    }
}
