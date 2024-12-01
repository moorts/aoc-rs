use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let lines = input.lines();

    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in lines {
        let pair: Vec<&str> = line.split_ascii_whitespace().collect();

        let a = pair[0].parse::<i32>().unwrap();
        let b = pair[1].parse::<i32>().unwrap();

        left_heap.push(a);
        right_heap.push(b);
    }

    let mut total = 0;

    for (a, b) in left_heap.into_iter_sorted().zip(right_heap.into_iter_sorted()) {
        total += (a - b).abs();
    }

    return total;
}

pub fn part2(input: &str) -> impl Display {
    let lines = input.lines();

    let mut frequencies = HashMap::new();
    let mut locations = Vec::new();

    for line in lines {
        let pair: Vec<&str> = line.split_ascii_whitespace().collect();

        let a = pair[0].parse::<i32>().unwrap();
        let b = pair[1].parse::<i32>().unwrap();

        locations.push(a);
        *frequencies.entry(b).or_insert(0) += 1;
    }

    let mut similarity = 0;

    for id in locations {
        similarity += id * frequencies.get(&id).unwrap_or(&0);
    }

    return similarity;

}
