use std::collections::{BinaryHeap, HashMap, HashSet};

use advent2024::*;
fn main() {
    let map = grid(input!());
    //part 1
    let res = part1(&map);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&map);
    println!("Part 2: {}", res);
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Coord,
    dir: Dir,
    cost: u64,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| (other.pos.x() + other.pos.y()).cmp(&(self.pos.x() + self.pos.y())))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(map: MapRef) -> u64 {
    let start = find_first(map, b'S');
    let end = find_first(map, b'E');
    let mut cost_map = HashMap::new();
    let mut queue = BinaryHeap::with_capacity(4096);
    queue.push(State {
        pos: start,
        dir: Dir::East,
        cost: 0,
    });
    //print_map(map);
    while let Some(cur) = queue.pop() {
        /*
        println!(
            "Now evaluating pos {:?}, dir: {:?}, cost is {}",
            cur.pos, cur.dir, cur.cost
        );
        */
        if cur.pos == end {
            //   println!("END REACHED\n===============================\n");
            return cur.cost;
        }
        for next_dir in [cur.dir, cur.dir.rotate90(), cur.dir.rotate270()].into_iter() {
            let next_cost = cur.cost + if next_dir == cur.dir { 1 } else { 1000 };
            let next_pos = if next_dir == cur.dir {
                cur.pos + cur.dir
            } else {
                cur.pos
            };
            if !next_pos.valid_for(map) || map[next_pos.y()][next_pos.x()] == b'#' {
                continue;
            }
            if let Some(cost) = cost_map.get(&(next_pos, next_dir))
                && *cost <= next_cost
            {
                continue;
            }
            cost_map.insert((next_pos, next_dir), next_cost);
            queue.push(State {
                pos: next_pos,
                dir: next_dir,
                cost: next_cost,
            });
        }
    }
    unreachable!()
}

#[derive(Eq, PartialEq)]
struct State2(State, HashSet<Coord>);
impl Ord for State2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(map: MapRef) -> usize {
    let start = find_first(map, b'S');
    let end = find_first(map, b'E');
    let mut cost_map = HashMap::new();
    let mut queue = BinaryHeap::with_capacity(4096);
    let coord_set = HashSet::new();
    queue.push(State2(
        State {
            pos: start,
            dir: Dir::East,
            cost: 0,
        },
        coord_set,
    ));
    //print_map(map);
    while let Some(State2(cur, mut set)) = queue.pop() {
        /*
        println!(
            "Now evaluating pos {:?}, dir: {:?}, cost is {}",
            cur.pos, cur.dir, cur.cost
        );
        */
        if cur.pos == end {
            //   println!("END REACHED\n===============================\n");
            // Pop the rest of the identical costs and sum the sets, then count them
            set.insert(cur.pos);
            while let Some(State2(other, set2)) = queue.pop()
                && cur.cost == other.cost
                && cur.pos == other.pos
            {
                set = set.union(&set2).copied().collect();
            }
            /*
            let mut map2: Map = map.to_vec();
            set.iter().for_each(|c| map2[c.y()][c.x()] = b'O');
            print_map(&map2);
            */
            return set.len();
        }
        for next_dir in [cur.dir, cur.dir.rotate90(), cur.dir.rotate270()].into_iter() {
            let next_cost = cur.cost + if next_dir == cur.dir { 1 } else { 1000 };
            let mut next_set = set.clone();
            next_set.insert(cur.pos);
            let next_pos = if next_dir == cur.dir {
                cur.pos + cur.dir
            } else {
                cur.pos
            };
            if !next_pos.valid_for(map) || map[next_pos.y()][next_pos.x()] == b'#' {
                continue;
            }
            if let Some(cost) = cost_map.get(&(next_pos, next_dir))
                && *cost < next_cost
            {
                continue;
            }
            cost_map.insert((next_pos, next_dir), next_cost);
            queue.push(State2(
                State {
                    pos: next_pos,
                    dir: next_dir,
                    cost: next_cost,
                },
                next_set,
            ));
        }
    }
    unreachable!()
}

#[test]
fn test() {
    let things = grid(sample!());
    let things2 = grid(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    );
    //part 1
    let res = part1(&things);
    assert_eq!(res, 7036);
    assert_eq!(part1(&things2), 11048);
    //part 2
    let res = part2(&things);
    assert_eq!(res, 45);
    assert_eq!(part2(&things2), 64);
}
