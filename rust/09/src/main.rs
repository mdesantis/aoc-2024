#![feature(iter_next_chunk)]

const INPUT_CONTENTS: &str = include_str!("../../../inputs/09/input");

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlockType {
    FileId,
    FreeSpace,
}

fn char_to_i64(c: char) -> i64 {
    c.to_string().parse::<i64>().unwrap()
}

fn blocks(input_contents: &str) -> Vec<(BlockType, Option<i64>)> {
    let input_contents = input_contents.trim();
    let mut chars = input_contents.chars();
    let mut blocks = vec![];
    let mut file_id = 0;

    loop {
        let (file_blocks_amount, maybe_free_space_amount) = match chars.next_chunk() {
            Ok([fia, fsa]) => (char_to_i64(fia), Some(char_to_i64(fsa))),
            Err(rem) => (char_to_i64(rem.as_slice()[0]), None),
        };

        for _ in 0..file_blocks_amount {
            blocks.push((BlockType::FileId, Some(file_id)));
        }

        match maybe_free_space_amount {
            Some(free_space_amount) => {
                for _ in 0..free_space_amount {
                    blocks.push((BlockType::FreeSpace, None))
                }
            }
            _ => break,
        }

        file_id += 1;
    }

    blocks
}

fn compact_file_blocks(blocks: &mut Vec<(BlockType, Option<i64>)>) {
    for i in 0..blocks.len() {
        let maybe_v = blocks.get(i);

        if maybe_v.is_none() {
            break;
        }

        let v = *maybe_v.unwrap();

        if v.0 == BlockType::FileId {
            continue;
        }

        loop {
            let back = blocks.pop().unwrap();

            if back.0 == BlockType::FreeSpace {
                continue;
            }

            if blocks.get(i).is_some() {
                blocks[i] = back;
            } else {
                blocks.push(back);
            }

            break;
        }
    }
}

fn filesystem_checksum_after_file_blocks_compacting(input_contents: &str) -> i64 {
    let mut blocks = blocks(input_contents);

    compact_file_blocks(&mut blocks);
    blocks
        .iter()
        .enumerate()
        .map(|(i, (_, v))| (i as i64) * v.unwrap())
        .sum::<i64>()
}

fn main() {
    let result = filesystem_checksum_after_file_blocks_compacting(INPUT_CONTENTS);
    println!("Filesystem checksum after file blocks compacting: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "2333133121414131402";

    #[test]
    fn test_filesystem_checksum() {
        let expected = 1928;
        let actual = filesystem_checksum_after_file_blocks_compacting(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }
}
