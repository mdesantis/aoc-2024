fn main() {
    let input_contents = include_str!("../../../inputs/01/input");
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

    println!("distances_sum: {distances_sum:?}");
}
