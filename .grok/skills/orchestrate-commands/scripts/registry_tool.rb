#!/usr/bin/env ruby
# frozen_string_literal: true

require "yaml"

ALLOWED_STATUS = %w[
  pending planning implementing testing reviewing pr_open done blocked
].freeze

def repo_root
  # scripts/ -> orchestrate-commands/ -> skills/ -> .grok/ -> repo
  File.expand_path("../../../..", __dir__)
end

def default_registry
  File.join(repo_root, ".grok/orchestration/suite-registry.yaml")
end

def load_registry(path)
  data = YAML.load_file(path)
  raise "registry root must be a Hash" unless data.is_a?(Hash)
  raise "missing suites" unless data["suites"].is_a?(Array)
  data
end

def suite_map(data)
  map = {}
  data["suites"].each do |s|
    raise "suite missing id" unless s["id"]
    raise "duplicate suite id: #{s['id']}" if map.key?(s["id"])
    map[s["id"]] = s
  end
  map
end

def score_of(s)
  s.fetch("priority", 0).to_i +
    s.fetch("unlock_bonus", 0).to_i +
    s.fetch("infra_reuse_bonus", 0).to_i -
    s.fetch("blocked_penalty", 0).to_i
end

def deps_satisfied?(suite, map)
  deps = suite["depends_on"] || []
  deps.all? do |dep|
    map.key?(dep) && map[dep]["status"] == "done"
  end
end

def missing_deps(suite, map)
  (suite["depends_on"] || []).reject { |d| map.key?(d) }
end

def validate!(data, path)
  errors = []
  errors << "version must be 1" unless data["version"] == 1
  cap = data["concurrency_cap"]
  errors << "concurrency_cap must be positive int" unless cap.is_a?(Integer) && cap > 0

  map = {}
  data["suites"].each_with_index do |s, i|
    id = s["id"]
    if id.nil? || id.to_s.strip.empty?
      errors << "suites[#{i}] missing id"
      next
    end
    errors << "duplicate id #{id}" if map.key?(id)
    map[id] = s

    st = s["status"]
    errors << "#{id}: invalid status #{st.inspect}" unless ALLOWED_STATUS.include?(st)

    %w[priority unlock_bonus infra_reuse_bonus blocked_penalty].each do |k|
      v = s[k]
      errors << "#{id}: #{k} must be int" unless v.is_a?(Integer)
    end

    errors << "#{id}: commands must be array" unless s["commands"].is_a?(Array)
    errors << "#{id}: depends_on must be array" unless s["depends_on"].is_a?(Array)
    errors << "#{id}: paths must be hash" unless s["paths"].is_a?(Hash)

    if st == "blocked" && (s["blocked_reason"].nil? || s["blocked_reason"].to_s.strip.empty?)
      errors << "#{id}: blocked_reason required when status=blocked"
    end
  end

  # Unknown depends_on ids are not hard errors: suites wait via missing_deps /
  # waiting_on_deps. Cycle check only follows edges that exist in the map.

  # cycle check (DFS)
  state = {}
  visit = lambda do |id, stack|
    return if state[id] == :done
    if state[id] == :visiting
      errors << "dependency cycle involving #{id}"
      return
    end
    state[id] = :visiting
    (map[id]["depends_on"] || []).each { |d| visit.call(d, stack + [id]) if map.key?(d) }
    state[id] = :done
  end
  map.each_key { |id| visit.call(id, []) }

  if errors.empty?
    puts "OK: #{path} (#{map.size} suites)"
    0
  else
    errors.each { |e| warn "ERROR: #{e}" }
    1
  end
end

def cmd_status(data)
  map = suite_map(data)
  buckets = Hash.new { |h, k| h[k] = [] }
  ready = []

  map.each_value do |s|
    buckets[s["status"]] << s["id"]
    if s["status"] == "pending" && missing_deps(s, map).empty? && deps_satisfied?(s, map)
      ready << s
    end
  end

  puts "Registry status"
  puts "concurrency_cap: #{data['concurrency_cap']}"
  ALLOWED_STATUS.each do |st|
    ids = buckets[st] || []
    next if ids.empty?
    puts "#{st} (#{ids.size}): #{ids.sort.join(', ')}"
  end
  puts "ready (#{ready.size}): #{ready.sort_by { |s| -score_of(s) }.map { |s| "#{s['id']}(#{score_of(s)})" }.join(', ')}"
  waiting = map.values.select do |s|
    s["status"] == "pending" && (!missing_deps(s, map).empty? || !deps_satisfied?(s, map))
  end
  unless waiting.empty?
    puts "waiting_on_deps (#{waiting.size}): #{waiting.map { |s| s['id'] }.sort.join(', ')}"
  end
  0
end

def cmd_next(data, n)
  n = n.to_i
  n = 1 if n <= 0
  map = suite_map(data)
  ready = map.values.select do |s|
    s["status"] == "pending" && missing_deps(s, map).empty? && deps_satisfied?(s, map)
  end
  ready.sort_by! { |s| [-score_of(s), s["id"]] }
  picked = ready.first(n)
  if picked.empty?
    puts "No ready suites."
    return 0
  end
  picked.each do |s|
    puts "#{s['id']}\tscore=#{score_of(s)}\tpriority=#{s['priority']}\tdeps=#{(s['depends_on'] || []).join(',')}"
  end
  0
end

def cmd_score(data, id)
  map = suite_map(data)
  s = map[id]
  raise "unknown suite: #{id}" unless s
  puts "#{id} #{score_of(s)}"
  0
end

# Update only status/blocked_reason inside the suite block; keep comments and style.
def surgical_mark_done(text, id)
  lines = text.lines
  id_re = /^\s*-\s*id:\s*#{Regexp.escape(id)}\s*(?:#.*)?$/
  suite_start = lines.index { |l| l.match?(id_re) }
  raise "suite block not found in file text: #{id}" unless suite_start

  suite_end = lines.length
  ((suite_start + 1)...lines.length).each do |i|
    if lines[i].match?(/^\s*-\s*id:\s+\S/)
      suite_end = i
      break
    end
  end

  status_found = false
  blocked_found = false
  (suite_start...suite_end).each do |i|
    if lines[i] =~ /^(\s*)status:\s*/
      lines[i] = "#{$1}status: done\n"
      status_found = true
    elsif lines[i] =~ /^(\s*)blocked_reason:\s*/
      lines[i] = "#{$1}blocked_reason: null\n"
      blocked_found = true
    end
  end

  raise "status field not found in suite block: #{id}" unless status_found
  raise "blocked_reason field not found in suite block: #{id}" unless blocked_found
  lines.join
end

def cmd_record_merge(data, id, path, write:)
  map = suite_map(data)
  raise "unknown suite: #{id}" unless map[id]

  original = File.read(path)
  out = surgical_mark_done(original, id)

  if write
    File.write(path, out)
    puts "Wrote #{path}: #{id} -> done"
  else
    puts out
  end
  0
end

def main
  cmd = ARGV[0] || "status"
  case cmd
  when "validate"
    path = ARGV[1] || default_registry
    data = load_registry(path)
    exit validate!(data, path)
  when "status"
    path = ARGV[1] || default_registry
    data = load_registry(path)
    exit cmd_status(data)
  when "next"
    # next [path] [N]  OR  next [N] when path omitted
    path = default_registry
    n = 1
    if ARGV[1] && ARGV[1].end_with?(".yaml", ".yml")
      path = ARGV[1]
      n = (ARGV[2] || "1")
    elsif ARGV[1]
      n = ARGV[1]
    end
    data = load_registry(path)
    exit cmd_next(data, n)
  when "score"
    path = default_registry
    id = nil
    if ARGV[1]&.end_with?(".yaml", ".yml")
      path = ARGV[1]
      id = ARGV[2]
    else
      id = ARGV[1]
    end
    raise "usage: score [path] <suite_id>" if id.nil?
    data = load_registry(path)
    exit cmd_score(data, id)
  when "record-merge"
    write = ARGV.include?("--write")
    args = ARGV[1..].reject { |a| a == "--write" }
    path = default_registry
    id = nil
    if args[0]&.end_with?(".yaml", ".yml")
      path = args[0]
      id = args[1]
    else
      id = args[0]
    end
    raise "usage: record-merge [path] <suite_id> [--write]" if id.nil?
    data = load_registry(path)
    exit cmd_record_merge(data, id, path, write: write)
  else
    warn "Unknown command: #{cmd}"
    warn "Commands: validate status next score record-merge"
    exit 2
  end
rescue StandardError => e
  warn "ERROR: #{e.message}"
  exit 1
end

main
