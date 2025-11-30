/*
const fn sample() -> &'static str {
    include_str!(concat!(
        concat!("../samples/", env!("CARGO_BIN_NAME")),
        ".txt"
    ))
}
*/
#[macro_export]
macro_rules! sample {
    () => {
        include_str!(concat!(
            concat!("../samples/", env!("CARGO_BIN_NAME")),
            ".txt"
        ))
    };
}
#[cfg(feature = "ci_no_input")]
#[macro_export]
macro_rules! input_dir {
    () => {
        "../samples/"
    };
}
#[cfg(not(feature = "ci_no_input"))]
#[macro_export]
macro_rules! input_dir {
    () => {
        "../inputs/"
    };
}
#[macro_export]
macro_rules! input {
    () => {
        include_str!(concat!(
            concat!(input_dir!(), env!("CARGO_BIN_NAME")),
            ".txt"
        ))
    };
}

pub fn space_indent(recursion_level: u8, max: u8) {
    (0..(max - recursion_level)).for_each(|_| print! {" "});
}

pub fn ints<T>(s: &str) -> impl Iterator<Item = T> + Clone + '_
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split(|c: char| !(c.is_ascii_digit() || c == '-'))
        .filter(|x| !x.is_empty())
        .filter(|x| *x != "-")
        .map(|x| x.parse::<T>().expect("an int"))
}
#[test]
fn ints_test() {
    assert_eq!(
        ints("Hello 1: 42,3874 384|81  1").collect::<Vec<u16>>(),
        vec![1, 42, 3874, 384, 81, 1],
    );
    assert_eq!(
        ints("Hello 1: 42 -3874 - 384|81  -1").collect::<Vec<i16>>(),
        vec![1, 42, -3874, 384, 81, -1],
    );
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

/*
 * Grid library
 */
pub type Map = Vec<Vec<u8>>;
pub type MapRef<'a> = &'a [Vec<u8>];
pub type MapRefMut<'a> = &'a mut [Vec<u8>];
pub fn grid(s: &str) -> Map {
    s.lines().map(|l| l.bytes().collect()).collect()
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    North = 0,
    East,
    South,
    West,
}
use crate::Dir::*;
pub const DIRS4: [Dir; 4] = [North, East, South, West];
impl Dir {
    /* TODO: better direction operations */
    pub fn rotate90(self) -> Dir {
        ((self as u8 + 1) % 4).into()
    }
    pub fn rotate180(self) -> Dir {
        ((self as u8 + 2) % 4).into()
    }
    pub fn rotate270(self) -> Dir {
        ((self as u8 + 3) % 4).into()
    }
    pub fn is_vertical(self) -> bool {
        self == North || self == South
    }
}
impl From<Dir> for Coord {
    fn from(d: Dir) -> Coord {
        match d {
            North => Coord(0, -1),
            East => Coord(1, 0),
            South => Coord(0, 1),
            West => Coord(-1, 0),
        }
    }
}
impl From<u8> for Dir {
    fn from(value: u8) -> Self {
        match value {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord(i32, i32);
impl Coord {
    pub fn y(&self) -> usize {
        self.1 as usize
    }
    pub fn x(&self) -> usize {
        self.0 as usize
    }
    pub fn iy(&self) -> i32 {
        self.1
    }
    pub fn ix(&self) -> i32 {
        self.0
    }
    pub fn valid_for<T>(&self, map: &[Vec<T>]) -> bool {
        self.0 >= 0 && self.0 < map[0].len() as i32 && self.1 >= 0 && self.1 < map.len() as i32
    }
}
impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as i32, value.1 as i32)
    }
}
impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}
impl std::ops::Add<Dir> for Coord {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        let dir: Coord = rhs.into();
        Self(self.0 + dir.0, self.1 + dir.1)
    }
}

pub fn print_map(map: &[Vec<u8>]) {
    map.iter().for_each(|l| {
        l.iter().for_each(|c| {
            print!("{}", *c as char);
        });
        println!();
    })
}

pub fn iter_items(map: MapRef) -> impl Iterator<Item = (Coord, u8)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| ((x, y).into(), *c)))
}
pub fn find_first(map: MapRef, start: u8) -> Coord {
    iter_items(map)
        .filter(|(_, c)| *c == start)
        .map(|(coord, _)| coord)
        .next()
        .unwrap()
}
