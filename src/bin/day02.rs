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
type ParsedItem = Vec<i16>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| ints(x).collect())
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .filter(|r| {
            let inc = r[0] < r[1];
            for x in r.windows(2) {
                if x[0] >= x[1] && inc {
                    return false;
                }
                if x[0] <= x[1] && !inc {
                    return false;
                }
                if (x[0] - x[1]).abs() > 3 {
                    return false;
                }
            }
            true
        })
        .inspect(|r| {
            dbg!(&r);
        })
        .count()
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things
        .filter(|r| {
            if test_report(r) {
                return true;
            }
            for i in 0..r.len() {
                let mut r2 = r.clone();
                r2.remove(i);
                if test_report(&r2) {
                    return true;
                }
            }
            false
        })
        .inspect(|r| {
            dbg!(&r);
        })
        .count()
}
fn test_report(r: &[i16]) -> bool {
    let inc = r[0] < r[1];
    for x in r.windows(2) {
        if !test_suite(x[0], x[1], inc) {
            return false;
        }
    }
    true
}
fn test_report_skipped(r: &[i16], skip: usize) -> bool {
    let inc = r[0] < r[1];
    for x in r.windows(2) {
        if !test_suite(x[0], x[1], inc) {
            return false;
        }
    }
    true
}

fn test_suite(a: i16, b: i16, inc: bool) -> bool {
    if a >= b && inc {
        return false;
    }
    if a <= b && !inc {
        return false;
    }
    if (a - b).abs() > 3 {
        return false;
    }
    true
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 2);
    //part 2
    let res = part2(things);
    assert_eq!(res, 4);
}
