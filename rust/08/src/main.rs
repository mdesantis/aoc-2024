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

fn generate_antinodes(
    antennas_pair: ((i32, i32), (i32, i32)),
    rows: i32,
    cols: i32,
) -> Vec<(i32, i32)> {
    let ((x1, y1), (x2, y2)) = antennas_pair;
    let (dx, dy) = (2 * (x2 - x1), 2 * (y2 - y1));
    let antinodes = vec![(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)];
    let in_bound_antinodes = antinodes
        .iter()
        .filter(|antinode| is_antinode_in_bound(**antinode, rows, cols));

    in_bound_antinodes.copied().collect::<Vec<_>>()
}

fn generate_antinodes_along_all_the_line(
    antennas_pair: ((i32, i32), (i32, i32)),
    rows: i32,
    cols: i32,
) -> Vec<(i32, i32)> {
    let ((x1, y1), (x2, y2)) = antennas_pair;
    let in_bound_antinodes = (1..)
        .into_iter()
        .map_while(|i| {
            let (dx, dy) = (i * (x2 - x1), i * (y2 - y1));
            let antinodes = vec![(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)];
            let in_bound_antinodes = antinodes
                .iter()
                .filter(|antinode| is_antinode_in_bound(**antinode, rows, cols))
                .copied()
                .collect::<Vec<_>>();
            (!in_bound_antinodes.is_empty()).then_some(in_bound_antinodes)
        })
        .flatten();

    in_bound_antinodes.collect::<Vec<_>>()
}

fn antennas_pair_to_antinodes(
    antennas_pair: ((i32, i32), (i32, i32)),
    rows: i32,
    cols: i32,
    antinodes_along_all_the_line: bool,
) -> Option<Vec<(i32, i32)>> {
    let antinodes = match antinodes_along_all_the_line {
        false => generate_antinodes(antennas_pair, rows, cols),
        true => generate_antinodes_along_all_the_line(antennas_pair, rows, cols),
    };

    (!antinodes.is_empty()).then_some(antinodes)
}

fn antinodes(
    antennas: &[(i32, i32, char)],
    rows: i32,
    cols: i32,
    antinodes_along_all_the_line: bool,
) -> HashSet<(i32, i32)> {
    antennas
        .iter()
        .flat_map(move |antenna1| {
            antennas
                .iter()
                .filter_map(move |antenna2| match (antenna1, antenna2) {
                    ((x1, y1, c1), (x2, y2, c2)) if c1 == c2 && (x1, y1) < (x2, y2) => {
                        antennas_pair_to_antinodes(
                            ((*x1, *y1), (*x2, *y2)),
                            rows,
                            cols,
                            antinodes_along_all_the_line,
                        )
                    }
                    _ => None,
                })
        })
        .flatten()
        .collect()
}

fn antinodes_amount(input_contents: &str, antinodes_along_all_the_line: bool) -> i32 {
    let lines = input_contents.lines().collect::<Vec<_>>();
    let cols = lines.len() as i32;
    let rows = lines[0].chars().collect::<Vec<_>>().len() as i32;
    let antennas = antennas(&lines);
    let antinodes = antinodes(&antennas, rows, cols, antinodes_along_all_the_line);

    antinodes.len() as i32
}

fn main() {
    let result = antinodes_amount(INPUT_CONTENTS, false);
    println!("Antinodes amount: {result}");

    let result = antinodes_amount(INPUT_CONTENTS, true);
    println!("Antinodes along all the line amount: {result}");
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
    fn test_in_bound_antinodes_amount_part_one() {
        let expected = 14;
        let actual = antinodes_amount(TEST_INPUT_CONTENTS, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_in_bound_antinodes_amount_part_two() {
        let expected = 34;
        let actual = antinodes_amount(TEST_INPUT_CONTENTS, true);
        assert_eq!(expected, actual);
    }
}
