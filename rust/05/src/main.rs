use std::{cmp::Ordering, collections::HashMap};

const INPUT_CONTENTS: &str = include_str!("../../../inputs/05/input");

fn collect_rules(rules_input: &str) -> HashMap<&str, Vec<&str>> {
    rules_input
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .fold(HashMap::new(), |mut acc, (greater, less)| {
            acc.entry(greater).or_default().push(less);
            acc
        })
}

fn split_updates_line(updates_line: &str) -> std::str::Split<'_, &str> {
    updates_line.split(",")
}

fn is_updates_line_sorted(line: &str, rules: &HashMap<&str, Vec<&str>>) -> bool {
    split_updates_line(line).is_sorted_by(|a, b| match rules.get(b) {
        Some(b_rule_values) => b_rule_values.iter().all(|v| *v != *a),
        _ => true,
    })
}

fn correctly_ordered_updates_middle_pages_sum(input_contents: &str) -> i32 {
    let (rules_input, updates_input) = input_contents.split_once("\n\n").unwrap();
    let rules = collect_rules(rules_input);

    updates_input
        .lines()
        .filter_map(|line| {
            if is_updates_line_sorted(line, &rules) {
                let updates_line_values = split_updates_line(line).collect::<Vec<_>>();

                return updates_line_values[updates_line_values.len() / 2]
                    .parse::<i32>()
                    .ok();
            }

            None
        })
        .sum::<i32>()
}

fn sort_updates_line_values(a: &str, b: &str, rules: &HashMap<&str, Vec<&str>>) -> Ordering {
    match (rules.get(a), rules.get(b)) {
        (Some(a_rule_values), _) if a_rule_values.iter().any(|v| *v == b) => Ordering::Greater,
        (_, Some(b_rule_values)) if b_rule_values.iter().any(|v| *v == a) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

fn reordered_wrongly_ordered_updates_middle_pages_sum(input_contents: &str) -> i32 {
    let (rules_input, updates_input) = input_contents.split_once("\n\n").unwrap();
    let rules = collect_rules(rules_input);

    updates_input
        .lines()
        .filter_map(|line| {
            if !is_updates_line_sorted(line, &rules) {
                let mut updates_line_values = split_updates_line(line).collect::<Vec<_>>();

                updates_line_values.sort_by(|a, b| sort_updates_line_values(a, b, &rules));

                return updates_line_values[updates_line_values.len() / 2]
                    .parse::<i32>()
                    .ok();
            }

            None
        })
        .sum::<i32>()
}

fn main() {
    let result = correctly_ordered_updates_middle_pages_sum(INPUT_CONTENTS);
    println!("Correctly ordered updates middle pages sum: {result}");

    let result = reordered_wrongly_ordered_updates_middle_pages_sum(INPUT_CONTENTS);
    println!("Reordered wrongly ordered updates middle pages sum: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_correctly_ordered_updates_middle_pages_sum() {
        let expected = 143;
        let actual = correctly_ordered_updates_middle_pages_sum(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_reordered_wrongly_ordered_updates_middle_pages_sum() {
        let expected = 123;
        let actual = reordered_wrongly_ordered_updates_middle_pages_sum(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }
}
