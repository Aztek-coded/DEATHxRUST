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

# record-merge: dry-run does not change file content or mtime
tmp = File.expand_path("fixtures/tmp-record-merge.yaml", __dir__)
FileUtils.cp(FIXTURE, tmp)
before_content = File.read(tmp)
mtime_before = File.mtime(tmp)
sleep 0.05
stdout, stderr, status = run_tool("record-merge", tmp, "suite-ready")
assert(status.success?, "record-merge dry-run exits 0", failures)
assert(stdout.include?("status: done"), "record-merge dry-run prints status done", failures)
assert(File.read(tmp) == before_content, "record-merge dry-run does not change file content", failures)
assert(File.mtime(tmp) == mtime_before, "record-merge dry-run does not change mtime", failures)

# record-merge --write sets status done; preserves other suite text and comments
comment_line = "  # keep-me-comment\n"
seed = File.read(FIXTURE).sub(
  "  - id: suite-ready\n",
  "#{comment_line}  - id: suite-ready\n"
)
File.write(tmp, seed)
stdout, stderr, status = run_tool("record-merge", tmp, "suite-ready", "--write")
assert(status.success?, "record-merge --write exits 0", failures)
assert(stdout.include?("Wrote"), "record-merge --write reports write", failures)
after = File.read(tmp)
assert(after.include?("keep-me-comment"), "record-merge --write preserves comments", failures)
# suite-ready block has status done
ready_idx = after.index("- id: suite-ready")
blocked_idx = after.index("- id: suite-blocked-dep")
ready_block = after[ready_idx...blocked_idx]
assert(ready_block.match?(/^\s*status:\s*done\s*$/), "record-merge --write sets status done", failures)
assert(ready_block.match?(/^\s*blocked_reason:\s*null\s*$/), "record-merge --write clears blocked_reason", failures)
# other suites unchanged status
assert(after.include?("id: foundation-a") && after.match?(/id: foundation-a[\s\S]*?status:\s*done/), "other done suite unchanged", failures)
assert(after.include?("status: blocked"), "blocked suite status preserved", failures)
# re-validate after write
stdout, stderr, status = run_tool("validate", tmp)
assert(status.success?, "registry still valid after record-merge --write", failures)
FileUtils.rm_f(tmp)

if failures.empty?
  puts "\nAll tests passed."
  exit 0
else
  puts "\n#{failures.length} failure(s)."
  exit 1
end
