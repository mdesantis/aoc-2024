const INPUT_CONTENTS: &str = include_str!("../../../inputs/09/input");

#[derive(Debug)]
enum BlockEntry {
    FileId(i64),
    FreeSpace,
}

fn blocks(input_contents: &str) -> Vec<BlockEntry> {
    let mut blocks = vec![];
    let mut chars = input_contents.trim().char_indices().peekable();

    while let Some((i, file_blocks_amount_char)) = chars.next() {
        let file_id = (i / 2) as i64;

        for _ in 0..file_blocks_amount_char.to_digit(10).unwrap() {
            blocks.push(BlockEntry::FileId(file_id));
        }

        if let Some((_, free_space_amount_char)) = chars.peek() {
            for _ in 0..free_space_amount_char.to_digit(10).unwrap() {
                blocks.push(BlockEntry::FreeSpace)
            }

            chars.next();
        } else {
            break;
        }
    }

    blocks
}

fn compact_file_blocks(blocks: &mut Vec<BlockEntry>) {
    for i in 0..blocks.len() {
        let block_entry = match blocks.get(i) {
            Some(block_entry) => block_entry,
            _ => break,
        };

        if let BlockEntry::FreeSpace = block_entry {
            while let Some(last_block_entry) = blocks.pop() {
                if let BlockEntry::FileId(_) = last_block_entry {
                    if blocks.get(i).is_some() {
                        blocks[i] = last_block_entry;
                    } else {
                        blocks.push(last_block_entry);
                    }

                    break;
                }
            }
        }
    }
}

fn filesystem_checksum_after_file_blocks_compacting(input_contents: &str) -> i64 {
    let mut blocks = blocks(input_contents);

    compact_file_blocks(&mut blocks);
    blocks
        .iter()
        .enumerate()
        .map(|(i, v)| match v {
            BlockEntry::FileId(id) => (i as i64) * id,
            _ => unreachable!(),
        })
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
