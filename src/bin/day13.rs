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

type ParsedItem = [i32; 6];
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input
        .split("\n\n")
        .map(|eq| ints(eq).collect::<Vec<_>>().try_into().unwrap())
}
fn part1<I>(equations: I) -> i32
where
    I: Iterator<Item = ParsedItem>,
{
    equations.map(min_token).sum()
}

fn min_token(eq: ParsedItem) -> i32 {
    const A_WEIGHT: i32 = 3;
    const B_WEIGHT: i32 = 1;
    const AX: usize = 0;
    const AY: usize = 1;
    const BX: usize = 2;
    const BY: usize = 3;
    const PX: usize = 4;
    const PY: usize = 5;
    (0..=100)
        .flat_map(|i| (0..=100).map(move |j| (i, j)))
        .filter(|(i, j)| eq[AX] * i + eq[BX] * j == eq[PX] && eq[AY] * i + eq[BY] * j == eq[PY])
        .map(|(i, j)| i * A_WEIGHT + j * B_WEIGHT)
        .min()
        .unwrap_or(0)
}
fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    for _ in things {
        todo!()
    }
    42
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 42);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
