use std::collections::HashSet;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/06/input");

#[derive(Copy, Clone, Debug)]
enum GuardDirection {
    Top,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for GuardDirection {
    type Error = ();

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '^' => Ok(Self::Top),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

fn get_map_and_starting_values(
    input_contents: &str,
) -> (Vec<Vec<char>>, (usize, usize), GuardDirection) {
    let mut curr_pos: Option<(usize, usize)> = None;
    let mut curr_dir: Option<GuardDirection> = None;

    let map = input_contents
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .fold(Vec::new(), |mut acc, (y, char)| {
                    if curr_pos.is_none() {
                        if let Ok(direction) = GuardDirection::try_from(char) {
                            curr_pos = Some((x, y));
                            curr_dir = Some(direction);
                        }
                    }

                    acc.push(char);
                    acc
                })
        })
        .collect::<Vec<_>>();

    (map, curr_pos.unwrap(), curr_dir.unwrap())
}

fn next_dir(curr_dir: &GuardDirection) -> GuardDirection {
    match curr_dir {
        GuardDirection::Top => GuardDirection::Right,
        GuardDirection::Right => GuardDirection::Down,
        GuardDirection::Down => GuardDirection::Left,
        GuardDirection::Left => GuardDirection::Top,
    }
}

fn is_obstructed(tile: char) -> bool {
    tile == '#'
}

fn maybe_next_values(
    map: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    curr_pos: (usize, usize),
    curr_dir: &GuardDirection,
) -> Option<((usize, usize), GuardDirection)> {
    let (curr_x, curr_y) = curr_pos;
    let (maybe_next_x, maybe_next_y) = match curr_dir {
        GuardDirection::Top => (curr_x.checked_sub(1), Some(curr_y)),
        GuardDirection::Right => (Some(curr_x), curr_y.checked_add(1)),
        GuardDirection::Down => (curr_x.checked_add(1), Some(curr_y)),
        GuardDirection::Left => (Some(curr_x), curr_y.checked_sub(1)),
    };
    match (maybe_next_x, maybe_next_y) {
        (None, _) => None,
        (_, None) => None,
        (Some(next_x), Some(next_y)) if next_x >= rows || next_y >= cols => None,
        (Some(next_x), Some(next_y)) if is_obstructed(map[next_x][next_y]) => {
            maybe_next_values(map, rows, cols, curr_pos, &next_dir(curr_dir))
        }
        (Some(next_x), Some(next_y)) => Some(((next_x, next_y), *curr_dir)),
    }
}

fn visited_positions_amount(input_contents: &str) -> i32 {
    let (map, mut curr_pos, mut curr_dir) = get_map_and_starting_values(input_contents);
    let mut visited_positions = HashSet::new();
    let rows = map.len();
    let cols = map[0].len();

    visited_positions.insert(curr_pos);

    while let Some((next_pos, next_dir)) = maybe_next_values(&map, rows, cols, curr_pos, &curr_dir)
    {
        curr_pos = next_pos;
        curr_dir = next_dir;
        visited_positions.insert(curr_pos);
    }

    visited_positions.len() as i32
}

fn main() {
    let result = visited_positions_amount(INPUT_CONTENTS);
    println!("Visited positions amount: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_visited_positions_amount() {
        let expected = 41;
        let actual = visited_positions_amount(TEST_INPUT_CONTENTS);

        assert_eq!(expected, actual);
    }
}
