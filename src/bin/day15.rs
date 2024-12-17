use advent2024::*;
fn main() {
    let (map, moves) = parse(input!());
    //part 1
    let res = part1(&map, &moves);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map, &moves);
    println!("Part 2: {}", res);
}

use advent2024::Dir::*;

fn parse(input: &str) -> (Map, Vec<Dir>) {
    let mut parts = input.split("\n\n");
    (
        grid(parts.next().unwrap()),
        parts
            .next()
            .unwrap()
            .bytes()
            .filter_map(|c| match c {
                b'^' => Some(North),
                b'v' => Some(South),
                b'>' => Some(East),
                b'<' => Some(West),
                _ => None,
            })
            .collect(),
    )
}
fn part1(map: MapRef, moves: &[Dir]) -> usize {
    let mut pos = find_first(map, b'@');
    let mut map = map.to_vec();
    moves.iter().for_each(|m| {
        //print_map(&map);
        let next = pos + *m;
        if !next.valid_for(&map) {
            panic!("Invalid pos {next:?}");
        }
        if map[next.y()][next.x()] == b'#' {
            return;
        }
        if map[next.y()][next.x()] == b'O' {
            // Push all O recursively
            if !push(&mut map, next, *m) {
                return;
            }
        }
        map[pos.y()][pos.x()] = b'.';
        map[next.y()][next.x()] = b'@';
        pos = next;
    });

    iter_items(&map)
        .filter(|(_, c)| *c == b'O')
        .map(move |(coord, _)| coord.y() * 100 + coord.x())
        .sum()
}

//returns true on sucessful push
fn push(map: MapRefMut, pos: Coord, dir: Dir) -> bool {
    let next = pos + dir;
    if !next.valid_for(map) {
        panic!("Invalid pos {next:?}");
    }
    let c = &mut map[next.y()][next.x()];
    if *c == b'#' {
        return false;
    }
    if *c != b'O' || push(map, next, dir) {
        map[next.y()][next.x()] = b'O';
        map[pos.y()][pos.x()] = b'.';
        return true;
    }
    false
}

fn part2(map: MapRef, moves: &[Dir]) -> usize {
    42
}

#[test]
fn test() {
    let (map, moves) = parse(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
    );
    //part 1
    let res = part1(&map, &moves);
    assert_eq!(res, 2028);
    let (map, moves) = parse(sample!());
    //part 1
    let res = part1(&map, &moves);
    assert_eq!(res, 10092);
    //part 2
    let res = part2(&map, &moves);
    assert_eq!(res, 42);
}
