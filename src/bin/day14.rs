use std::collections::HashMap;

use advent2024::*;
fn main() {
    let robots = parse(input!());
    //part 1
    let res = part1(robots.clone(), 101, 103);
    println!("Part 1: {}", res);
    //part 2
    let res = part2(robots, 101, 103);
    println!("Part 2: {}", res);
}
struct Robot {
    pos: Coord,
    vel: Coord,
}
type ParsedItem = Robot;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|l| {
        let mut li = ints::<i32>(l);
        Robot {
            pos: Coord::from((li.next().unwrap(), li.next().unwrap())),
            vel: Coord::from((li.next().unwrap(), li.next().unwrap())),
        }
    })
}
fn part1<I>(robots: I, width: i32, height: i32) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut quadrants = [0_usize; 4];
    robots
        .map(|r| {
            Coord::from((
                (r.pos.ix() + r.vel.ix() * 100 + 100 * width) % width,
                (r.pos.iy() + r.vel.iy() * 100 + 100 * height) % height,
            ))
        })
        .for_each(|pos| {
            if pos.iy() < height / 2 {
                //quadrant 1 & 2
                if pos.ix() < width / 2 {
                    quadrants[0] += 1;
                }
                if pos.ix() > width / 2 {
                    quadrants[1] += 1;
                }
            }
            if pos.iy() > height / 2 {
                //quadrant 3 & 4
                if pos.ix() < width / 2 {
                    quadrants[2] += 1;
                }
                if pos.ix() > width / 2 {
                    quadrants[3] += 1;
                }
            }
        });

    quadrants.iter().product()
}

fn part2<I>(robots: I, width: i32, height: i32) -> usize
where
    I: Iterator<Item = ParsedItem> + Clone,
{
    println!(
        "Merged period factors: {:?}",
        robots
            .clone()
            .inspect(|r| {
                println!(
                    "Robot has X {} has prime factors: {:?}",
                    r.vel.ix(),
                    prime_factors_below_100(r.vel.ix())
                );
            })
            .map(|r| prime_factors_below_100(r.vel.ix()))
            .reduce(|mut f1, f2| {
                f2.into_iter().for_each(|(key, mut value)| {
                    let e = f1.entry(key).or_insert(0);
                    *e = *e.max(&mut value);
                });
                f1
            })
            .map(
                |factors| factors.into_iter().fold(width as u128, |total, (k, v)| {
                    println!("Total: {total} * {k} * {v}");
                    total * k as u128 * v as u128
                })
            )
            .unwrap()
    );
    /*
    println!(
        "Max Y period: {}",
        robots
            .clone()
            .map(|r| lcm(r.vel.iy().unsigned_abs() as usize, height as usize))
            .inspect(|period| {
                println!("Robot has y period of {period}");
            })
            .max()
            .unwrap()
    );
    */
    0
    /*
    let robots: Vec<_> = robots.collect();
    for i in 1..1000 {
        let mut map = vec![vec![b' '; width as usize]; height as usize];
        println!("After {i} iterations:");
        robots
            .iter()
            .map(|r| {
                Coord::from((
                    (r.pos.ix() + r.vel.ix() * i + i * width) % width,
                    (r.pos.iy() + r.vel.iy() * i + i * height) % height,
                ))
            })
            .for_each(|pos| {
                map[pos.y()][pos.x()] = b'x';
            });
        print_map(&map);
        println!();
    }
    0
    */
}

fn prime_factors_below_100(mut n: i32) -> HashMap<u64, u64> {
    assert!(n <= 103);
    let primes: Vec<u64> = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103,
    ];
    let mut factors = HashMap::new();
    let mut i = 0;
    while n != 1 && i < primes.len() {
        if n % primes[i] as i32 == 0 {
            n /= primes[i] as i32;
            *factors.entry(primes[i]).or_insert(0) += 1;
        } else {
            i += 1;
        }
    }
    factors
}

#[test]
fn test() {
    let robots = parse(sample!());
    //part 1
    let res = part1(robots.clone(), 11, 7);
    assert_eq!(res, 42);
}
