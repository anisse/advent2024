use advent2024::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}
fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut lists = (vec![], vec![]);
    ints(input).enumerate().for_each(|(i, v)| {
        if i % 2 == 0 {
            lists.0.push(v)
        } else {
            lists.1.push(v)
        }
    });
    lists
}
fn part1(mut lists: (Vec<u32>, Vec<u32>)) -> u32 {
    lists.0.sort();
    lists.1.sort();
    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum()
}

fn part2(lists: (Vec<u32>, Vec<u32>)) -> u32 {
    lists
        .0
        .iter()
        .map(|a| *a * lists.1.iter().filter(|b| *b == a).count() as u32)
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 11);
    //part 2
    let res = part2(things);
    assert_eq!(res, 31);
}
