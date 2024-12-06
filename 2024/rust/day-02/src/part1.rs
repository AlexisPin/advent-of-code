#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let safe_reports = input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            is_safe(&levels, true) || is_safe(&levels, false)
        })
        .count();

    Ok(safe_reports.to_string())
}

fn is_safe(levels: &[i32], ascending: bool) -> bool {
    levels.windows(2).all(|w| {
        let difference = (w[0] - w[1]).abs();
        if ascending {
            w[0] < w[1] && difference <= 3
        } else {
            w[0] > w[1] && difference <= 3
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
