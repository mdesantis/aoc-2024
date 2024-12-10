# frozen_string_literal: true

require 'bundler/setup'
require 'minitest/autorun'
require 'minitest/pride'

require_relative 'file_blocks_compacting'

INPUT_CONTENTS = '2333133121414131402'

class TestFileBlocksCompacting < Minitest::Test
  def test_filesystem_checksum_after_file_blocks_compacting
    result = FileBlocksCompacting.new(INPUT_CONTENTS).filesystem_checksum_after_file_blocks_compacting

    assert_equal 1928, result
  end
end

class TestFileBlocksCompactingDealingWithFileSystemFragmentation < Minitest::Test
  def test_filesystem_checksum_after_file_blocks_compacting
    result = FileBlocksCompactingDealingWithFileSystemFragmentation
      .new(INPUT_CONTENTS)
      .filesystem_checksum_after_file_blocks_compacting

    assert_equal 2858, result
  end
end
