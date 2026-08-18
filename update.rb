# frozen_string_literal: true
require "English"

version = ARGV[0]
if !version.match?(/\A[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+?\z/)
  raise "Invalid version format: #{version}"
end

sourcemonitor_version = version.match(/\A([0-9]+\.[0-9]+\.[0-9]+)\.[0-9]+?\z/)[1]

aviutl2_toml = File.read("./aviutl2.toml", mode: "rb")
aviutl2_toml.gsub!(/\/v([0-9]+\.[0-9]+\.[0-9]+)/, "\/v#{sourcemonitor_version}") or raise "Unreachable: No replacement"
old_version = Regexp.last_match[1]
aviutl2_toml.gsub!(/version = "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+"/, "version = \"#{version}\"") or raise "Unreachable: No replacement"

File.write("aviutl2.toml", aviutl2_toml, mode: "wb")

puts "#{old_version} -> #{version}"
