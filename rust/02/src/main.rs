const INPUT_CONTENTS: &str = include_str!("../../../inputs/02/input");

use std::cmp::Ordering;

fn is_report_safe(levels: &Vec<i32>) -> bool {
    let mut maybe_prev_level: Option<i32> = None;
    let mut maybe_prev_report_ordering: Option<Ordering> = None;
    let mut report_ordering;

    for (i, level) in levels.iter().enumerate() {
        if i > 0 {
            let prev_level = maybe_prev_level.unwrap();
            report_ordering = prev_level.cmp(level);

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

        maybe_prev_level = Some(*level);
    }

    true
}

fn problem_dampener_levels<'a>(levels: &'a Vec<i32>) -> impl Iterator<Item = Vec<i32>> + 'a {
    (0..levels.len()).map(|i| {
        let mut problem_dampener_level = (*levels).clone();

        problem_dampener_level.remove(i);

        problem_dampener_level
    })
}

fn safe_reports_amount(input_contents: &str, problem_dampener_module_enabled: bool) -> i32 {
    input_contents
        .lines()
        .filter(|line| {
            let levels = line
                .split_whitespace()
                .map(|slice| slice.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            match problem_dampener_module_enabled {
                false => is_report_safe(&levels),
                _ => {
                    is_report_safe(&levels)
                        || problem_dampener_levels(&levels).any(|levels| is_report_safe(&levels))
                }
            }
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
