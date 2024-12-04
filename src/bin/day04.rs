use std::collections::HashMap;

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
type ParsedItem = Vec<u8>;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.lines().map(|x| x.bytes().collect())
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let grid: Vec<_> = things.collect();
    let len_x = grid[0].len() as i32;
    let len_y = grid.len() as i32;
    let mut count = 0;
    let word = b"XMAS";
    for (start, inc, dir) in [
        ((0, 0), (0_i32, 1_i32), (1_i32, 0)),
        ((0, 0), (1, 0), (0, 1)),
        ((len_x - 1, 0), (0, 1), (-1, 0)),
        ((0, len_y - 1), (1, 0), (0, -1)),
        /*diag 1 */
        ((0, 0), (1, 0), (1, 1)),
        ((0, 1), (0, 1), (1, 1)),
        /* diag 2 */
        ((len_x - 1, len_y - 1), (0, -1), (-1, -1)),
        ((len_x - 2, len_y - 1), (-1, 0), (-1, -1)),
        /* diag 3 */
        ((len_x - 1, 0), (-1, 0), (-1, 1)),
        ((len_x - 1, 1), (0, 1), (-1, 1)),
        /* diag 4 */
        ((0, len_y - 1), (0, -1), (1, -1)),
        ((1, len_y - 1), (1, 0), (1, -1)),
    ]
    .iter_mut()
    {
        let mut i = 0;
        while i < len_x && i < len_y {
            let mut x = (start.0 + (inc.0 * i)) as usize;
            let mut y = (start.1 + (inc.1 * i)) as usize;
            let mut pos_in_word = 0;

            while x < (len_x as usize) && y < (len_y as usize) {
                println!(
                    "At {x},{y} / {inc:?}, {dir:?}, pos is {pos_in_word}, letter is {} vs {}",
                    word[pos_in_word] as char, grid[y][x] as char
                );
                if grid[y][x] == word[pos_in_word] {
                    pos_in_word += 1;
                } else {
                    pos_in_word = 0;
                    if grid[y][x] == word[pos_in_word] {
                        pos_in_word += 1;
                    }
                }
                if pos_in_word == word.len() {
                    println!("found!");
                    count += 1;
                    pos_in_word = 0;
                }
                let x_i = x as i32 + dir.0;
                let y_i = y as i32 + dir.1;
                if x_i < 0 || y_i < 0 {
                    break;
                }
                x = x_i as usize;
                y = y_i as usize;
            }
            println!();
            i += 1;
        }
        println!();
    }
    count
}

fn part2<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let grid: Vec<_> = things.collect();
    let len_x = grid[0].len() as i32;
    let len_y = grid.len() as i32;
    let word = b"MAS";
    let mut a_positions: HashMap<(i32, i32), u8> = HashMap::new();
    for (start, inc, dir) in [
        /*diag 1 */
        ((0, 0), (1_i32, 0_i32), (1_i32, 1_i32)),
        ((0, 1), (0, 1), (1, 1)),
        /* diag 2 */
        ((len_x - 1, len_y - 1), (0, -1), (-1, -1)),
        ((len_x - 2, len_y - 1), (-1, 0), (-1, -1)),
        /* diag 3 */
        ((len_x - 1, 0), (-1, 0), (-1, 1)),
        ((len_x - 1, 1), (0, 1), (-1, 1)),
        /* diag 4 */
        ((0, len_y - 1), (0, -1), (1, -1)),
        ((1, len_y - 1), (1, 0), (1, -1)),
    ]
    .iter_mut()
    {
        let mut i = 0;
        while i < len_x && i < len_y {
            let mut x = (start.0 + (inc.0 * i)) as usize;
            let mut y = (start.1 + (inc.1 * i)) as usize;
            let mut pos_in_word = 0;

            while x < (len_x as usize) && y < (len_y as usize) {
                println!(
                    "At {x},{y} / {inc:?}, {dir:?}, pos is {pos_in_word}, letter is {} vs {}",
                    word[pos_in_word] as char, grid[y][x] as char
                );
                if grid[y][x] == word[pos_in_word] {
                    pos_in_word += 1;
                } else {
                    pos_in_word = 0;
                    if grid[y][x] == word[pos_in_word] {
                        pos_in_word += 1;
                    }
                }
                if pos_in_word == word.len() {
                    println!("found!");
                    pos_in_word = 0;
                    *a_positions
                        .entry((x as i32 - dir.0, y as i32 - dir.1))
                        .or_insert(0) += 1;
                }
                let x_i = x as i32 + dir.0;
                let y_i = y as i32 + dir.1;
                if x_i < 0 || y_i < 0 {
                    break;
                }
                x = x_i as usize;
                y = y_i as usize;
            }
            println!();
            i += 1;
        }
        println!();
    }
    a_positions.iter().filter(|(_, x)| **x > 1).count()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 18);
    //part 2
    let res = part2(things);
    assert_eq!(res, 9);
}
