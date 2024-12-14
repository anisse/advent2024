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

type ParsedItem = [u64; 6];
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input
        .split("\n\n")
        .map(|eq| ints(eq).collect::<Vec<_>>().try_into().unwrap())
}
fn part1<I>(equations: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    equations.map(min_token).sum()
}

fn min_token(eq: ParsedItem) -> u64 {
    const A_WEIGHT: u64 = 3;
    const B_WEIGHT: u64 = 1;
    // Who needs a struct anyway ?
    const AX: usize = 0;
    const AY: usize = 1;
    const BX: usize = 2;
    const BY: usize = 3;
    const PX: usize = 4;
    const PY: usize = 5;
    let arr: Vec<_> = (0..=100)
        .flat_map(|i| (0..=100).map(move |j| (i, j)))
        .filter(|(i, j)| eq[AX] * i + eq[BX] * j == eq[PX] && eq[AY] * i + eq[BY] * j == eq[PY])
        /*
        .inspect(|(i, j)| {
            println!(
                "{} * {i} + {} * {j} = {}\t\t{} * {i} + {} * {j} = {}",
                eq[AX], eq[BX], eq[PX], eq[AY], eq[BY], eq[PY]
            );
        })
            .inspect(|(i, j)| {
                println!(
                    "==? {}\t {} / {} = {} ;\t {} / {} = {} ;\t {} / {} = {}",
                    eq[AX] as f32 / eq[BX] as f32 == eq[AY] as f32 / eq[BY] as f32,
                    eq[AX],
                    eq[BX],
                    eq[AX] as f32 / eq[BX] as f32,
                    eq[AY],
                    eq[BY],
                    eq[AY] as f32 / eq[BY] as f32,
                    eq[PX],
                    eq[PY],
                    eq[PX] as f32 / eq[PY] as f32,
                );
            })
        .inspect(|(i, j)| {
            println!("{i}, {j}");
        })
        */
        .map(|(i, j)| i * A_WEIGHT + j * B_WEIGHT)
        .collect();
    //println!("There is {} solution(s)", arr.len());
    *arr.iter().min().unwrap_or(&0)
}
fn part2<I>(equations: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    equations
        .map(|eq| {
            [
                eq[0],
                eq[1],
                eq[2],
                eq[3],
                eq[4] + 10000000000000,
                eq[5] + 10000000000000,
            ]
        })
        .map(min_token2)
        .sum()
}
fn min_token2(eq: ParsedItem) -> u64 {
    const A_WEIGHT: f64 = 3.0;
    const B_WEIGHT: f64 = 1.0;
    // Who needs a struct anyway ?
    const A1: usize = 0;
    const A2: usize = 1;
    const B1: usize = 2;
    const B2: usize = 3;
    const C1: usize = 4;
    const C2: usize = 5;
    let a1 = eq[A1] as f64;
    let a2 = eq[A2] as f64;
    let b1 = eq[B1] as f64;
    let b2 = eq[B2] as f64;
    let c1 = eq[C1] as f64;
    let c2 = eq[C2] as f64;
    let delta = a1 * b2 - a2 * b1;
    let x = (c1 * b2 - c2 * b1) / delta;
    let y = (a1 * c2 - a2 * c1) / delta;
    if x < 0.0 || x.fract() != 0.0 || y < 0.0 || y.fract() != 0.0 {
        return 0;
    }
    //println!("{x}, {y}");
    (x * A_WEIGHT + y * B_WEIGHT) as u64
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
