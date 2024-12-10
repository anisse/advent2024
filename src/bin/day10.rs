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
fn parse(input: &str) -> Map {
    grid(input)
}
fn part1(map: MapRef) -> usize {
    map.iter()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == b'0')
                .map(move |(x, _)| Coord::from((x, y)))
        })
        .filter_map(|c| trail(map, c, b'1'))
        .map(|s| s.len())
        .sum()
}

fn trail(map: MapRef, c: Coord, next: u8) -> Option<HashSet<Coord>> {
    Some(
        DIRS4
            .iter()
            .filter_map(|dir| {
                let pos = c + *dir;
                if !pos.valid_for(map) {
                    return None;
                }
                let val = map[pos.y()][pos.x()];
                if val == next {
                    if val == b'9' {
                        return Some(HashSet::from([pos]));
                    }
                    return trail(map, pos, next + 1);
                }
                None
            })
            .fold(HashSet::new(), |s1, s2| &s1 | &s2),
    )
}

fn part2(map: MapRef) -> usize {
    map.iter()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == b'0')
                .map(move |(x, _)| Coord::from((x, y)))
        })
        .map(|c| trail_rating(map, c, b'1'))
        //.map(|c| if c > 0 { c - 1 } else { 0 })
        .sum()
}
fn trail_rating(map: MapRef, c: Coord, next: u8) -> usize {
    DIRS4
        .iter()
        .map(|dir| {
            let pos = c + *dir;
            if !pos.valid_for(map) {
                return 0;
            }
            let val = map[pos.y()][pos.x()];
            if val == next {
                if val == b'9' {
                    return 1;
                }
                return trail_rating(map, pos, next + 1);
            }
            0
        })
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    for (sample, res) in [
        (
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
            2,
        ),
        (
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
            4,
        ),
    ]
    .into_iter()
    {
        assert_eq!(part1(&parse(sample)), res);
    }

    let res = part1(&things);
    assert_eq!(res, 36);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 81);
}
