use advent2024::*;
use regex::Regex;
fn main() {
    //part 1
    let res = part1(input!());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(input!());
    println!("Part 2: {}", res);
}
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+,[0-9]+\))").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [c])| {
            let mut i = ints::<u32>(c);
            (i.next().unwrap(), i.next().unwrap())
        })
        .map(|(a, b)| a * b)
        .sum()
}
fn part2(input: &str) -> u32 {
    let re = Regex::new(
        r"(?x)
            mul\((?P<mul>[0-9]+,[0-9]+)\) |
            (?P<do>do\(\)) |
            (?P<dont>don't\(\))
            ",
    )
    .unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .scan(true, |keep, (_, [m])| {
            match m {
                "do()" => *keep = true,
                "don't()" => *keep = false,
                _ => {
                    let mut i = ints(m);
                    if *keep {
                        return Some((i.next().unwrap(), i.next().unwrap()));
                    }
                }
            }
            Some((0, 0))
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[test]
fn test() {
    //part 1
    let res = part1(sample!());
    assert_eq!(res, 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5);
    //part 2
    let res = part2(sample!());
    assert_eq!(res, 48);
}
