use crate::support::parse_error::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct Field {
    pub field: Vec<Vec<char>>,
    pub width: isize,
    pub height: isize,
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

impl Field {
    pub fn is_in_bounds(&self, point: &Point) -> bool {
        if (0..self.width).contains(&point.x) && (0..self.height).contains(&point.y) {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Default, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
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

impl std::ops::Mul<(isize, isize)> for Point {
    type Output = Point;

    fn mul(self, rhs: (isize, isize)) -> Self::Output {
        Self::Output {
            x: self.x * rhs.0,
            y: self.y * rhs.1,
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

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point {
            x: value.0 as isize,
            y: value.1 as isize,
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

    //pub fn from(t0: isize, t1: isize) -> Point {
    //    return Point { x: t0, y: t1 };
    //}
}
