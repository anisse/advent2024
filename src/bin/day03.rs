use advent2024::*;
use regex::Regex;
fn main() {
    let things = parse1(input!());
    //part 1
    let res = part1(things);
    println!("Part 1: {}", res);
    //part 2
    let things = parse2(input!());
    let res = part2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = (u32, u32);
fn parse1(input: &str) -> Vec<ParsedItem> {
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+\))").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [c])| {
            let mut i = ints(c);
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect()
    /*
    input.match_indices("mul")
        .map(|i| if input.chars().nth(i + 1) == '(
        */
}
fn parse2(input: &str) -> Vec<ParsedItem> {
    let re =
        Regex::new(r"mul\((?P<mul>[0-9]+,[0-9]+)\)|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .scan(true, |keep, (_, [full])| {
            match full {
                "do()" => {
                    *keep = true;
                }
                "don't()" => *keep = false,
                _ => {
                    let mut i = ints(full);
                    if *keep {
                        return Some((i.next().unwrap(), i.next().unwrap()));
                    }
                }
            }
            Some((0, 0))
        })
        .collect()
}
fn part1(things: Vec<ParsedItem>) -> u32 {
    things.iter().map(|(a, b)| a * b).sum()
}

fn part2(things: Vec<ParsedItem>) -> u32 {
    things.iter().map(|(a, b)| a * b).sum()
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
