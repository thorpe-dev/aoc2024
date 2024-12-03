use regex::Regex;

use crate::Day;

pub(crate) struct Day3 {
    data: String,
}

impl Day3 {
    pub(crate) fn new(data: String) -> Self {
        Self { data }
    }
}

#[derive(Debug, PartialEq)]
enum SimpleOp {
    Mul(i64, i64),
}

#[derive(Debug, PartialEq)]
enum Op {
    Mul(i64, i64),
    Enable,
    Disable,
}

fn parse_basic(input: &str) -> Vec<SimpleOp> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract::<2>().1)
        .map(|[lhs, rhs]| SimpleOp::Mul(lhs.parse::<i64>().unwrap(), rhs.parse::<i64>().unwrap()))
        .collect()
}

fn parse_with_op(input: &str) -> Vec<Op> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(input)
        .map(|c| match c.get(0).unwrap().as_str() {
            "do()" => Op::Enable,
            "don't()" => Op::Disable,
            _ => Op::Mul(c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()),
        })
        .collect()
}

impl Day for Day3 {
    fn first(&self) -> i64 {
        parse_basic(&self.data)
            .into_iter()
            .map(|op| match op {
                SimpleOp::Mul(lhs, rhs) => lhs * rhs,
            })
            .sum()
    }

    fn second(&self) -> i64 {
        parse_with_op(&self.data)
            .into_iter()
            .fold((Op::Enable, 0 as i64), |(prev_op, total), op| {
                match (prev_op, op) {
                    (_, Op::Enable) => (Op::Enable, total),
                    (_, Op::Disable) => (Op::Disable, total),
                    (Op::Disable, _) => (Op::Disable, total),
                    (Op::Enable, Op::Mul(lhs, rhs)) => (Op::Enable, total + lhs * rhs),
                    (Op::Mul(_, _), _) => panic!("Invalid!"),
                }
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    const TEST_BASIC_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    fn test_parsed_basic() -> Vec<SimpleOp> {
        vec![
            SimpleOp::Mul(2, 4),
            SimpleOp::Mul(5, 5),
            SimpleOp::Mul(11, 8),
            SimpleOp::Mul(8, 5),
        ]
    }

    #[test]
    fn test_parse_basic() {
        assert_eq!(parse_basic(TEST_BASIC_INPUT), test_parsed_basic());
    }

    #[test]
    fn test_1() {
        let d = Day3::new(TEST_BASIC_INPUT.to_string());
        assert_eq!(d.first(), 161);
    }

    const TEST_EXT_INPUT: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    fn test_parsed_ext() -> Vec<Op> {
        vec![
            Op::Mul(2, 4),
            Op::Disable,
            Op::Mul(5, 5),
            Op::Mul(11, 8),
            Op::Enable,
            Op::Mul(8, 5),
        ]
    }

    #[test]
    fn test_parse_ext() {
        assert_eq!(parse_with_op(TEST_EXT_INPUT), test_parsed_ext());
    }

    #[test]
    fn test_2() {
        let d = Day3::new(TEST_EXT_INPUT.to_string());
        assert_eq!(d.second(), 48);
    }
}
