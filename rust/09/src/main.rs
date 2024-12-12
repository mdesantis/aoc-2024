const INPUT_CONTENTS: &str = include_str!("../../../inputs/09/input");

#[derive(Debug)]
enum BlockEntry {
    FileId(i64),
    FreeSpace,
}

fn char_to_i64(c: char) -> i64 {
    c.to_string().parse::<i64>().unwrap()
}

fn blocks(input_contents: &str) -> Vec<BlockEntry> {
    let input_contents = input_contents.trim();
    let mut chars = input_contents.chars().peekable();
    let mut blocks = vec![];
    let mut file_id = 0;

    while let Some(file_blocks_amount) = chars.next() {
        let file_blocks_amount = char_to_i64(file_blocks_amount);

        for _ in 0..file_blocks_amount {
            blocks.push(BlockEntry::FileId(file_id));
        }

        if let Some(_) = chars.peek() {
            let free_space_amount = char_to_i64(chars.next().unwrap());

            for _ in 0..free_space_amount {
                blocks.push(BlockEntry::FreeSpace)
            }
        } else {
            break;
        }

        file_id += 1;
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
