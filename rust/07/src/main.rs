#![feature(fn_traits)]

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

fn total_calibration_result(input_contents: &str) -> i64 {
    let operations = vec![<i64 as std::ops::Add>::add, <i64 as std::ops::Mul>::mul];

    input_contents
        .lines()
        .filter_map(|line| {
            let (result_string, test_values_string) = line.split_once(':').unwrap();
            let result = result_string.parse::<i64>().unwrap();
            let test_values = test_values_string
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let combinations = combinations(&operations, test_values.len() - 1);
            combinations
                .iter()
                .any(|operations| {
                    let mut operations_iter = operations.iter();
                    let test_result = test_values
                        .clone()
                        .into_iter()
                        .reduce(|acc, test_value| {
                            let operation = operations_iter.next().unwrap();
                            let value = operation.call((acc, test_value));
                            value
                        })
                        .unwrap();

                    test_result == result
                })
                .then_some(result)
        })
        .sum::<i64>()
}

fn main() {
    let result = total_calibration_result(INPUT_CONTENTS);
    println!("Total calibration result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_total_calibration_result() {
        let expected = 3749_i64;
        let actual = total_calibration_result(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }
}
