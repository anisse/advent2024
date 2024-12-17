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
    let mut map: Map = map
        .iter()
        .map(|l| {
            l.iter()
                .flat_map(|c| {
                    match *c {
                        b'@' => [b'@', b'.'],
                        b'O' => [b'[', b']'],
                        _ => [*c, *c],
                    }
                    .into_iter()
                })
                .collect()
        })
        .collect();
    let mut pos = find_first(&map, b'@');
    moves.iter().for_each(|m| {
        println!("Next move: {m:?}");
        //print_map(&map);
        let next = pos + *m;
        assert!(next.valid_for(&map), "Invalid pos {next:?}");
        let cnext = map[next.y()][next.x()];
        if cnext == b'#' {
            return;
        }
        if [b'[', b']'].contains(&cnext) {
            /*
            let next = match cnext {
                b'[' => next,
                b']' => next + West,
                _ => unreachable!(),
            };
            */
            // Push all [] recursively
            if !push2(&mut map, next, *m) {
                return;
            }
        }
        map[pos.y()][pos.x()] = b'.';
        map[next.y()][next.x()] = b'@';
        pos = next;
    });

    iter_items(&map)
        .filter(|(_, c)| *c == b'[')
        .map(move |(coord, _)| coord.y() * 100 + coord.x())
        .sum()
}

//returns true on sucessful push
fn push2(map: MapRefMut, pos: Coord, dir: Dir) -> bool {
    let c = map[pos.y()][pos.x()];
    {
        println!("At pos {pos:?} to try to push {dir:?}");
        //let c = map[pos.y()][pos.x()];
        map[pos.y()][pos.x()] = b'x';
        //print_map(map);
        map[pos.y()][pos.x()] = c;
        println!();
    }
    assert!(
        [b'[', b']'].contains(&map[pos.y()][pos.x()]),
        "Invalid pushed box at {pos:?}: {}",
        map[pos.y()][pos.x()] as char
    );

    /*
    let next = if dir.is_vertical() {
        let pos2 = match map[pos.y()][pos.x()] {
            b'[' => pos + East,
            b']' => pos + West,
            _ => unreachable!(),
        };
        if [b'[', b']'].contains(&map[pos2.y()][pos2.x()]) {
            println!("Pushing second part {pos2:?}");
            push2(map, pos2, dir);
        }
        pos + dir
    } else {
        pos + dir + dir
    };
    */
    let next = pos + dir;
    assert!(next.valid_for(map), "Invalid pos {next:?}");
    let c = map[next.y()][next.x()];
    if c == b'#' {
        return false;
    }
    /*
    if ![b'[', b']'].contains(&c) {
        move_boxes(map, pos, dir, false);
        return true;
    }
    */
    println!("bli");
    if can_push2(map, pos, dir, false) {
        println!("Pushing {pos:?} to {next:?}");
        move_boxes(map, pos, dir, false);
        //push2(map, next, dir);
        /*
        if dir.is_vertical() {
            let next2 = match c {
                b'[' => next + East,
                b']' => next + West,
                _ => unreachable!(),
            };
            if [b'[', b']'].contains(&map[next2.y()][next2.x()]) {
                println!("Pushing second part {next2:?}");
                push2(map, next2, dir);
            }
        }
        */

        //move_box(map, pos, dir);
        return true;
    }
    false
}
/*
fn move_box(map: MapRefMut, pos: Coord, dir: Dir) {
    let prev_l = match map[pos.y()][pos.x()] {
        b'[' => pos,
        b']' if dir.is_vertical() => return,
        b']' => pos + West,
        _ => unreachable!(),
    };
    let prev_r = prev_l + East;
    let next_r = prev_r + dir;
    let next_l = prev_l + dir;
    map[next_l.y()][next_l.x()] = b'[';
    map[next_r.y()][next_r.x()] = b']';
    if ![next_r, next_l].contains(&prev_l) {
        map[prev_l.y()][prev_l.x()] = b'.';
    }
    if ![next_r, next_l].contains(&prev_r) {
        map[prev_r.y()][prev_r.x()] = b'.';
    }
}
*/

fn move_boxes(map: MapRefMut, pos: Coord, dir: Dir, is_sibling: bool) {
    let c = map[pos.y()][pos.x()];
    assert!([b'[', b']'].contains(&c), "Invalid pushed box at {pos:?}");
    println!("Moving {pos:?} {dir:?}");
    if dir.is_vertical() && !is_sibling {
        // move sibling, too
        println!("And its sibling {:?} {dir:?}", sibling(map, pos));
        move_boxes(map, sibling(map, pos), dir, true);
    }
    let next = pos + dir;
    let cnext = map[next.y()][next.x()];
    match cnext {
        b'[' | b']' => move_boxes(map, next, dir, false),
        b'#' => panic!("Move called to {next:?} which is a wall"),
        _ => {}
    };
    map[next.y()][next.x()] = map[pos.y()][pos.x()];
    map[pos.y()][pos.x()] = b'.';
}
fn sibling(map: MapRef, pos: Coord) -> Coord {
    assert!(
        [b'[', b']'].contains(&map[pos.y()][pos.x()]),
        "Invalid pushed box at {pos:?}"
    );
    match map[pos.y()][pos.x()] {
        b'[' => pos + East,
        b']' => pos + West,
        _ => unreachable!(),
    }
}

//returns true on sucessful push
fn can_push2(map: MapRef, pos: Coord, dir: Dir, is_sibling: bool) -> bool {
    println!("Verifying if we can push {pos:?} to {dir:?}");
    assert!(
        [b'[', b']'].contains(&map[pos.y()][pos.x()]),
        "Invalid pushed box at {pos:?}"
    );
    let next = if dir.is_vertical() {
        if !is_sibling && !can_push2(map, sibling(map, pos), dir, true) {
            return false;
        }
        pos + dir
    } else {
        // small optimization, skipping sibling
        pos + dir + dir
    };
    assert!(next.valid_for(map), "Invalid pos {next:?}");
    let cnext = map[next.y()][next.x()];
    if cnext == b'#' {
        return false;
    }
    if ![b'[', b']'].contains(&cnext) {
        return true;
    }
    /*
    if !dir.is_vertical() {
        return can_push2(map, next, dir);
    }
    let next2 = match c {
        b'[' => next + East,
        b']' => next + West,
        _ => unreachable!(),
    };
    */
    can_push2(map, next, dir, false) // && can_push2(map, next2, dir)
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
    //part 2 test
    let (map2, moves2) = parse(
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    );
    let res = part2(&map2, &moves2);
    //assert_eq!(res, 42);
    //part 2
    let res = part2(&map, &moves);
    assert_eq!(res, 9021);
}
