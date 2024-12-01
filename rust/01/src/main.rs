fn distances_sum(input_contents: &str) -> i32 {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    let mut distances_sum = 0;

    for line in input_contents.lines() {
        for (i, slice) in line.split_whitespace().enumerate() {
            let value = slice.parse::<i32>().unwrap();

            match i % 2 {
                0 => lefts.push(value),
                _ => rights.push(value),
            }
        }
    }

    lefts.sort();
    rights.sort();

    for (i, left) in lefts.iter().enumerate() {
        let right = rights[i];
        let distance = (left - right).abs();
        distances_sum += distance;
    }

    distances_sum
}

fn similarity_score(input_contents: &str) -> i32 {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    let mut similarity_score = 0;

    for line in input_contents.lines() {
        for (i, slice) in line.split_whitespace().enumerate() {
            let value = slice.parse::<i32>().unwrap();

            match i % 2 {
                0 => lefts.push(value),
                _ => rights.push(value),
            }
        }
    }

    for left in lefts.iter() {
        let count = rights.iter().filter(|&right| right == left).count();

        similarity_score += left * (count as i32);
    }

    similarity_score
}

fn main() {
    let input_contents = include_str!("../../../inputs/01/input");

    let distances_sum = distances_sum(input_contents);
    println!("Distances sum: {distances_sum}");

    let similarity_score = similarity_score(input_contents);
    println!("Similarity score: {similarity_score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    ";

    #[test]
    fn test_distances_sum() {
        let expected = 11;
        let actual = distances_sum(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_similarity_score() {
        let expected = 31;
        let actual = similarity_score(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }
}
