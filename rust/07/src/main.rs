#![feature(fn_traits, test)]

extern crate test;

use rayon::prelude::*;
use std::collections::HashMap;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/07/input");

fn combinations<'a, T: 'a + Copy>(a: &'a [T], n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();

    for v in a {
        let mut sub_combinations = combinations(a, n - 1);
        for combination in &mut sub_combinations {
            combination.push(*v);
        }
        result.extend(sub_combinations);
    }

    result
}

fn valid_combinations_results(
    combinations: &Vec<Vec<fn(i64, i64) -> i64>>,
    test_values: &Vec<i64>,
    result: i64,
) -> Option<i64> {
    combinations
        .par_iter()
        .any(|operations| any_valid_combination(operations, &test_values, result))
        .then_some(result)
}

fn any_valid_combination(
    operations: &Vec<fn(i64, i64) -> i64>,
    test_values: &Vec<i64>,
    result: i64,
) -> bool {
    let mut operations_iter = operations.iter();
    let test_result = test_values
        .clone()
        .into_iter()
        .reduce(|acc, test_value| operations_iter.next().unwrap().call((acc, test_value)))
        .unwrap();

    test_result == result
}

fn is_valid_equation_line(
    line: &str,
    operations: &Vec<fn(i64, i64) -> i64>,
    combinations_cache: &mut HashMap<usize, Vec<Vec<fn(i64, i64) -> i64>>>,
) -> Option<i64> {
    let (result_string, test_values_string) = line.split_once(':').unwrap();
    let result = result_string.parse::<i64>().unwrap();
    let test_values = test_values_string
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let combinations_amount = test_values.len() - 1;
    let combinations = combinations_cache
        .entry(combinations_amount)
        .or_insert_with(|| combinations(&operations, combinations_amount));

    valid_combinations_results(&combinations, &test_values, result)
}

fn total_calibration_result(input_contents: &str) -> i64 {
    let operations = vec![<i64 as std::ops::Add>::add, <i64 as std::ops::Mul>::mul];
    let mut combinations_cache = HashMap::new();

    input_contents
        .lines()
        .filter_map(|line| is_valid_equation_line(line, &operations, &mut combinations_cache))
        .sum::<i64>()
}

fn concatenate(a: i64, b: i64) -> i64 {
    (a.to_string() + &b.to_string()).parse::<i64>().unwrap()
}

fn total_calibration_with_concatenation_result(input_contents: &str) -> i64 {
    let operations = vec![
        <i64 as std::ops::Add>::add,
        <i64 as std::ops::Mul>::mul,
        concatenate,
    ];
    let mut combinations_cache = HashMap::new();

    input_contents
        .lines()
        .filter_map(|line| is_valid_equation_line(line, &operations, &mut combinations_cache))
        .sum::<i64>()
}

fn main() {
    let result = total_calibration_result(INPUT_CONTENTS);
    println!("Total calibration result: {result}");

    let result = total_calibration_with_concatenation_result(INPUT_CONTENTS);
    println!("Total calibration with concatenation result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT_CONTENTS: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    const BENCH_INPUT_CONTENTS: &str = "31084: 8 67 8 735 38
40541461584: 5 81 9 4 32 43 283 228
6208: 915 6 2 539 148 29
306501986: 6 3 630 45 536 64 6
21276000: 8 5 5 91 6 5 8 6 8 985 2 8
1432991: 3 403 4 821 591 2 236
17115994: 69 321 254 764 3 99
19961295: 5 311 84 6 63
22953531: 22 95 35 2 8
18894343830: 90 5 3 2 65 7 159 2 91 1
4312695565: 158 8 716 615 95 59
391: 1 326 64
25202: 8 5 62 1 401
5324514: 6 8 466 59 169
658234: 3 83 592 97 57 4
26804472776473: 7 61 1 447 29 496 71 9
1503368: 653 23 18 824 625
24803314445: 5 2 4 4 417 1 987 4 447
1021069871: 7 3 2 7 9 37 2 2 8 5 58
1614: 63 645 4 3 31 868
";

    #[test]
    fn test_total_calibration_result() {
        let expected = 3749_i64;
        let actual = total_calibration_result(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_total_calibration_with_concatenations_result() {
        let expected = 11387_i64;
        let actual = total_calibration_with_concatenation_result(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }

    #[bench]
    fn bench_total_calibration_with_concatenation_result(bencher: &mut Bencher) {
        bencher.iter(|| {
            black_box(total_calibration_with_concatenation_result(black_box(
                BENCH_INPUT_CONTENTS,
            )))
        });
    }
}
