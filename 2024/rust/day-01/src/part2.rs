use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        left.push(parts.next().unwrap().parse::<i32>().unwrap());
        right
            .entry(parts.next().unwrap().parse::<i32>().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let res: i32 = left.iter().map(|l| l * right.get(l).unwrap_or(&0)).sum();

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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
