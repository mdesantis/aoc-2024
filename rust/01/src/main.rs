fn part_1() {
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

    println!("distances sum: {distances_sum:?}");
}

fn part_2() {
    let input_contents = include_str!("../../../inputs/01/input");

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

    println!("similarity score: {similarity_score:?}");
}

fn main() {
    part_1();
    part_2();
}
