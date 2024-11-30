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
type ParsedItem = u8;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| x.parse().expect("not int"))
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    for _ in things {
        todo!()
    }
    42
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
