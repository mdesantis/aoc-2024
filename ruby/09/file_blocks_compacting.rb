# frozen_string_literal: true

class FileBlocksCompacting
  FileBlock = Data.define(:file_id) do
    def same_file_id?(other)
      file_id == other.file_id
    end
  end
  FreeSpace = Data.define

  attr_reader :input_contents

  def initialize(input_contents)
    @input_contents = input_contents.chomp
  end

  def filesystem_checksum_after_file_blocks_compacting
    compact_file_blocks(blocks).each_with_index.select { |(v, _)| v.is_a? FileBlock }.sum { |(v, i)| i * v.file_id }
  end

  private

  def blocks
    input_contents
      .each_char
      .each_slice(2)
      .with_index
      .with_object([]) do |((file_blocks_amount, free_space_amount), i), blocks|
      file_blocks_amount = file_blocks_amount.to_i
      free_space_amount = free_space_amount&.to_i

      file_blocks_amount.times { blocks << FileBlock[file_id: i] }
      free_space_amount&.times { blocks << FreeSpace[] }
    end
  end

  def compact_file_blocks(blocks)
    blocks.size.times do |i|
      next if blocks[i].is_a? FileBlock

      loop do
        back = blocks.pop

        next if back.is_a? FreeSpace

        if blocks[i]
          blocks[i] = back
        else
          blocks.push back
        end

        break
      end
    end

    blocks
  end
end

class FileBlocksCompactingDealingWithFileSystemFragmentation < FileBlocksCompacting
  private

  def compact_file_blocks(blocks)
    blocks = blocks.to_a

    file_blocks_with_the_same_file_id_chunks = chunk_by_file_block_with_same_file_id blocks

    file_blocks_with_the_same_file_id_chunks.each do |file_blocks_with_blocks_i|
      free_spaces_with_blocks_i = first_free_spaces_suitable_for_compacting blocks, file_blocks_with_blocks_i.size

      next unless free_spaces_with_blocks_i

      first_file_block_blocks_i = file_blocks_with_blocks_i.last.last
      free_spaces_with_blocks_i => _, first_free_space_blocks_i

      next if first_file_block_blocks_i < first_free_space_blocks_i

      swap_file_blocks_with_free_spaces blocks, file_blocks_with_blocks_i, first_free_space_blocks_i
    end

    blocks
  end

  def chunk_by_file_block_with_same_file_id(blocks)
    blocks
      .each_with_index
      .select { |(block, _)| block.is_a? FileBlock }
      .reverse_each
      .chunk_while { |(file_block_before, _), (file_block_after, _)| file_block_before.same_file_id? file_block_after }
  end

  def first_free_spaces_suitable_for_compacting(blocks, file_blocks_with_blocks_i_size)
    blocks
      .each_with_index
      .each_cons(file_blocks_with_blocks_i_size)
      .find { |file_blocks| file_blocks.all? { |(file_block, _)| file_block.is_a? FreeSpace } }
      &.first
  end

  def swap_file_blocks_with_free_spaces(blocks, file_blocks_with_blocks_i, first_free_space_blocks_i)
    file_blocks_with_blocks_i.size.times do |i|
      blocks[first_free_space_blocks_i + i] = FileBlock[file_id: file_blocks_with_blocks_i.first.first.file_id]
      blocks[file_blocks_with_blocks_i.last.last + i] = FreeSpace[]
    end
  end
end
