use std::collections::HashSet;

const INPUT_CONTENTS: &str = include_str!("../../../inputs/06/input");
const OBSTRUCTED_TILE: char = '#';

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
                            (curr_pos, curr_dir) = (Some((x, y)), Some(direction));
                        }
                    }

                    acc.push(char);
                    acc
                })
        })
        .collect::<Vec<_>>();

    (map, curr_pos.unwrap(), curr_dir.unwrap())
}

fn next_dir(curr_dir: GuardDirection) -> GuardDirection {
    match curr_dir {
        GuardDirection::Top => GuardDirection::Right,
        GuardDirection::Right => GuardDirection::Down,
        GuardDirection::Down => GuardDirection::Left,
        GuardDirection::Left => GuardDirection::Top,
    }
}

fn is_obstructed(tile: char) -> bool {
    tile == OBSTRUCTED_TILE
}

fn maybe_next_values(
    map: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    curr_pos: (usize, usize),
    curr_dir: GuardDirection,
) -> Option<((usize, usize), GuardDirection)> {
    let (curr_x, curr_y) = curr_pos;
    let (maybe_next_x, maybe_next_y) = match curr_dir {
        GuardDirection::Top if curr_x > 0 => (Some(curr_x - 1), Some(curr_y)),
        GuardDirection::Right => (Some(curr_x), Some(curr_y + 1)),
        GuardDirection::Down => (Some(curr_x + 1), Some(curr_y)),
        GuardDirection::Left if curr_y > 0 => (Some(curr_x), Some(curr_y - 1)),
        _ => (None, None),
    };

    match (maybe_next_x, maybe_next_y) {
        (None, _) => None,
        (_, None) => None,
        (Some(next_x), Some(next_y)) if next_x >= rows || next_y >= cols => None,
        (Some(next_x), Some(next_y)) if is_obstructed(map[next_x][next_y]) => {
            maybe_next_values(map, rows, cols, curr_pos, next_dir(curr_dir))
        }
        (Some(next_x), Some(next_y)) => Some(((next_x, next_y), curr_dir)),
    }
}

fn visited_positions(
    map: &Vec<Vec<char>>,
    mut curr_pos: (usize, usize),
    mut curr_dir: GuardDirection,
) -> HashSet<(usize, usize)> {
    let mut visited_positions = HashSet::new();
    let rows = map.len();
    let cols = map[0].len();

    visited_positions.insert(curr_pos);

    while let Some((next_pos, next_dir)) = maybe_next_values(&map, rows, cols, curr_pos, curr_dir) {
        (curr_pos, curr_dir) = (next_pos, next_dir);

        visited_positions.insert(curr_pos);
    }

    visited_positions
}

fn visited_positions_amount(visited_positions: &HashSet<(usize, usize)>) -> i32 {
    visited_positions.len() as i32
}

fn is_stuck_in_loop(
    map: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    mut curr_pos: (usize, usize),
    mut curr_dir: GuardDirection,
) -> bool {
    let mut visited_positions_and_direction = HashSet::new();

    while let Some((next_pos, next_dir)) = maybe_next_values(&map, rows, cols, curr_pos, curr_dir) {
        (curr_pos, curr_dir) = (next_pos, next_dir);

        if !visited_positions_and_direction.insert((curr_pos, curr_dir)) {
            return true;
        }
    }

    false
}

fn stuck_in_loop_amount(
    map: Vec<Vec<char>>,
    visited_positions: &HashSet<(usize, usize)>,
    curr_pos: (usize, usize),
    curr_dir: GuardDirection,
) -> i32 {
    let rows = map.len();
    let cols = map[0].len();
    let stuck_in_loop = visited_positions.iter().filter(|(x, y)| {
        let (x, y) = (*x, *y);

        if (x, y) == curr_pos {
            return false;
        }

        let mut map = map.clone();

        map[x][y] = OBSTRUCTED_TILE;
        is_stuck_in_loop(map, rows, cols, curr_pos, curr_dir)
    });

    stuck_in_loop.count() as i32
}

fn main() {
    let (map, curr_pos, curr_dir) = get_map_and_starting_values(INPUT_CONTENTS);
    let visited_positions = visited_positions(&map, curr_pos, curr_dir);

    let result = visited_positions_amount(&visited_positions);
    println!("Visited positions amount: {result}");

    let result = stuck_in_loop_amount(map, &visited_positions, curr_pos, curr_dir);
    println!("Stuck in loop amount: {result}");
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
        let (map, curr_pos, curr_dir) = get_map_and_starting_values(TEST_INPUT_CONTENTS);
        let visited_positions = visited_positions(&map, curr_pos, curr_dir);
        let actual = visited_positions_amount(&visited_positions);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_stuck_in_loop_amount() {
        let expected = 6;
        let (map, curr_pos, curr_dir) = get_map_and_starting_values(TEST_INPUT_CONTENTS);
        let visited_positions = visited_positions(&map, curr_pos, curr_dir);
        let actual = stuck_in_loop_amount(map, &visited_positions, curr_pos, curr_dir);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_stuck_in_loop_amount_with_real_input() {
        let expected = 1719;
        let (map, curr_pos, curr_dir) = get_map_and_starting_values(INPUT_CONTENTS);
        let visited_positions = visited_positions(&map, curr_pos, curr_dir);
        let actual = stuck_in_loop_amount(map, &visited_positions, curr_pos, curr_dir);

        assert_eq!(expected, actual);
    }
}
