#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"
require "fileutils"

ROOT = File.expand_path("../../../../..", __dir__)
TOOL = File.expand_path("registry_tool.rb", __dir__)
FIXTURE = File.expand_path("fixtures/minimal-registry.yaml", __dir__)

def run_tool(*args)
  cmd = ["ruby", TOOL, *args]
  Open3.capture3(*cmd)
end

failures = []

def assert(cond, msg, failures)
  if cond
    puts "PASS: #{msg}"
  else
    puts "FAIL: #{msg}"
    failures << msg
  end
end

# validate ok
stdout, stderr, status = run_tool("validate", FIXTURE)
assert(status.success?, "validate exits 0 on good fixture", failures)
assert(stdout.include?("OK"), "validate prints OK", failures)

# status lists buckets
stdout, stderr, status = run_tool("status", FIXTURE)
assert(status.success?, "status exits 0", failures)
assert(stdout.include?("done"), "status mentions done", failures)
assert(stdout.include?("ready") || stdout.include?("suite-ready"), "status mentions ready suite", failures)

# next returns highest-scoring ready suite
stdout, stderr, status = run_tool("next", FIXTURE, "1")
assert(status.success?, "next exits 0", failures)
assert(stdout.include?("suite-ready"), "next picks suite-ready", failures)
assert(!stdout.include?("suite-blocked-dep"), "next skips missing dep", failures)
assert(!stdout.include?("suite-explicit-blocked"), "next skips blocked status", failures)

# next score: suite-ready score = 900+10+5-0 = 915
stdout, stderr, status = run_tool("score", FIXTURE, "suite-ready")
assert(status.success?, "score exits 0", failures)
assert(stdout.strip.end_with?("915") || stdout.include?("915"), "score is 915 for suite-ready", failures)

# validate fails on bad status
bad = File.expand_path("fixtures/bad-status.yaml", __dir__)
File.write(bad, File.read(FIXTURE).sub("status: pending\n    blocked_reason: null\n    paths:\n      commands: src/commands/moderation/", "status: nope\n    blocked_reason: null\n    paths:\n      commands: src/commands/moderation/"))
stdout, stderr, status = run_tool("validate", bad)
assert(!status.success?, "validate fails on bad status", failures)
FileUtils.rm_f(bad)

if failures.empty?
  puts "\nAll tests passed."
  exit 0
else
  puts "\n#{failures.length} failure(s)."
  exit 1
end
