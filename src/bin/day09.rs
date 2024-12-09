use std::fmt::Display;

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
    input.lines().next().unwrap().bytes().map(|b| b - b'0')
}
#[derive(Debug, Clone, Copy)]
enum Block {
    Space,
    Id(u16),
}
fn part1<I>(files: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    let mut disk: Vec<Block> = files
        .enumerate()
        .flat_map(|(i, len)| {
            std::iter::repeat_n(
                match i % 2 {
                    0 => Block::Id((i / 2) as u16),
                    1 => Block::Space,
                    _ => unreachable!(),
                },
                len as usize,
            )
        })
        .collect();
    let mut end = disk.len() - 1;
    let mut start = 0;
    while end > start && start < disk.len() {
        while let Block::Id(_) = disk[start] {
            start += 1;
            if start >= disk.len() {
                break;
            }
        }
        while let Block::Space = disk[end] {
            end -= 1;
            disk.pop();
        }
        if start >= disk.len() {
            break;
        }
        if let Block::Space = disk[start] {
            if let Block::Id(_) = disk[end] {
                (disk[start], disk[end]) = (disk[end], disk[start]);
            }
        }
    }
    disk.into_iter()
        .enumerate()
        .map(|(i, b)| {
            if let Block::Id(id) = b {
                i as u64 * id as u64
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Block2 {
    Space { len: u8 },
    File { id: u16, len: u8 },
}
impl Display for Block2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block2::Space { len } => {
                write!(
                    f,
                    "{}",
                    std::iter::repeat_n(".", *len as usize).collect::<String>()
                )
            }
            Block2::File { id, len } => write!(
                f,
                "{}",
                std::iter::repeat_n(format!("{}", id % 10), *len as usize).collect::<String>()
            ),
        }
    }
}
fn _print_disk(d: &[Block2]) {
    d.iter().for_each(|b| {
        print!("{}", b);
    });
    println!();
}
fn part2<I>(files: I) -> u64
where
    I: Iterator<Item = ParsedItem>,
{
    let mut disk: Vec<Block2> = files
        .enumerate()
        .map(|(i, len)| match i % 2 {
            0 => Block2::File {
                id: (i / 2) as u16,
                len,
            },
            1 => Block2::Space { len },
            _ => unreachable!(),
        })
        .collect();
    //_print_disk(&disk);
    for file in (0..disk.len()).rev() {
        match disk[file] {
            Block2::Space { .. } => {
                //disk.pop();
                continue;
            }
            Block2::File { len, .. } => {
                //coucou
                for space in 0..file {
                    if let Block2::Space { len: len_space } = disk[space] {
                        if len_space >= len {
                            (disk[space], disk[file]) = (disk[file], Block2::Space { len });
                            if len_space > len {
                                disk.insert(space + 1, Block2::Space {
                                    len: len_space - len,
                                })
                            }
                            //println!("swapped {space} and {file}");
                            //_print_disk(&disk);
                            break;
                        }
                    }
                }
            }
        }
    }
    disk.into_iter()
        .scan(0, |s, b| match b {
            Block2::Space { len } => {
                *s += len as u64;
                Some(0)
            }
            Block2::File { id, len } => {
                *s += len as u64;
                Some((0..len).map(|i| (*s - (i + 1) as u64) * id as u64).sum())
            }
        })
        .sum()
}

#[test]
fn test() {
    let things = parse(sample!());
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 1928);
    //part 2
    let res = part2(things);
    assert_eq!(res, 2858);
}
