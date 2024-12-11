use std::collections::HashMap;

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
fn parse(input: &str) -> Vec<u64> {
    ints(input).collect()
}
fn part1(stones: &[u64]) -> u64 {
    blink_recur_memo(stones, 25, &mut HashMap::new())
}

#[derive(Debug)]
enum StoneBlink {
    One([u64; 1]),
    Two([u64; 2]),
}

use crate::StoneBlink::*;

fn blink_stone(stone: u64) -> StoneBlink {
    if stone == 0 {
        return One([1]);
    }
    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let left = stone / (10_u64.pow(digits / 2));
        let up = left * (10_u64.pow(digits / 2));
        return Two([left, stone - up]);
    }
    One([stone * 2024])
}

fn blink_recur(stone: u64, depth: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    let b = blink_stone(stone);
    //println!("{stone} becomes {b:?}");
    if depth == 1 {
        match b {
            One(..) => return 1,
            Two(..) => return 2,
        }
    }
    match b {
        One(s) => blink_recur_memo(&s, depth - 1, memo),
        Two(s) => blink_recur_memo(&s, depth - 1, memo),
    }
}
fn blink_recur_memo(stones: &[u64], depth: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    stones
        .iter()
        .map(|s| {
            let key = (*s, depth);
            if let Some(count) = memo.get(&key) {
                return *count;
            }

            let count = blink_recur(*s, depth, memo);
            memo.insert(key, count);
            count
        })
        .sum()
}

fn part2(stones: &[u64]) -> u64 {
    blink_recur_memo(stones, 75, &mut HashMap::new())
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    assert_eq!(blink_recur_memo(&things, 1, &mut HashMap::new()), 3);
    assert_eq!(blink_recur_memo(&things, 2, &mut HashMap::new()), 4);
    //assert_eq!(blink_recur_memo(&things, 3, &mut HashMap::new()), 5);
    assert_eq!(blink_recur_memo(&things, 6, &mut HashMap::new()), 22);
    let res = part1(&things);
    assert_eq!(res, 55312);
    //part 2
    //let res = part2(&things);
    ////assert_eq!(res, 42);
}
