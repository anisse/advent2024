use std::collections::{HashMap, HashSet};

use advent2024::*;
fn main() {
    let (o, u) = parse(input!());
    //part 1
    let res = part1(&o, &u);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(&o, &u);
    println!("Part 2: {}", res);
}

type Order = (u8, u8);
type Update = Vec<u8>;

fn parse(input: &str) -> (Vec<Order>, Vec<Update>) {
    let mut lines = input.split("\n\n");
    let orders = lines
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut i = ints(l);
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect();
    let updates = lines
        .next()
        .unwrap()
        .lines()
        .map(|l| ints(l).collect())
        .collect();
    (orders, updates)
}

fn part1(order: &[Order], updates: &[Update]) -> usize {
    let (obefore, oafter) = order_graph(order);
    updates
        .iter()
        .filter(|u| {
            for i in 0..u.len() {
                for j in (i + 1)..u.len() {
                    if let Some(l) = oafter.get(&u[i]) {
                        if l.contains(&u[j]) {
                            return false;
                        }
                    }
                    if let Some(l) = obefore.get(&u[j]) {
                        if l.contains(&u[i]) {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .inspect(|u| {
            println!("is ok: {u:?}");
        })
        .map(|u| u[u.len() / 2] as usize)
        .sum()
}
type OrderMap = HashMap<u8, HashSet<u8>>;
fn order_graph(order: &[Order]) -> (OrderMap, OrderMap) {
    let mut order_after = HashMap::new();
    let mut order_before = HashMap::new();
    order.iter().cloned().for_each(|(a, b)| {
        order_after.entry(a).or_insert(HashSet::new()).insert(b);
        order_before.entry(b).or_insert(HashSet::new()).insert(a);
    });
    (order_after, order_before)
}

fn part2(order: &[Order], updates: &[Update]) -> usize {
    42
}

#[test]
fn test() {
    let (o, u) = parse(sample!());
    //part 1
    let res = part1(&o, &u);
    assert_eq!(res, 143);
    //part 2
    let res = part2(&o, &u);
    assert_eq!(res, 42);
}
