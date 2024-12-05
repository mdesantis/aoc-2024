use std::collections::HashMap;

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
    updates_line_values
        .iter()
        .enumerate()
        .is_sorted_by(|(a_i, a_k), (b_i, b_k)| {
            if *a_i == *b_i {
                return true;
            }

            let (greater_value, lesser_value) = if *a_i < *b_i { (a_k, b_k) } else { (b_k, a_k) };

            if let Some(lesser_rule_values) = rules.get(&lesser_value) {
                return lesser_rule_values
                    .iter()
                    .all(|lesser_rule_value| *lesser_rule_value != **greater_value);
            }

            true
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

            are_updates_line_values_sorted(&updates_line_values, &rules)
                .then(|| updates_line_values[updates_line_values.len() / 2])
        })
        .sum::<i32>()
}

fn main() {
    let result = correctly_ordered_updates_middle_pages_sum(INPUT_CONTENTS);
    println!("Correctly ordered updates middle pages sum: {result}");
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
}
