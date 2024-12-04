use regex::Regex;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/04/input");

enum SlantDirection {
    Clockwise,
    AntiClockwise,
}

fn rotate_clockwise(input: &str) -> String {
    let mut result = "".to_string();

    let lines = input.lines().collect::<Vec<_>>();
    let rows = lines.len();
    let cols = lines[0].len();
    let chars = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..cols {
        for j in (0..rows).rev() {
            let char = chars[j][i];

            result.push(char);
            if j == 0 {
                result.push('\n');
            }
        }
    }

    result
}

fn slant(input: &str, direction: SlantDirection) -> String {
    let rows = input.lines().collect::<Vec<_>>().len();

    input
        .lines()
        .enumerate()
        .map(|(i, slice)| {
            let (prefix, suffix) = match direction {
                SlantDirection::Clockwise => (" ".repeat(i), " ".repeat(rows - i - 1)),
                SlantDirection::AntiClockwise => (" ".repeat(rows - i - 1), " ".repeat(i)),
            };

            format!("{prefix}{slice}{suffix}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn slant_clockwise(input: &str) -> String {
    slant(input, SlantDirection::Clockwise)
}

fn slant_anticlockwise(input: &str) -> String {
    slant(input, SlantDirection::AntiClockwise)
}

fn find_xmas_matches(input: &str) -> usize {
    let regex = Regex::new(r"XMAS").unwrap();

    regex.find_iter(input).collect::<Vec<_>>().len()
}

fn find_samx_matches(input: &str) -> usize {
    let regex = Regex::new(r"SAMX").unwrap();

    regex.find_iter(input).collect::<Vec<_>>().len()
}

fn words_amount(input_contents: &str) -> i32 {
    let normal = input_contents;
    let rotated_clockwise = rotate_clockwise(normal);
    let slanted_clockwise_rotated_clockwise = rotate_clockwise(&slant_clockwise(normal));
    let slanted_anticlockwise_rotated_clockwise = rotate_clockwise(&slant_anticlockwise(normal));

    [
        find_xmas_matches(normal),
        find_xmas_matches(&rotated_clockwise),
        find_xmas_matches(&slanted_clockwise_rotated_clockwise),
        find_xmas_matches(&slanted_anticlockwise_rotated_clockwise),
        find_samx_matches(normal),
        find_samx_matches(&rotated_clockwise),
        find_samx_matches(&slanted_clockwise_rotated_clockwise),
        find_samx_matches(&slanted_anticlockwise_rotated_clockwise),
    ]
    .iter()
    .sum::<usize>() as i32
}

fn crosses_amount(input_contents: &str) -> i32 {
    let lines = input_contents.lines().collect::<Vec<_>>();
    let rows = lines.len();
    let cols = lines[0].len();
    let chars = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut amount = 0;

    for i in 1..(cols - 1) {
        for j in 1..(rows - 1) {
            if chars[i][j] == 'A' {
                let cross = match (
                    chars[i - 1][j - 1],
                    chars[i - 1][j + 1],
                    chars[i + 1][j - 1],
                    chars[i + 1][j + 1],
                ) {
                    ('M', 'M', 'S', 'S') => Some(()),
                    ('S', 'S', 'M', 'M') => Some(()),
                    ('M', 'S', 'M', 'S') => Some(()),
                    ('S', 'M', 'S', 'M') => Some(()),
                    _ => None,
                };

                if cross.is_some() {
                    amount += 1
                }
            }
        }
    }

    amount
}

fn main() {
    let result = words_amount(INPUT_CONTENTS);
    println!("Words amount: {result}");

    let result = crosses_amount(INPUT_CONTENTS);
    println!("Crosses amount: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
";
    const TEST_INPUT_CONTENTS_2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

    #[test]
    fn test_words_amount() {
        let expected = 18;
        let actual = words_amount(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_crosses_amount() {
        let expected = 9;
        let actual = crosses_amount(TEST_INPUT_CONTENTS_2);

        assert_eq!(expected, actual);
    }
}
