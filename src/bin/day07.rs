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
    calibration(things, &[|a, b| a + b, |a, b| a * b])
}
fn part2<I>(things: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    calibration(things, &[|a, b| a + b, |a, b| a * b, concat])
}
fn calibration<I>(equations: I, operators: &[fn(u64, u64) -> u64]) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    equations
        .filter(|(result, operands)| {
            eval_equations(operands[0], &operands[1..], *result, operators)
        })
        .map(|(result, _)| result)
        .sum()
}

fn eval_equations(
    acc: u64,
    operands: &[u64],
    expected: u64,
    operators: &[fn(u64, u64) -> u64],
) -> bool {
    let mut results = operators.iter().map(|f| f(acc, operands[0]));
    if operands.len() == 1 {
        return results.any(|r| r == expected);
    }
    results.any(|r| eval_equations(r, &operands[1..], expected, operators))
}

fn concat(a: u64, b: u64) -> u64 {
    10_f64.powi(((b as f64).log10() + 1.0).floor() as i32) as u64 * a + b
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
