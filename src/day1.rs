use std::collections::HashMap;

use crate::Day;
use itertools::Itertools;

pub(crate) struct Day1 {
    data: String,
}

impl Day1 {
    pub(crate) fn new(data: String) -> Self {
        Self { data }
    }
}

fn parse(input: &str) -> [Vec<i64>; 2] {
    let mut result = [Vec::new(), Vec::new()];

    for line in input.lines() {
        match line.trim().split_once(" ") {
            Some((lhs, rhs)) => {
                result[0].push(lhs.trim().parse::<i64>().unwrap());
                result[1].push(rhs.trim().parse::<i64>().unwrap());
            }
            _ => {}
        }
    }

    result
}

fn sorted_vec(val: Vec<i64>) -> Vec<i64> {
    val.into_iter().sorted().collect()
}

fn cmp(elem: (&i64, &i64)) -> i64 {
    let (lhs, rhs) = elem;

    (lhs - rhs).abs()
}

impl Day for Day1 {
    fn first(&self) -> i64 {
        let parsed = parse(&self.data);
        let sorted = parsed.map(&sorted_vec);

        let zip = sorted[0].iter().zip(sorted[1].iter());

        zip.map(cmp).sum()
    }

    fn second(&self) -> i64 {
        let [lhs, rhs] = parse(&self.data);

        let mut map = HashMap::with_capacity(rhs.len());

        for i in rhs.iter() {
            *map.entry(i).or_insert(0) += 1;
        }

        lhs.iter()
            .map(|v| match map.get(v) {
                Some(d) => d * v,
                None => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    const TEST_INPUT: &str = r"\
3   4
4   3
2   5
1   3
3   9
3   3
";

    fn test_parsed() -> [Vec<i64>; 2] {
        [vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]]
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(TEST_INPUT), test_parsed());
    }

    #[test]
    fn test_1() {
        let d = Day1::new(TEST_INPUT.to_string());
        assert_eq!(d.first(), 11);
    }

    #[test]
    fn test_2() {
        let d = Day1::new(TEST_INPUT.to_string());
        assert_eq!(d.second(), 31);
    }
}
