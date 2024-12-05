use std::{cmp::Ordering, collections::HashMap};

const INPUT_CONTENTS: &str = include_str!("../../../inputs/05/input");

fn collect_rules(rules_input: &str) -> HashMap<i32, Vec<i32>> {
    let mut rules = HashMap::new();

    for line in rules_input.lines() {
        let mut split = line.split("|");
        let (greater, lesser) = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );

        rules
            .entry(greater)
            .and_modify(|entry: &mut Vec<_>| entry.push(lesser))
            .or_insert(vec![lesser]);
    }

    rules
}

fn collect_updates_line_values(updates_line: &str) -> Vec<i32> {
    updates_line
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn are_updates_line_values_sorted(
    updates_line_values: &[i32],
    rules: &HashMap<i32, Vec<i32>>,
) -> bool {
    updates_line_values.is_sorted_by(|a, b| match rules.get(b) {
        Some(b_rule_values) => b_rule_values.iter().all(|v| *v != *a),
        _ => true,
    })
}

fn correctly_ordered_updates_middle_pages_sum(input_contents: &str) -> i32 {
    let mut split = input_contents.split("\n\n");
    let (rules_input, updates_input) = (split.next().unwrap(), split.next().unwrap());
    let rules = collect_rules(rules_input);

    updates_input
        .lines()
        .filter_map(|line| {
            let updates_line_values = collect_updates_line_values(line);
            let is_sorted = are_updates_line_values_sorted(&updates_line_values, &rules);

            is_sorted.then(|| updates_line_values[updates_line_values.len() / 2])
        })
        .sum::<i32>()
}

fn sort_updates_line_values(a: &i32, b: &i32, rules: &HashMap<i32, Vec<i32>>) -> Ordering {
    match (rules.get(a), rules.get(b)) {
        (Some(a_rule_values), _) if a_rule_values.iter().any(|v| *v == *b) => Ordering::Greater,
        (_, Some(b_rule_values)) if b_rule_values.iter().any(|v| *v == *a) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

fn reordered_wrongly_ordered_updates_middle_pages_sum(input_contents: &str) -> i32 {
    let mut split = input_contents.split("\n\n");
    let (rules_input, updates_input) = (split.next().unwrap(), split.next().unwrap());
    let rules = collect_rules(rules_input);

    updates_input
        .lines()
        .filter_map(|line| {
            let mut updates_line_values = collect_updates_line_values(line);
            let rules = &rules;

            (!are_updates_line_values_sorted(&updates_line_values, rules)).then(move || {
                updates_line_values.sort_by(|a, b| sort_updates_line_values(a, b, rules));

                updates_line_values[updates_line_values.len() / 2]
            })
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
