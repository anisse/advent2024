use std::{
    collections::{HashMap, HashSet},
    thread,
};

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
    verify_order(order, updates)
    /*
    // Needs a bigger stack
    let o2 = order.to_vec();
    let u2 = updates.to_vec();

    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || verify_order(&o2, &u2))
        .unwrap();
    let outside = child.join().unwrap();
    outside
    */
}

fn verify_order(order: &[Order], updates: &[Update]) -> usize {
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
            /*
            u.windows(2)
                .all(|s| is_after(s[0], s[1], &o, &mut HashSet::new()))
                */
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
    //let mut full = vec![];
    order.iter().cloned().for_each(|(a, b)| {
        order_after.entry(a).or_insert(HashSet::new()).insert(b);
        order_before.entry(b).or_insert(HashSet::new()).insert(a);
    });
    (order_after, order_before)
}

fn is_after(a: u8, b: u8, o: &OrderMap, visited: &mut HashSet<u8>) -> bool {
    //(0..(visited.len())).for_each(|_| print! {" "});
    //println!("Checking {a} -> {b}");
    visited.insert(a);
    // We look for b in the graph after a
    let ret = match o.get(&a) {
        Some(l) => l.iter().any(|x| {
            if *x == b {
                return true;
            }
            if visited.contains(x) {
                return false;
            }
            is_after(*x, b, o, visited)
        }),
        None => false,
    };
    visited.remove(&a);
    ret
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
