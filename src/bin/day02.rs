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
                if test_report_skipped(r, i) {
                    return true;
                }
            }
            false
        })
        /*
            .inspect(|r| {
                dbg!(r);
            })
        */
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
    let mut it = r.iter().take(skip).chain(r.iter().skip(skip + 1));
    let mut prev = it.next().unwrap();
    let mut inc = false;
    for (i, n) in it.enumerate() {
        if i == 0 {
            inc = prev < n;
        }
        if !test_suite(*prev, *n, inc) {
            //println!("Unsafe {r:?} at {i} between {prev} and {n}, skipping {skip} ({inc})");
            return false;
        }
        prev = n;
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
