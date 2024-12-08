use std::collections::HashSet;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/08/input");

fn is_antinode_in_bound(antinode: (i32, i32), rows: i32, cols: i32) -> bool {
    let (x, y) = antinode;

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

fn antennas_pair_to_in_bound_antinodes(
    antenna1: (i32, i32),
    antenna2: (i32, i32),
    rows: i32,
    cols: i32,
) -> Option<Vec<(i32, i32)>> {
    let ((x1, y1), (x2, y2)) = (antenna1, antenna2);
    let (dx, dy) = (2 * (x2 - x1), 2 * (y2 - y1));
    let antinodes = vec![(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)];
    let in_bound_antinodes = antinodes
        .iter()
        .filter(|antinode| is_antinode_in_bound(**antinode, rows, cols))
        .copied()
        .collect::<Vec<_>>();

    (!in_bound_antinodes.is_empty()).then_some(in_bound_antinodes)
}

fn in_bound_antinodes(antennas: &[(i32, i32, char)], rows: i32, cols: i32) -> HashSet<(i32, i32)> {
    antennas
        .iter()
        .flat_map(move |antenna1| {
            antennas.iter().filter_map(move |antenna2| {
                // antennas_pair_to_in_bound_antinodes(*antenna1, *antenna2, rows, cols)
                match (antenna1, antenna2) {
                    ((x1, y1, c1), (x2, y2, c2)) if c1 == c2 && (x1, y1) < (x2, y2) => {
                        antennas_pair_to_in_bound_antinodes((*x1, *y1), (*x2, *y2), rows, cols)
                    }
                    _ => None,
                }
            })
        })
        .flatten()
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
