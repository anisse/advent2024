use std::collections::HashSet;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    fn iter<'a>(&self, map: MapRef<'a>, dir: Dir) -> Iter<'a> {
        Iter {
            map,
            pos: *self,
            dir,
        }
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
struct Iter<'a> {
    map: MapRef<'a>,
    pos: Coord,
    dir: Dir,
}
impl Iterator for Iter<'_> {
    type Item = (Coord, Dir);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pos + self.dir.to_coord();
        if !next.valid_for(self.map) {
            return None;
        }
        if self.map[next.y()][next.x()] == b'#' {
            self.dir = self.dir.rotate();
            return self.next();
        }
        self.pos = next;
        Some((self.pos, self.dir))
    }
}
fn part1(map: MapRef) -> usize {
    let ipos = start_pos(map);
    let dir = North;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[ipos.y()][ipos.x()] = true;
    for (pos, _) in ipos.iter(map, dir) {
        visited[pos.y()][pos.x()] = true;
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
fn _print_dir_map(map: &[Vec<Option<Dir>>], map2: MapRef) {
    map.iter().zip(map2.iter()).for_each(|(l, l2)| {
        l.iter().zip(l2.iter()).for_each(|(c, c2)| match *c {
            Some(North) => print!("^"),
            Some(West) => print!("<"),
            Some(South) => print!("v"),
            Some(East) => print!(">"),
            None => print!("{}", *c2 as char),
        });
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
    let spos = start_pos(map);
    let dir = North;
    let mut obstacles = HashSet::<Coord>::new();
    let mut map2 = map.to_vec();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for (pos, dir) in spos.iter(map, dir) {
        //dbg!(pos);
        //dbg!(dir);

        let next = pos.iter(map, dir).next();
        if next.is_none() {
            continue;
        }
        let (next, _) = next.unwrap();
        if next.valid_for(map)
            && map[next.y()][next.x()] != b'#'
            && next != spos
            && !obstacles.contains(&next)
            && !visited[next.y()][next.x()]
        {
            let mut visited_loop: Vec<Vec<Option<Dir>>> = vec![vec![None; map[0].len()]; map.len()];
            /*
            println!(
                "{}: At ({},{}) Evaluating block at ({},{}) {dir:?}",
                obstacles.len(),
                pos.x(),
                pos.y(),
                next.x(),
                next.y()
            );
            */
            map2[next.y()][next.x()] = b'#';
            for (pos2, dir2) in pos.iter(&map2, dir.rotate()) {
                //dbg!(pos2);
                //dbg!(dir2);
                //dbg!(map[pos2.y()][pos2.x()]);
                if let Some(d) = visited_loop[pos2.y()][pos2.x()] {
                    if d == dir2 {
                        obstacles.insert(next);
                        //println!(".");
                        break;
                    }
                }
                visited_loop[pos2.y()][pos2.x()] = Some(dir2);
            }
            //map2[next.y()][next.x()] = b'O';
            //_print_dir_map(&visited_loop, &map2);
            map2[next.y()][next.x()] = b'.';
        }
        visited[pos.y()][pos.x()] = true;
    }
    obstacles.len()
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
