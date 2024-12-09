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
    dbg!(&disk);
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
    assert_eq!(res, 1928);
    //part 2
    let res = part2(things);
    assert_eq!(res, 42);
}
