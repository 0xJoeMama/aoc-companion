use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point> for Point {
    type Output = i64;

    fn mul(self, rhs: Point) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl FromIterator<i64> for Point {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Point {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap_or(0),
        }
    }
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0, z: 0 };
    const I: Point = Point { x: 1, y: 0, z: 0 };
    const J: Point = Point { x: 0, y: 1, z: 0 };
    const K: Point = Point { x: 0, y: 0, z: 1 };
    const ZERO: Point = Point { x: 0, y: 0, z: 0 };

    #[must_use]
    pub fn origin() -> &'static Self {
        &Self::ORIGIN
    }

    #[must_use]
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// # Panics
    /// Panics if the input is the zero vector which cannot be normalized as it has no dimension
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn normalized(&self) -> Point {
        assert!(*self != Self::ZERO, "Cannot normalize zero vector!");
        *self / (self.len_squared() as f64).sqrt() as i64
    }

    #[must_use]
    pub fn distance_squared(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }

    #[must_use]
    pub fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    #[must_use]
    pub fn plane_neighbors(&self) -> PlaneNeighbours<'_> {
        PlaneNeighbours::new(self)
    }

    #[must_use]
    pub fn points_between(&self, other: &Point) -> PointsBetween {
        PointsBetween::new(*self, *other)
    }

    #[must_use]
    pub fn len_squared(&self) -> i64 {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }

    #[must_use]
    pub fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }

    #[must_use]
    pub fn up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    #[must_use]
    pub fn left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }

    #[must_use]
    pub fn right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }
}

pub struct PointsBetween {
    current: Point,
    end: Point,
    step: Point,
    done: bool,
}

impl PointsBetween {
    #[must_use]
    pub fn new(start: Point, end: Point) -> PointsBetween {
        let step = (end - start).normalized();
        Self {
            current: start,
            end: end + step,
            step,
            done: false,
        }
    }

    pub fn with_step(start: Point, end: Point, step: Point) -> PointsBetween {
        Self {
            current: start,
            end: end + step,
            step,
            done: false,
        }
    }
}

impl Iterator for PointsBetween {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.current;
        self.current += self.step;

        if self.current == self.end {
            self.done = true;
        }

        Some(current)
    }
}

pub struct PlaneNeighbours<'a> {
    p: &'a Point,
    dir_idx: usize,
    _phantom: PhantomData<&'a Point>,
}

impl PlaneNeighbours<'_> {
    fn new(p: &Point) -> PlaneNeighbours<'_> {
        PlaneNeighbours {
            p,
            dir_idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl Iterator for PlaneNeighbours<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Direction::plane()
            .get(self.dir_idx)
            .map(|d| *self.p + d.to_point());

        self.dir_idx += 1;
        res
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Direction {
    const PLANE: [Direction; 4] = [
        Direction::PosX,
        Direction::NegX,
        Direction::PosY,
        Direction::NegY,
    ];

    #[must_use]
    pub fn to_point(&self) -> Point {
        match self {
            Direction::PosX => Point::I,
            Direction::NegX => -Point::I,
            Direction::PosY => Point::J,
            Direction::NegY => -Point::J,
            Direction::PosZ => Point::K,
            Direction::NegZ => -Point::K,
        }
    }

    #[must_use]
    pub fn plane() -> &'static [Direction; 4] {
        &Self::PLANE
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn distance() {
        let a = Point::new(3, 0, 0);
        let b = Point::new(4, 0, 0);
        assert!(a.distance_squared(&b) == 1);
    }
}
