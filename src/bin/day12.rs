use std::collections::VecDeque;

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
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(move |(x, c)| (Coord::from((x, y)), c))
        })
        .map(|(pos, c)| {
            if !seen[pos.y()][pos.x()] {
                let mut queue = VecDeque::<Coord>::new();
                let mut perimeter = 0;
                let mut surface = 0;

                queue.push_back(pos);
                seen[pos.y()][pos.x()] = true;
                while let Some(p) = queue.pop_front() {
                    println!("At {p:?} checking for {} of {pos:?} (currently at perim = {perimeter} and surface = {surface}) (seen:{})",*c as char, seen[p.y()][p.x()]);
                    surface += 1;
                    DIRS4.iter().for_each(|dir| {
                        let p2 = p + *dir;
                        if !p2.valid_for(map) {
                            perimeter += 1;
                            return;
                        }
                        if map[p2.y()][p2.x()] == *c {
                            if !seen[p2.y()][p2.x()] {
                                seen[p2.y()][p2.x()] = true;
                                queue.push_back(p2);
                            }
                        } else {
                            perimeter += 1;
                        }
                    });
                }
                println!("{} has perim of {perimeter} and surface of {surface}", *c as char);
                (perimeter, surface)
            } else {
                (0, 0)
            }
        })
        .map(|(a, b)| a * b)
        .sum()
}

fn part2(things: MapRef) -> usize {
    for _ in things {
        todo!()
    }
    42
}
#[test]
fn test() {
    assert_eq!(
        part1(&parse(
            "AAAA
BBCD
BBCC
EEEC"
        )),
        140
    );

    let things = parse(sample!());
    //part 1
    let res = part1(&things);
    assert_eq!(res, 1930);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 42);
}
