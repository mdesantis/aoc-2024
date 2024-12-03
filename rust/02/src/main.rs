const INPUT_CONTENTS: &str = include_str!("../../../inputs/02/input");

use std::cmp::Ordering;

fn is_report_safe(levels: &[i32]) -> bool {
    let prev_level = levels[0];
    let mut prev_report_ordering = prev_level.cmp(&levels[1]);

    for pair in levels.windows(2) {
        let (current, next) = (pair[0], pair[1]);
        let report_ordering = current.cmp(&next);

        if report_ordering == Ordering::Equal || current.abs_diff(next) > 3 {
            return false;
        }

        if report_ordering != prev_report_ordering {
            return false;
        }

        prev_report_ordering = report_ordering;
    }

    true
}

fn problem_dampener_levels<'a>(levels: &'a [i32]) -> impl Iterator<Item = Vec<i32>> + 'a {
    (0..levels.len()).map(|i| {
        let mut problem_dampener_level = levels.to_vec();

        problem_dampener_level.remove(i);

        problem_dampener_level
    })
}

fn safe_reports_amount(input_contents: &str, problem_dampener_module_enabled: bool) -> i32 {
    input_contents
        .lines()
        .filter_map(|line| {
            let levels = line
                .split_whitespace()
                .map(|slice| slice.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if is_report_safe(&levels) {
                return Some(());
            }

            if problem_dampener_module_enabled {
                return problem_dampener_levels(&levels)
                    .any(|levels| is_report_safe(&levels))
                    .then_some(());
            }

            None
        })
        .count() as i32
}

fn main() {
    let safe_reports_amount_with_problem_dampener_module_disabled =
        safe_reports_amount(INPUT_CONTENTS, false);
    println!(
        "Safe reports amount with problem dampener module disabled: {}",
        safe_reports_amount_with_problem_dampener_module_disabled
    );

    let safe_reports_amount_with_problem_dampener_module_enabled =
        safe_reports_amount(INPUT_CONTENTS, true);
    println!(
        "Safe reports amount with problem dampener module enabled: {}",
        safe_reports_amount_with_problem_dampener_module_enabled
    )
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
    fn test_safe_reports_amount_with_problem_dampener_module_disabled() {
        let expected = 2;
        let actual = safe_reports_amount(TEST_INPUT_CONTENTS, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_safe_reports_amount_with_problem_dampener_module_enabled() {
        let expected = 4;
        let actual = safe_reports_amount(TEST_INPUT_CONTENTS, true);
        assert_eq!(expected, actual);
    }
}
