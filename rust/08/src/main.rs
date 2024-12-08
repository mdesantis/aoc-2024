use std::collections::HashSet;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/08/input");

fn is_antinode_in_bound(x: i32, y: i32, rows: i32, cols: i32) -> bool {
    0 <= x && x < rows && 0 <= y && y < cols
}

fn antennas(lines: &[&str]) -> Vec<(i32, i32, char)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, char)| {
                char.is_alphanumeric().then(|| (i as i32, j as i32, char))
            })
        })
        .collect::<Vec<_>>()
}

fn antinode_pairs(
    x1: &i32,
    y1: &i32,
    c1: &char,
    x2: &i32,
    y2: &i32,
    c2: &char,
) -> Option<Vec<(i32, i32)>> {
    if c1 == c2 && (x1, y1) < (x2, y2) {
        let dx = 2 * (x2 - x1);
        let dy = 2 * (y2 - y1);

        return Some(vec![(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)]);
    }

    None
}

fn in_bound_antinodes(antennas: &[(i32, i32, char)], rows: i32, cols: i32) -> HashSet<(i32, i32)> {
    antennas
        .iter()
        .flat_map(move |(x1, y1, c1)| {
            antennas
                .iter()
                .filter_map(|(x2, y2, c2)| antinode_pairs(x1, y1, c1, x2, y2, c2))
        })
        .flatten()
        .filter(|(x, y)| is_antinode_in_bound(*x, *y, rows, cols))
        .collect()
}

fn in_bound_antinodes_amount(input_contents: &str) -> i32 {
    let lines = input_contents.lines().collect::<Vec<_>>();
    let cols = lines.len() as i32;
    let rows = lines[0].chars().collect::<Vec<_>>().len() as i32;
    let antennas = antennas(&lines);
    let in_bound_antinodes = in_bound_antinodes(&antennas, rows, cols);

    in_bound_antinodes.len() as i32
}

fn main() {
    let result = in_bound_antinodes_amount(INPUT_CONTENTS);
    println!("In bound antinodes amount: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_in_bound_antinodes_amount() {
        let expected = 14;
        let actual = in_bound_antinodes_amount(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }
}
