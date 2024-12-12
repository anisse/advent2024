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
                    //println!("At {p:?} checking for {} of {pos:?} (currently at perim = {perimeter} and surface = {surface}) (seen:{})",*c as char, seen[p.y()][p.x()]);
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
                /*
                println!(
                    "{} has perim of {perimeter} and surface of {surface}",
                    *c as char
                );
                */
                (perimeter, surface)
            } else {
                (0, 0)
            }
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[derive(Default, Clone, Copy)]
struct Seen(u8);
impl Seen {
    fn border_set(&mut self, dir: Dir) {
        self.0 |= 1 << dir as u8
    }
    fn border_get(&self, dir: Dir) -> bool {
        (self.0 & (1 << dir as u8)) != 0
    }
    fn surface_set(&mut self) {
        self.0 |= 1 << 5
    }
    fn surface_get(&self) -> bool {
        (self.0 & 1 << 5) != 0
    }
}
impl std::fmt::Display for Seen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in DIRS4.iter() {
            if self.border_get(*d) {
                write!(f, "{d:?} ")?;
            }
        }
        if self.surface_get() {
            return write!(f, "SURF");
        }
        Ok(())
    }
}

fn part2(map: MapRef) -> usize {
    //println!();
    //println!();
    let mut seen = vec![vec![Seen::default(); map[0].len()]; map.len()];
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(move |(x, c)| (Coord::from((x, y)), c))
        })
        .map(|(pos, c)| {
            if seen[pos.y()][pos.x()].surface_get() {
                return (0, 0);
            }
            let mut queue = VecDeque::<Coord>::new();
            let mut perimeter = 0;
            let mut surface = 0;

            queue.push_back(pos);
            seen[pos.y()][pos.x()].surface_set();
            while let Some(p) = queue.pop_front() {
                //println!("At {p:?} checking for {} of {pos:?} (currently at perim = {perimeter} and surface = {surface}) (seen:{})",*c as char, seen[p.y()][p.x()]);
                surface += 1;
                DIRS4.iter().for_each(|dir| {
                    if seen[p.y()][p.x()].border_get(*dir) {
                        //println!("Ignoring {dir:?} for {p:?}");
                        return;
                    }
                    // Explore dir in two directions, until we reach end of border
                    let p2 = p + *dir;
                    //println!("Looking at {dir:?} for {p:?} : {p2:?}");
                    if !p2.valid_for(map) {
                        perimeter += 1;
                        explore_dir(&p, *dir, &mut seen, map);
                        return;
                    }
                    if map[p2.y()][p2.x()] == *c {
                        if !seen[p2.y()][p2.x()].surface_get() {
                            seen[p2.y()][p2.x()].surface_set();
                            queue.push_back(p2);
                        }
                    } else {
                        perimeter += 1;
                        explore_dir(&p, *dir, &mut seen, map);
                    }
                });
            }
            /*
            println!(
                "{} has perim of {perimeter} and surface of {surface}",
                *c as char
            );
            */
            (perimeter, surface)
        })
        .map(|(a, b)| a * b)
        .sum()
}
fn explore_dir(from: &Coord, dir: Dir, seen: &mut [Vec<Seen>], map: MapRef) {
    let p2 = *from + dir;
    let dir1 = dir.rotate90();
    let dir2 = dir1.rotate180();
    let c = map[from.y()][from.x()];
    seen[from.y()][from.x()].border_set(dir);
    if !p2.valid_for(seen) {
        for next_dir in [dir1, dir2].into_iter() {
            let mut current = *from + next_dir;
            let mut next = current + dir;
            while current.valid_for(seen)
                && !next.valid_for(seen)
                && map[current.y()][current.x()] == c
            {
                //println!("Setting outer border {dir:?} for {current:?}");
                seen[current.y()][current.x()].border_set(dir);
                current = current + next_dir;
                next = current + dir;
            }
        }
        return;
    }
    if map[p2.y()][p2.x()] != c {
        for next_dir in [dir1, dir2].into_iter() {
            let mut current = *from + next_dir;
            let mut next = current + dir;
            while current.valid_for(seen)
                && map[current.y()][current.x()] == c
                && next.valid_for(seen)
                && map[next.y()][next.x()] != c
            {
                /*
                println!(
                    "Setting inner border {dir:?} for {current:?} {} vs {} {next:?}",
                    c as char,
                    map[next.y()][next.x()] as char
                );
                */
                seen[current.y()][current.x()].border_set(dir);
                current = current + next_dir;
                next = current + dir;
            }
        }
    }
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
    for (sample, res) in [
        (
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
            236,
        ),
        (
            "AAAA
BBCD
BBCC
EEEC",
            80,
        ),
        (
            "AAAA
AABA
ABAA
AAAA",
            14 * 12 + 4 * 2,
        ),
        (
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
            368,
        ),
    ]
    .into_iter()
    {
        assert_eq!(part2(&parse(sample)), res);
    }

    let things = parse(sample!());
    //part 1
    let res = part1(&things);
    assert_eq!(res, 1930);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 1206);
}
