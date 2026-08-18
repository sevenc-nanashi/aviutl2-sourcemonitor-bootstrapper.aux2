# frozen_string_literal: true
require "English"

version = ARGV[0]

aviutl2_toml = File.read("./aviutl2.toml", mode: "rb")
aviutl2_toml.gsub!(/\/v([0-9]+\.[0-9]+\.[0-9]+)/, "\/v#{version}") or raise "Unreachable: No replacement"
old_version = Regexp.last_match[1]
aviutl2_toml.gsub!(/version = "[0-9]+\.[0-9]+\.[0-9]+(\.[0-9]+)"/, "version = \"#{version}\\1\"") or raise "Unreachable: No replacement"

File.write("aviutl2.toml", aviutl2_toml, mode: "wb")

puts "#{old_version} -> #{version}"
