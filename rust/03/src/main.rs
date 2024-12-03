use regex::Regex;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/03/input");

fn multiplications_sum(input_contents: &str) -> i32 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    regex
        .captures_iter(input_contents)
        .map(|captures| {
            let (_, [first, second]) = captures.extract();

            first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

fn enabled_multiplications_sum(input_contents: &str) -> i32 {
    let regex = Regex::new(
        r"(mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\))|(?<disabler>don't)|(?<enabler>do)",
    )
    .unwrap();

    let mut enabled = true;

    regex
        .captures_iter(input_contents)
        .filter_map(|captures| {
            enabled = match (captures.name("disabler"), captures.name("enabler")) {
                (Some(_), _) => false,
                (_, Some(_)) => true,
                _ => enabled,
            };

            if enabled {
                let first = captures.name("first")?.as_str().parse::<i32>().unwrap();
                let second = captures.name("second")?.as_str().parse::<i32>().unwrap();

                return Some(first * second);
            }

            None
        })
        .sum()
}

fn main() {
    let multiplications_sum = multiplications_sum(INPUT_CONTENTS);
    println!("Multiplications sum: {multiplications_sum}");

    let enabled_multiplications_sum = enabled_multiplications_sum(INPUT_CONTENTS);
    println!("Enabled multiplications sum: {enabled_multiplications_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_CONTENTS_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_multiplications_sum() {
        let expected = 161;
        let actual = multiplications_sum(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_enabled_multiplications_sum() {
        let expected = 48;
        let actual = enabled_multiplications_sum(TEST_INPUT_CONTENTS_2);

        assert_eq!(expected, actual);
    }
}
