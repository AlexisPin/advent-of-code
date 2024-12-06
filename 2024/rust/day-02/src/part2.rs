#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let safe_reports = input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            is_safe(&levels, true, false) || is_safe(&levels, false, false)
        })
        .count();

    Ok(safe_reports.to_string())
}

fn is_safe(levels: &[i32], ascending: bool, dampener: bool) -> bool {
    let safe = levels.windows(2).all(|w| {
        let difference = (w[0] - w[1]).abs();
        if ascending {
            w[0] < w[1] && difference <= 3
        } else {
            w[0] > w[1] && difference <= 3
        }
    });

    if !safe && !dampener {
        for i in 0..levels.len() {
            let mut levels = Vec::from(levels);
            levels.remove(i);
            if is_safe(&levels, ascending, true) {
                return true;
            }
        }
        return false;
    }
    return safe;
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
