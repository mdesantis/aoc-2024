#![feature(let_chains)]

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
        match blocks.get(i) {
            None => break,
            Some(block_entry) if matches!(block_entry, BlockEntry::FreeSpace) => {
                while let Some(last_block_entry) = blocks.pop() {
                    if let BlockEntry::FileId(_) = last_block_entry {
                        if let Some(_) = blocks.get(i) {
                            blocks[i] = last_block_entry;
                        } else {
                            blocks.push(last_block_entry);
                        }

                        break;
                    }
                }
            }
            _ => (),
        };
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

#[derive(Debug)]
struct File {
    start_index: usize,
    size: usize,
}

fn files(blocks: &mut Vec<BlockEntry>) -> Vec<File> {
    let mut iter = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, v)| match v {
            BlockEntry::FileId(file_id) => Some((i, *file_id)),
            _ => None,
        })
        .rev()
        .peekable();
    let mut files = vec![];
    let mut size = 1;

    while let Some((i, file_id)) = iter.next() {
        match iter.peek() {
            Some((_, peek_file_id)) if file_id == *peek_file_id => size += 1,
            _ => {
                files.push(File {
                    size,
                    start_index: i,
                });
                size = 1;
            }
        }
    }

    files
}

fn free_spaces_suitable_for_compacting_start_index(
    blocks: &mut Vec<BlockEntry>,
    file: &File,
) -> Option<usize> {
    let size = file.size;
    let mut count = 0;
    let mut start_index = None;

    for (i, v) in blocks.iter().enumerate() {
        if let BlockEntry::FreeSpace = v {
            if start_index.is_none() {
                start_index = Some(i);
            }
            count += 1;

            if count == size {
                return start_index;
            }
        } else {
            count = 0;
            start_index = None;
        }
    }

    None
}

fn compact_files(blocks: &mut Vec<BlockEntry>) {
    let files = files(blocks);

    for file in files {
        let maybe_free_space_start_index =
            free_spaces_suitable_for_compacting_start_index(blocks, &file);

        if let Some(free_space_start_index) = maybe_free_space_start_index
            && file.start_index > free_space_start_index
        {
            swap_file_blocks_with_free_spaces(blocks, &file, free_space_start_index);
        }
    }
}

fn swap_file_blocks_with_free_spaces(blocks: &mut Vec<BlockEntry>, file: &File, fsi: usize) {
    for (file_block_index, free_space_index) in
        (file.start_index..(file.start_index + file.size)).zip(fsi..(fsi + file.size))
    {
        blocks.swap(file_block_index, free_space_index);
    }
}

fn filesystem_checksum_after_files_compacting(input_contents: &str) -> i64 {
    let mut blocks = blocks(input_contents);

    compact_files(&mut blocks);
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, v)| match v {
            BlockEntry::FileId(id) => Some((i as i64) * id),
            _ => None,
        })
        .sum::<i64>()
}

fn main() {
    let result = filesystem_checksum_after_file_blocks_compacting(INPUT_CONTENTS);
    println!("Filesystem checksum after file blocks compacting: {result}");

    let result = filesystem_checksum_after_files_compacting(INPUT_CONTENTS);
    println!("Filesystem checksum after files compacting: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_CONTENTS: &str = "2333133121414131402";

    #[test]
    fn test_filesystem_checksum_after_file_blocks_compacting() {
        let expected = 1928;
        let actual = filesystem_checksum_after_file_blocks_compacting(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_filesystem_checksum_after_files_compacting() {
        let expected = 2858;
        let actual = filesystem_checksum_after_files_compacting(TEST_INPUT_CONTENTS);
        assert_eq!(expected, actual);
    }
}
