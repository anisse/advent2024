use advent2024::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(&things);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&things);
    println!("Part 2: {}", res);
}
type Map = Vec<Vec<u8>>;
type MapRef<'a> = &'a [Vec<u8>];
fn parse(input: &str) -> Map {
    input.lines().map(|l| l.bytes().collect()).collect()
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
use crate::Dir::*;
impl Dir {
    fn rotate(self) -> Dir {
        ((self as u8 + 1) % 4).into()
    }
    fn to_coord(self) -> Coord {
        match self {
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
#[derive(Clone, Copy, Debug)]
struct Coord(i32, i32);
impl Coord {
    fn y(&self) -> usize {
        self.1 as usize
    }
    fn x(&self) -> usize {
        self.0 as usize
    }
    fn valid_for(&self, map: MapRef) -> bool {
        self.0 >= 0 && self.0 < map[0].len() as i32 && self.1 >= 0 && self.1 < map.len() as i32
    }
}
impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as i32, value.1 as i32)
    }
}
impl std::ops::Add<Self> for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
fn part1(map: MapRef) -> usize {
    let mut ipos = start_pos(map);
    let mut dir = North;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    while ipos.valid_for(map) {
        visited[ipos.y()][ipos.x()] = true;
        let next = ipos + dir.to_coord();
        if next.valid_for(map) && map[next.y()][next.x()] == b'#' {
            dir = dir.rotate();
            continue;
        }
        ipos = next;
        //_print_map(&visited);
    }
    visited.into_iter().flatten().filter(|x| *x).count()
}

fn _print_map(map: &[Vec<bool>]) {
    map.iter().for_each(|l| {
        l.iter()
            .for_each(|c| if *c { print!("x") } else { print!(".") });
        println!();
    })
}

fn start_pos(map: MapRef) -> Coord {
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == b'^')
                .map(move |(x, _)| (x, y))
        })
        .next()
        .unwrap()
        .into()
}

fn part2(map: MapRef) -> usize {
    let mut ipos = start_pos(map);
    let mut dir = North;
    let mut visited: Vec<Vec<Option<Dir>>> = vec![vec![None; map[0].len()]; map.len()];
    let mut count = 0;
    while ipos.valid_for(map) {
        let next = ipos + dir.to_coord();
        if next.valid_for(map)
            && map[next.y()][next.x()] != b'#'
            && visited_same_dir(map, ipos, dir.rotate(), &visited)
        {
            count += 1;
        }
        visited[ipos.y()][ipos.x()] = Some(dir);
        if next.valid_for(map) && map[next.y()][next.x()] == b'#' {
            dir = dir.rotate();
            continue;
        }
        ipos = next;
    }
    count
    //visited.into_iter().flatten().filter(|x| *x).count()
}

fn visited_same_dir(map: MapRef, mut pos: Coord, dir: Dir, visited: &[Vec<Option<Dir>>]) -> bool {
    while pos.valid_for(map) {
        if let Some(d) = visited[pos.y()][pos.x()] {
            if d == dir {
                return true;
            }
        }
        pos = pos + dir.to_coord();
    }
    false
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(&things);
    assert_eq!(res, 41);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 6);
}