# frozen_string_literal: true

require 'bundler/setup'
require_relative 'file_blocks_compacting'

INPUT_CONTENTS = File.read File.join(__dir__, '../../inputs/09/input')

result = FileBlocksCompacting.new(INPUT_CONTENTS).filesystem_checksum_after_file_blocks_compacting
puts "Filesystem checksum after file blocks compacting: #{result}"

result = FileBlocksCompactingDealingWithFileSystemFragmentation
  .new(INPUT_CONTENTS)
  .filesystem_checksum_after_file_blocks_compacting
puts "Filesystem checksum after file blocks compacting dealing with filesystem fragmentation: #{result}"
