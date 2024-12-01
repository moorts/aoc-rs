use aoc_rs::day1::*;
use std::io::Read;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut file = File::open("../input/day1/input.txt")?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}
