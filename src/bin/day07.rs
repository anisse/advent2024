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
type ParsedItem = (u64, Vec<u64>);
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let mut ints = ints(l);
        (ints.next().unwrap(), ints.collect())
    })
}
fn part1<I>(things: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .filter(|(result, operands)| eval1(operands[0], &operands[1..], *result))
        .map(|(result, _)| result)
        .sum()
}

fn eval1(a: u64, b: &[u64], expected: u64) -> bool {
    if b.len() == 1 {
        return a * b[0] == expected || a + b[0] == expected;
    }
    eval1(a + b[0], &b[1..], expected) || eval1(a * b[0], &b[1..], expected)
}

fn part2<I>(things: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .filter(|(result, operands)| eval2(operands[0], &operands[1..], *result))
        .map(|(result, _)| result)
        .sum()
}
fn eval2(a: u64, b: &[u64], expected: u64) -> bool {
    if b.len() == 1 {
        return a * b[0] == expected || a + b[0] == expected || concat(a, b[0]) == expected;
    }
    eval2(a + b[0], &b[1..], expected)
        || eval2(a * b[0], &b[1..], expected)
        || eval2(concat(a, b[0]), &b[1..], expected)
}

fn concat(a: u64, b: u64) -> u64 {
    //10_f64.powi(((b as f64).log10() + 1.0).floor() as i32) as u64 * a + b
    format!("{a}{b}").parse::<u64>().unwrap()
}
#[test]
fn test_concat() {
    //println!("{}", 10_f64.powi(((1 as f64).log10() + 1.0).ceil() as i32));
    assert_eq!(concat(1, 1), 11);
    assert_eq!(concat(10, 142), 10142);
    assert_eq!(concat(15, 6), 156);
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 3749);
    //part 2
    let res = part2(things);
    assert_eq!(res, 11387);
}
