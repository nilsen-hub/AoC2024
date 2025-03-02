use crate::support::parse_error::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Field {
    field: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::NoData);
        }
        let data = s.lines();
        let mut field: Vec<Vec<char>> = Vec::with_capacity(1000);
        for line in data {
            field.push(line.chars().collect::<Vec<char>>());
        }

        let width = field[0].len() as isize;
        let height = field.len() as isize;

        Ok(Self {
            field,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add<(isize, isize)> for Point {
    type Output = Point;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}
impl std::ops::Sub<(isize, isize)> for Point {
    type Output = Point;

    fn sub(self, rhs: (isize, isize)) -> Self::Output {
        Self::Output {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    pub const NORTH: Point = Point { x: 0, y: -1 };
    pub const SOUTH: Point = Point { x: 0, y: 1 };
    pub const EAST: Point = Point { x: 1, y: 0 };
    pub const WEST: Point = Point { x: -1, y: 0 };
    pub const NORTH_WEST: Point = Point { x: -1, y: -1 };
    pub const NORTH_EAST: Point = Point { x: 1, y: -1 };
    pub const SOUTH_WEST: Point = Point { x: -1, y: 1 };
    pub const SOUTH_EAST: Point = Point { x: 1, y: 1 };

    pub fn from(t0: isize, t1: isize) -> Point {
        return Point { x: t0, y: t1 };
    }
}
