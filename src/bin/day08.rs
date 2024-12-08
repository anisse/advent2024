use std::collections::{HashMap, HashSet};

use advent2024::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(&things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&things);
    println!("Part 2: {}", res);
}
fn parse(input: &str) -> Map {
    grid(input)
}
fn part1(map: MapRef) -> usize {
    let mut locations: HashMap<u8, Vec<Coord>> = HashMap::new();
    map.iter()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c != b'.')
                .map(move |(x, c)| (*c, Coord::from((x, y))))
        })
        .for_each(|(c, coord)| {
            (*locations.entry(c).or_default()).push(coord);
        });
    let set: HashSet<Coord> = locations
        .into_iter()
        .flat_map(|(_c, coords)| {
            (0..coords.len()).flat_map(move |i| {
                ((i + 1)..coords.len()).flat_map({
                    let coords = coords.clone();
                    move |j| {
                        //println!("For {}: {:?} vs {:?}", _c as char, coords[i], coords[j]);
                        let xdiff = coords[i].ix() - coords[j].ix();
                        let ydiff = coords[i].iy() - coords[j].iy();
                        [
                            Coord::from((coords[i].ix() + xdiff, coords[i].iy() + ydiff)),
                            Coord::from((coords[j].ix() - xdiff, coords[j].iy() - ydiff)),
                        ]
                        .into_iter()
                    }
                })
            })
        })
        /*
            .inspect(|c| {
                println!("{c:?}");
            })
        */
        .filter(|c| c.valid_for(map))
        //.filter(|c| map[c.y()][c.x()] == b'.')
        /*
        .inspect(|_| {
            println!("valid");
        })
        */
        .collect();
    set.len()
}

fn part2(map: MapRef) -> usize {
    let mut locations: HashMap<u8, Vec<Coord>> = HashMap::new();
    map.iter()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c != b'.')
                .map(move |(x, c)| (*c, Coord::from((x, y))))
        })
        .for_each(|(c, coord)| {
            (*locations.entry(c).or_default()).push(coord);
        });
    let xmax = map[0].len() as i32;
    let set: HashSet<Coord> = locations
        .into_iter()
        .flat_map(|(_c, coords)| {
            (0..coords.len()).flat_map(move |i| {
                ((i + 1)..coords.len()).flat_map({
                    let coords = coords.clone();
                    move |j| {
                        //println!("For {}: {:?} vs {:?}", _c as char, coords[i], coords[j]);
                        let xdiff = coords[i].ix() - coords[j].ix();
                        let ydiff = coords[i].iy() - coords[j].iy();
                        (0..=(xmax))
                            .map({
                                let coords = coords.clone();
                                move |k| {
                                    Coord::from((
                                        coords[i].ix() + k * xdiff,
                                        coords[i].iy() + k * ydiff,
                                    ))
                                }
                            })
                            .chain((0..=xmax).map({
                                let coords = coords.clone();
                                move |k| {
                                    Coord::from((
                                        coords[j].ix() - k * xdiff,
                                        coords[j].iy() - k * ydiff,
                                    ))
                                }
                            }))
                    }
                })
            })
        })
        /*
            .inspect(|c| {
                println!("{c:?}");
            })
        */
        .filter(|c| c.valid_for(map))
        /*
        .inspect(|_| {
            println!("valid");
        })
            */
        .collect();
    set.len()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(&things.clone());
    assert_eq!(res, 14);
    //part 2
    assert_eq!(
        9,
        part2(&parse(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
        ))
    );
    let res = part2(&things);
    assert_eq!(res, 34);
}
