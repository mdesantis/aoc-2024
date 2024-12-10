# frozen_string_literal: true

class FileBlocksCompacting
  FileBlock = Data.define :file_id
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

    blocks_chunks = blocks
      .each_with_index
      .select { |(fbs, _)| fbs.is_a? FileBlock }
      .reverse_each
      .chunk_while do |(elt_before, _), (elt_after, _)|
      elt_before.is_a?(FileBlock) && elt_after.is_a?(FileBlock) && elt_before.file_id == elt_after.file_id
    end

    blocks_chunks.each do |fbs_with_blocks_i|
      fss_with_blocks_i =
        blocks
          .each_with_index
          .each_cons(fbs_with_blocks_i.size)
          .find { |v| v.all? { |(fbs, _)| fbs.is_a? FreeSpace } }
          &.first

      next unless fss_with_blocks_i

      first_fbs_i = fbs_with_blocks_i.last.last
      fss_with_blocks_i => _, first_fs_i

      next if first_fbs_i < first_fs_i

      fbs_with_blocks_i.size.times do |i|
        blocks[first_fs_i + i] = FileBlock[file_id: fbs_with_blocks_i.first.first.file_id]
        blocks[fbs_with_blocks_i.last.last + i] = FreeSpace[]
      end
    end

    blocks
  end
end
