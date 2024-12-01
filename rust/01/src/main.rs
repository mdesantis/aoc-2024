#![feature(test)]

extern crate test;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/01/input");

fn distances_sum(input_contents: &str) -> i32 {
    let mut lefts = BinaryHeap::new();
    let mut rights = BinaryHeap::new();
    let mut distances_sum = 0;

    for line in input_contents.lines() {
        for (i, slice) in line.split_whitespace().enumerate() {
            let value = slice.parse::<i32>().unwrap();

            match i % 2 {
                0 => lefts.push(Reverse(value)),
                _ => rights.push(Reverse(value)),
            }
        }
    }

    while let Some(Reverse(left)) = lefts.pop() {
        let Reverse(right) = rights.pop().unwrap();
        let distance = (left - right).abs();

        distances_sum += distance;
    }

    distances_sum
}

fn similarity_score(input_contents: &str) -> i32 {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    let mut similarity_score = 0;

    for line in input_contents.lines() {
        for (i, slice) in line.split_whitespace().enumerate() {
            let value = slice.parse::<i32>().unwrap();

            match i % 2 {
                0 => lefts.push(value),
                _ => rights.push(value),
            }
        }
    }

    for left in lefts.iter() {
        let count = rights.iter().filter(|&right| right == left).count();

        similarity_score += left * (count as i32);
    }

    similarity_score
}

fn main() {
    let distances_sum = distances_sum(INPUT_CONTENTS);
    println!("Distances sum: {distances_sum}");

    let similarity_score = similarity_score(INPUT_CONTENTS);
    println!("Similarity score: {similarity_score}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT_CONTENTS: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    ";
    const BENCH_INPUT_CONTENTS: &str = INPUT_CONTENTS;

    #[test]
    fn test_distances_sum() {
        let expected = 11;
        let actual = distances_sum(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_similarity_score() {
        let expected = 31;
        let actual = similarity_score(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }

    #[bench]
    fn bench_distances_sum(bencher: &mut Bencher) {
        bencher.iter(|| black_box(distances_sum(black_box(BENCH_INPUT_CONTENTS))));
    }
    #[bench]
    fn bench_similarity_score(bencher: &mut Bencher) {
        bencher.iter(|| black_box(similarity_score(black_box(BENCH_INPUT_CONTENTS))));
    }
}
