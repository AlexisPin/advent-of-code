use std::vec;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        left.push(parts.next().unwrap().parse::<i32>().unwrap());
        right.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let res = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
