#![feature(test)]

extern crate test;

use std::collections::HashSet;

type Tile = char;
type Map = Vec<Vec<Tile>>;
type Coord = usize;
type Position = (Coord, Coord);
type VisitedPositions = HashSet<Position>;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Top,
    Right,
    Down,
    Left,
}

const INPUT_CONTENTS: &str = include_str!("../../../inputs/06/input");
const OBSTRUCTED_TILE: Tile = '#';

impl TryFrom<Tile> for Direction {
    type Error = ();

    fn try_from(tile: Tile) -> Result<Self, Self::Error> {
        match tile {
            '^' => Ok(Self::Top),
            _ => Err(()),
        }
    }
}

fn get_map_and_starting_values(input_contents: &str) -> (Map, Position, Direction) {
    let mut curr_pos: Option<Position> = None;
    let mut curr_dir: Option<Direction> = None;
    let map = input_contents
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, char)| {
                    let tile: Tile = char;

                    if curr_pos.is_none() {
                        if let Ok(direction) = Direction::try_from(tile) {
                            (curr_pos, curr_dir) = (Some((x, y)), Some(direction));
                        }
                    }

                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (map, curr_pos.unwrap(), curr_dir.unwrap())
}

fn next_dir(curr_dir: Direction) -> Direction {
    match curr_dir {
        Direction::Top => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Top,
    }
}

fn is_obstructed(tile: Tile) -> bool {
    tile == OBSTRUCTED_TILE
}

fn maybe_next_values(
    map: &Map,
    rows: Coord,
    cols: Coord,
    curr_pos: Position,
    curr_dir: Direction,
) -> Option<(Position, Direction)> {
    let (curr_x, curr_y) = curr_pos;
    let (maybe_next_x, maybe_next_y) = match curr_dir {
        Direction::Top if curr_x > 0 => (Some(curr_x - 1), Some(curr_y)),
        Direction::Right => (Some(curr_x), Some(curr_y + 1)),
        Direction::Down => (Some(curr_x + 1), Some(curr_y)),
        Direction::Left if curr_y > 0 => (Some(curr_x), Some(curr_y - 1)),
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
    map: &Map,
    mut curr_pos: Position,
    mut curr_dir: Direction,
) -> VisitedPositions {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited_positions = VisitedPositions::with_capacity(rows * cols);

    visited_positions.insert(curr_pos);

    while let Some((next_pos, next_dir)) = maybe_next_values(&map, rows, cols, curr_pos, curr_dir) {
        (curr_pos, curr_dir) = (next_pos, next_dir);

        visited_positions.insert(curr_pos);
    }

    visited_positions
}

fn visited_positions_amount(visited_positions: &VisitedPositions) -> i32 {
    visited_positions.len() as i32
}

fn is_stuck_in_loop(
    map: Map,
    rows: usize,
    cols: usize,
    mut curr_pos: Position,
    mut curr_dir: Direction,
) -> bool {
    let mut visited_positions_and_direction: HashSet<(Position, Direction)> =
        HashSet::with_capacity(rows * cols);

    while let Some((next_pos, next_dir)) = maybe_next_values(&map, rows, cols, curr_pos, curr_dir) {
        (curr_pos, curr_dir) = (next_pos, next_dir);

        if !visited_positions_and_direction.insert((curr_pos, curr_dir)) {
            return true;
        }
    }

    false
}

fn stuck_in_loop_amount(
    map: Map,
    visited_positions: &VisitedPositions,
    curr_pos: Position,
    curr_dir: Direction,
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
    use std::hint::black_box;
    use test::Bencher;

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
    const BENCH_INPUT_CONTENTS: &str = INPUT_CONTENTS;

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

    #[bench]
    fn bench_stuck_in_loop_amount(bencher: &mut Bencher) {
        bencher.iter(|| {
            let (map, curr_pos, curr_dir) =
                black_box(get_map_and_starting_values(BENCH_INPUT_CONTENTS));
            let visited_positions = black_box(visited_positions(&map, curr_pos, curr_dir));

            black_box(stuck_in_loop_amount(
                map,
                &visited_positions,
                curr_pos,
                curr_dir,
            ))
        });
    }
}
