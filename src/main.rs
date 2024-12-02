use aoc_rs::day2::*;
use std::io::Read;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let input = include_str!("../input/day2/input.txt");
    let example = include_str!("../input/day2/example.txt");

    println!("Part1:");
    println!("Example: {}", part1(&example));
    println!("Input: {}", part1(&input));
    println!("Part2:");
    println!("Example: {}", part2(&example));
    println!("Input: {}", part2(&input));

    Ok(())
}
