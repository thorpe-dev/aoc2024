use crate::Day;

pub(crate) struct Day2 {
    data: String,
}

impl Day2 {
    pub(crate) fn new(data: String) -> Self {
        Self { data }
    }
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect()
}

enum Direction {
    Unset,
    Asc,
    Dsc,
    Invalid,
}

fn current_direction(a: i64, b: i64) -> Direction {
    if a < b && b - a <= 3 {
        Direction::Asc
    } else if b < a && a - b <= 3 {
        Direction::Dsc
    } else {
        Direction::Invalid
    }
}

fn error_level(direction: Direction, elem: &[i64; 2]) -> Direction {
    let [a, b] = *elem;

    match (direction, current_direction(a, b)) {
        (Direction::Unset, val) => val,
        (Direction::Asc, Direction::Asc) => Direction::Asc,
        (Direction::Dsc, Direction::Dsc) => Direction::Dsc,
        (_, _) => Direction::Invalid,
    }
}

fn is_good(report: &Vec<i64>) -> bool {
    // Split the report into windows of (a, b), (b, c) etc
    match report
        .array_windows::<2>()
        .fold(Direction::Unset, error_level)
    {
        Direction::Unset => false,
        Direction::Invalid => false,
        Direction::Asc => true,
        Direction::Dsc => true,
    }
}

fn is_valid(report: &Vec<i64>) -> bool {
    if is_good(report) {
        return true;
    }

    (0..report.len())
        .map(|skip| {
            is_good(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip)
                    .map(|(_, x)| *x)
                    .collect(),
            )
        })
        .filter(|x| *x)
        .next()
        .unwrap_or(false)
}

impl Day for Day2 {
    fn first(&self) -> i64 {
        parse(&self.data)
            .into_iter()
            .filter(is_good)
            .map(|_| 1)
            .sum()
    }

    fn second(&self) -> i64 {
        parse(&self.data)
            .into_iter()
            .filter(is_valid)
            .map(|_| 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    const TEST_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    fn test_parsed() -> Vec<Vec<i64>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(TEST_INPUT), test_parsed());
    }

    #[test]
    fn test_1() {
        let d = Day2::new(TEST_INPUT.to_string());
        assert_eq!(d.first(), 2);
    }

    #[test]
    fn test_2() {
        let d = Day2::new(TEST_INPUT.to_string());
        assert_eq!(d.second(), 4);
    }
}
