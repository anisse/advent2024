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
                .map(move |(x, c)| Coord::from((x, y)))
        })
        .map(|c| trail(map, c, b'1'))
        .sum()
}

fn trail(map: MapRef, c: Coord, next: u8) -> usize {
    DIRS4
        .iter()
        .map(|dir| {
            let pos = c + *dir;
            if !pos.valid_for(map) {
                return 0;
            }
            let val = map[pos.y()][pos.x()];
            if val == b'9' {
                return 1;
            }
            if val == next {
                return trail(map, pos, next + 1);
            }
            0
        })
        .sum()
}

fn part2(map: MapRef) -> usize {
    todo!();
    42
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(&things);
    assert_eq!(res, 42);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 42);
}
