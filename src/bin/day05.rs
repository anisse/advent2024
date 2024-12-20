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
    let omap = order_graph(order);
    updates
        .iter()
        .filter(|u| u.is_sorted_by(ordered_by(&omap)))
        .map(|u| u[u.len() / 2] as usize)
        .sum()
}
type OrderMap = HashMap<u8, HashSet<u8>>;
fn order_graph(order: &[Order]) -> OrderMap {
    let mut order_map = HashMap::new();
    order.iter().cloned().for_each(|(a, b)| {
        order_map.entry(a).or_insert(HashSet::new()).insert(b);
    });
    order_map
}

fn part2(order: &[Order], updates: &[Update]) -> usize {
    let omap = order_graph(order);
    updates
        .iter()
        .filter(|u| !u.is_sorted_by(ordered_by(&omap)))
        .cloned()
        .map(|u| fix_order(u, &omap))
        .map(|u| u[u.len() / 2] as usize)
        .sum()
}

fn fix_order(mut u: Update, omap: &OrderMap) -> Update {
    let mut s = ordered_by(omap);
    u.sort_by(|a, b| {
        if s(a, b) {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Greater
    });
    u
}

fn ordered_by(omap: &OrderMap) -> impl FnMut(&u8, &u8) -> bool {
    |a, b| {
        if let Some(l) = omap.get(a) {
            if l.contains(b) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let (o, u) = parse(sample!());
    //part 1
    let res = part1(&o, &u);
    assert_eq!(res, 143);
    //part 2
    let res = part2(&o, &u);
    assert_eq!(res, 123);
}
