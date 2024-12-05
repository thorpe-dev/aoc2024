use itertools::Itertools;
use std::{
    ascii::{self, Char},
    ops,
};

use crate::Day;

pub(crate) struct Day4 {
    data: String,
}

impl Day4 {
    pub(crate) fn new(data: String) -> Self {
        Self { data }
    }
}

const X: Char = Char::CapitalX;
const M: Char = Char::CapitalM;
const A: Char = Char::CapitalA;
const S: Char = Char::CapitalS;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

struct Move {
    x: i8,
    y: i8,
}

#[derive(Clone, Copy)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    fn movement(&self) -> Move {
        match *self {
            Dir::N => Move { x: 0, y: -1 },
            Dir::NE => Move { x: 1, y: -1 },
            Dir::E => Move { x: 1, y: 0 },
            Dir::SE => Move { x: 1, y: 1 },
            Dir::S => Move { x: 0, y: 1 },
            Dir::SW => Move { x: -1, y: 1 },
            Dir::W => Move { x: -1, y: 0 },
            Dir::NW => Move { x: -1, y: -1 },
        }
    }
}

fn iadd(a: usize, b: i8) -> usize {
    ((a as i64) + b as i64).try_into().unwrap()
}

impl ops::Add<Dir> for Coords {
    type Output = Option<Coords>;
    fn add(self, rhs: Dir) -> Self::Output {
        match (self, rhs.movement()) {
            (Coords { x: 0, y: _ }, Move { x, y: _ }) if x < 0 => None,
            (Coords { x: _, y: 0 }, Move { x: _, y }) if y < 0 => None,
            (c, m) => Some(Coords {
                x: iadd(c.x, m.x),
                y: iadd(c.y, m.y),
            }),
        }
    }
}

struct WordSearch {
    data: Vec<ascii::Char>,
    width: usize,
    height: usize,
}

impl WordSearch {
    fn new(data: &str) -> Self {
        let lines = data
            .lines()
            .into_iter()
            .map(|l| l.trim().as_ascii().unwrap())
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();

        let height = lines.len();
        assert_ne!(height, 0);
        let width = lines[0].len();

        assert!(lines.iter().all(|v| v.len() == width));
        Self {
            data: lines.concat(),
            width,
            height,
        }
    }

    fn to_pos(&self, index: usize) -> Coords {
        Coords {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn at(&self, place: Coords) -> Option<Char> {
        if place.x < self.width && place.y < self.height {
            self.data.get(place.x + self.width * place.y).copied()
        } else {
            None
        }
    }

    fn neighbor(&self, place: Coords, dir: Dir) -> Option<(Char, Coords)> {
        (place + dir).and_then(|pos| self.at(pos).zip(Some(pos)))
    }

    fn neighbor_is(&self, place: Coords, dir: Dir, val: Char) -> Option<(Coords, Dir)> {
        self.neighbor(place, dir)
            .and_then(|(c, pos)| if c == val { Some((pos, dir)) } else { None })
    }

    fn diags(&self, place: Coords) -> Option<[Char; 4]> {
        let vals = [Dir::NW, Dir::NE, Dir::SW, Dir::SE].map(|d| self.neighbor(place, d));

        if vals.contains(&None) {
            None
        } else {
            Some(vals.map(|v| v.unwrap().0))
        }
    }
}

impl Day for Day4 {
    fn first(&self) -> i64 {
        let grid = WordSearch::new(&self.data);

        let dirs = [
            Dir::N,
            Dir::NE,
            Dir::E,
            Dir::SE,
            Dir::S,
            Dir::SW,
            Dir::W,
            Dir::NW,
        ];

        (0..(grid.data.len()))
            .map(|idx| grid.to_pos(idx))
            .filter(|pos| grid.at(*pos).is_some_and(|c| c == X))
            .cartesian_product(dirs)
            .filter_map(|(cur, dir)| grid.neighbor_is(cur, dir, M))
            .filter_map(|(cur, dir)| grid.neighbor_is(cur, dir, A))
            .filter_map(|(cur, dir)| grid.neighbor_is(cur, dir, S))
            .map(|_| 1)
            .sum()
    }

    fn second(&self) -> i64 {
        let grid = WordSearch::new(&self.data);

        grid.data
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| {
                if *c == A {
                    Some(grid.to_pos(idx))
                } else {
                    None
                }
            })
            .filter_map(|pos| grid.diags(pos))
            .filter(|vals| match vals {
                [M, M, S, S] => true,
                [M, S, M, S] => true,
                [S, S, M, M] => true,
                [S, M, S, M] => true,
                _ => false,
            })
            .map(|_| 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    const TEST_INPUT: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_grid() {
        let res = WordSearch::new(TEST_INPUT);
        assert_eq!(res.width, 10);
        assert_eq!(res.height, 10);

        assert_eq!(res.at(Coords { x: 0, y: 0 }), Some(M));
        assert_eq!(res.at(Coords { x: 1, y: 1 }), Some(S));
        assert_eq!(res.at(Coords { x: 1, y: 2 }), Some(M));
        assert_eq!(res.at(Coords { x: 1, y: 3 }), Some(S));
        assert_eq!(res.at(Coords { x: 9, y: 9 }), Some(X));

        assert_eq!(res.neighbor(Coords { x: 0, y: 0 }, Dir::N), None);
        assert_eq!(
            res.neighbor(Coords { x: 0, y: 0 }, Dir::S),
            Some((M, Coords { x: 0, y: 1 }))
        );
        assert_eq!(
            res.neighbor(Coords { x: 0, y: 0 }, Dir::E),
            Some((M, Coords { x: 1, y: 0 }))
        );
    }

    #[test]
    fn test_1() {
        let d = Day4::new(TEST_INPUT.to_string());
        assert_eq!(d.first(), 18);
    }

    #[test]
    fn test_2() {
        let d = Day4::new(TEST_INPUT.to_string());
        assert_eq!(d.second(), 9);
    }
}
