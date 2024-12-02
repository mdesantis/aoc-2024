const INPUT_CONTENTS: &str = include_str!("../../../inputs/02/input");

use std::cmp::Ordering;

fn is_report_safe(line: &str) -> bool {
    let levels = line
        .split_whitespace()
        .map(|slice| slice.parse::<i32>().unwrap());

    let mut maybe_prev_level: Option<i32> = None;
    let mut maybe_prev_report_ordering: Option<Ordering> = None;
    let mut report_ordering;

    for (i, level) in levels.enumerate() {
        if i > 0 {
            let prev_level = maybe_prev_level.unwrap();
            report_ordering = prev_level.cmp(&level);

            if report_ordering == Ordering::Equal {
                return false;
            }

            if (prev_level - level).abs() > 3 {
                return false;
            }

            if i > 1 && maybe_prev_report_ordering.unwrap() != report_ordering {
                return false;
            }

            maybe_prev_report_ordering = Some(report_ordering);
        }

        maybe_prev_level = Some(level);
    }

    true
}

fn safe_reports_amount(input_contents: &str) -> i32 {
    let mut safe_reports_amount = 0;

    for line in input_contents.lines() {
        if is_report_safe(line) {
            safe_reports_amount += 1
        }
    }

    safe_reports_amount
}

fn main() {
    let safe_reports_amount = safe_reports_amount(INPUT_CONTENTS);
    println!("Safe reports amount: {safe_reports_amount}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_safe_reports_amount() {
        let expected = 2;
        let actual = safe_reports_amount(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }
}
