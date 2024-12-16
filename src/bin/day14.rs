use std::collections::HashMap;

use advent2024::*;
fn main() {
    let robots = parse(input!());
    //part 1
    let res = part1(robots.clone(), 101, 103, 100);
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
fn part1<I>(robots: I, width: i32, height: i32, period: i32) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut quadrants = [0_usize; 4];
    robots
        .map(|r| {
            Coord::from((
                (r.pos.ix() + r.vel.ix() * period + period * width) % width,
                (r.pos.iy() + r.vel.iy() * period + period * height) % height,
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

fn part2<I>(robots: I, width: i32, height: i32) -> i32
where
    I: Iterator<Item = ParsedItem> + Clone,
{
    //let robots: Vec<_> = robots.collect();
    let res = (0..(width * height))
        .map(|i| (i, part1(robots.clone(), width, height, i)))
        .fold((0, 0), |(imax, max), (i, v)| {
            println!("{i}\t {v}");
            if v > max { (i, v) } else { (imax, max) }
        })
        .0;
    let mut map = vec![vec![b' '; width as usize]; height as usize];
    println!("After {res} iterations:");
    robots
        .map(|r| {
            Coord::from((
                (r.pos.ix() + r.vel.ix() * res + res * width) % width,
                (r.pos.iy() + r.vel.iy() * res + res * height) % height,
            ))
        })
        .for_each(|pos| {
            map[pos.y()][pos.x()] = b'x';
        });
    print_map(&map);

    res
}

#[test]
fn test() {
    let robots = parse(sample!());
    //part 1
    let res = part1(robots.clone(), 11, 7, 100);
    assert_eq!(res, 12);
}
