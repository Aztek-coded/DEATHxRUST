# Command List Orchestration F0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship F0 of the Bleed parity orchestration system: suite registry, playbook, coordinator skill, and small registry tooling — with no Discord bot code changes.

**Architecture:** A YAML suite registry is the machine-readable queue. Ruby helper scripts validate the registry and compute `status` / `next`. A Grok skill (`.grok/skills/orchestrate-commands/`) tells agents how to run plan → implement → test → review → PR pipelines in worktrees. A STE100 playbook documents waves, foundations, and recovery. F1+ application suites are seeds only; this plan does not implement moderation or message foundations.

**Tech Stack:** Markdown skills, YAML registry, Ruby (stdlib `yaml` only), git worktrees (documented, not required to run in F0 verification).

**Spec:** `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md`

## Global Constraints

- F0 only: skill + registry + playbook + optional scripts. No `src/**/*.rs` changes.
- No auto-merge to `main`. Human merges PRs.
- Default suite pipeline concurrency cap: **3**.
- Registry path: `.grok/orchestration/suite-registry.yaml` (single source of truth).
- Skill path: `.grok/skills/orchestrate-commands/SKILL.md`.
- Playbook path: `.grok/guidelines/command-orchestration-playbook.md`.
- Inventory stays at `.grok/roadmaps/full command list roadmap.md` (do not rewrite the full catalog).
- User-facing docs use ASD-STE100 Simplified Technical English.
- Do not commit, push, or merge unless the user explicitly asks (plan steps that say “Commit” assume the user approved commits for this implementation run).
- Allowed suite `status` values only: `pending`, `planning`, `implementing`, `testing`, `reviewing`, `pr_open`, `done`, `blocked`.
- Score formula: `score = priority + unlock_bonus + infra_reuse_bonus - blocked_penalty` (see Task 2 for exact field names).

---

## File map

| Path | Responsibility |
|------|----------------|
| `.grok/orchestration/suite-registry.yaml` | Suite DAG, priority, status, paths, branch, PR |
| `.grok/orchestration/README.md` | How to edit registry; link to skill and playbook |
| `.grok/skills/orchestrate-commands/SKILL.md` | Coordinator agent instructions and modes |
| `.grok/skills/orchestrate-commands/scripts/registry_tool.rb` | Validate, status, next, record-merge helpers |
| `.grok/skills/orchestrate-commands/scripts/test_registry_tool.rb` | Tests for registry_tool |
| `.grok/guidelines/command-orchestration-playbook.md` | Waves, roles, git rules, recovery (STE100) |
| `.grok/README.md` | Index the new skill and orchestration path |
| `AGENTS.md` | One-line pointer to orchestrate-commands skill |

---

### Task 1: Registry tool tests (TDD — fail first)

**Files:**
- Create: `.grok/skills/orchestrate-commands/scripts/test_registry_tool.rb`
- Create: `.grok/skills/orchestrate-commands/scripts/fixtures/minimal-registry.yaml` (test fixture only)

**Interfaces:**
- Consumes: nothing yet
- Produces: failing tests that expect `registry_tool.rb` commands: `validate`, `status`, `next`

- [ ] **Step 1: Create skill scripts directory and fixture**

```bash
mkdir -p .grok/skills/orchestrate-commands/scripts/fixtures
```

Write fixture `.grok/skills/orchestrate-commands/scripts/fixtures/minimal-registry.yaml`:

```yaml
version: 1
concurrency_cap: 3
suites:
  - id: foundation-a
    category: foundation
    title: Foundation A
    commands: []
    depends_on: []
    priority: 1000
    unlock_bonus: 50
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: done
    blocked_reason: null
    paths:
      commands: null
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: ""

  - id: suite-ready
    category: moderation
    title: Ready suite
    commands: [ban]
    depends_on: [foundation-a]
    priority: 900
    unlock_bonus: 10
    infra_reuse_bonus: 5
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/moderation/
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: "MVP ban"

  - id: suite-blocked-dep
    category: moderation
    title: Blocked by dep
    commands: [jail]
    depends_on: [suite-missing]
    priority: 800
    unlock_bonus: 0
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/moderation/
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: ""

  - id: suite-explicit-blocked
    category: music
    title: Explicit blocked
    commands: [play]
    depends_on: []
    priority: 200
    unlock_bonus: 0
    infra_reuse_bonus: 0
    blocked_penalty: 100
    status: blocked
    blocked_reason: "needs music stack"
    paths:
      commands: null
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: ""
```

- [ ] **Step 2: Write the test file**

Write `.grok/skills/orchestrate-commands/scripts/test_registry_tool.rb`:

```ruby
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
```

- [ ] **Step 3: Run tests — expect failure (tool missing)**

```bash
ruby .grok/skills/orchestrate-commands/scripts/test_registry_tool.rb
```

Expected: FAIL (cannot load tool or commands missing), non-zero exit.

- [ ] **Step 4: Commit** (if user approved commits for this run)

```bash
git add .grok/skills/orchestrate-commands/scripts/test_registry_tool.rb \
  .grok/skills/orchestrate-commands/scripts/fixtures/minimal-registry.yaml
git commit -m "test: add failing registry_tool tests for orchestration F0"
```

---

### Task 2: Implement `registry_tool.rb`

**Files:**
- Create: `.grok/skills/orchestrate-commands/scripts/registry_tool.rb`
- Test: `.grok/skills/orchestrate-commands/scripts/test_registry_tool.rb`

**Interfaces:**
- Consumes: registry YAML path (default `.grok/orchestration/suite-registry.yaml` relative to repo root)
- Produces CLI:
  - `ruby registry_tool.rb validate [path]`
  - `ruby registry_tool.rb status [path]`
  - `ruby registry_tool.rb next [path] [N]`
  - `ruby registry_tool.rb score [path] <suite_id>`
  - `ruby registry_tool.rb record-merge [path] <suite_id>` — sets `status: done`, clears `blocked_reason` if empty string not needed; prints new YAML to stdout **or** writes in place with `--write` flag
  - Use **`--write`** only for `record-merge` to mutate the file in place; default is dry-run print

Exact score:

```ruby
score = priority + unlock_bonus + infra_reuse_bonus - blocked_penalty
```

Ready suite rules:

1. `status == "pending"` (only pending is auto-selectable for `next`; other in-progress statuses are listed under status but not started again)
2. Every id in `depends_on` exists and has `status == "done"`
3. Not missing dependency ids

- [ ] **Step 1: Write `registry_tool.rb`**

```ruby
#!/usr/bin/env ruby
# frozen_string_literal: true

require "yaml"
require "set"

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

  map.each_value do |s|
    (s["depends_on"] || []).each do |d|
      errors << "#{s['id']}: unknown depends_on #{d}" unless map.key?(d)
    end
  end

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

def cmd_record_merge(data, id, path, write:)
  map = suite_map(data)
  s = map[id]
  raise "unknown suite: #{id}" unless s
  s["status"] = "done"
  s["blocked_reason"] = nil
  out = data.to_yaml
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
```

- [ ] **Step 2: Make executable and run tests**

```bash
chmod +x .grok/skills/orchestrate-commands/scripts/registry_tool.rb
chmod +x .grok/skills/orchestrate-commands/scripts/test_registry_tool.rb
ruby .grok/skills/orchestrate-commands/scripts/test_registry_tool.rb
```

Expected: `All tests passed.` exit 0.

- [ ] **Step 3: Commit**

```bash
git add .grok/skills/orchestrate-commands/scripts/registry_tool.rb
git commit -m "feat: add registry_tool for suite orchestration status and next"
```

---

### Task 3: Seed production suite registry

**Files:**
- Create: `.grok/orchestration/suite-registry.yaml`
- Create: `.grok/orchestration/README.md`

**Interfaces:**
- Consumes: field schema from Task 2
- Produces: seed suites F0–F2 + Wave 1–3 product suites from the design spec

- [ ] **Step 1: Write `.grok/orchestration/suite-registry.yaml`**

Use this content exactly (scores and deps match the design):

```yaml
version: 1
concurrency_cap: 3
# Bleed parity suite queue. Inventory detail:
# .grok/roadmaps/full command list roadmap.md
# Design:
# docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md

suites:
  # --- Foundations ---
  - id: agent-orchestration
    category: foundation
    title: Multi-agent orchestration F0
    commands: []
    depends_on: []
    priority: 1100
    unlock_bonus: 100
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: implementing
    blocked_reason: null
    paths:
      commands: null
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: docs/superpowers/plans/2026-07-19-command-list-orchestration-f0.md
    notes: "Skill, registry, playbook only. No bot src changes."

  - id: moderation-foundation
    category: foundation
    title: Moderation case store and staff helpers
    commands: []
    depends_on: [agent-orchestration]
    priority: 1050
    unlock_bonus: 80
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: null
      data: src/data/models/moderation.rs
      handlers: null
    branch: feature/moderation-foundation
    pr: null
    roadmap: null
    notes: "F1: case log, reason helpers, staff checks, shared mod embeds."

  - id: message-foundation
    category: foundation
    title: Shared message event router hooks
    commands: []
    depends_on: [agent-orchestration]
    priority: 1040
    unlock_bonus: 70
    infra_reuse_bonus: 10
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: null
      data: null
      handlers: src/handlers/message_router.rs
    branch: feature/message-foundation
    pr: null
    roadmap: null
    notes: "F2: MessageCreate/Update/Delete routing pattern."

  # --- Wave 1: moderation MVPs ---
  - id: moderation-punish
    category: moderation
    title: Core punish commands
    commands: [timeout, untimeout, ban, unban, softban, warn, warnings]
    depends_on: [moderation-foundation]
    priority: 900
    unlock_bonus: 40
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/moderation/
      data: src/data/models/moderation.rs
      handlers: null
    branch: feature/moderation-punish
    pr: null
    roadmap: null
    notes: "MVP cases; no hardban/jail."

  - id: moderation-purge-basic
    category: moderation
    title: Basic purge
    commands: [purge, "purge bots", "purge humans"]
    depends_on: [moderation-foundation]
    priority: 880
    unlock_bonus: 10
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/moderation/
      data: null
      handlers: null
    branch: feature/moderation-purge-basic
    pr: null
    roadmap: null
    notes: "Not full Bleed purge filter set."

  - id: moderation-history
    category: moderation
    title: Case history and reason edit
    commands: [history, caselog, reason]
    depends_on: [moderation-foundation, moderation-punish]
    priority: 870
    unlock_bonus: 5
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/moderation/
      data: src/data/models/moderation.rs
      handlers: null
    branch: feature/moderation-history
    pr: null
    roadmap: null
    notes: ""

  # --- Wave 2: existing handler leverage ---
  - id: boosts
    category: servers
    title: Boost messages
    commands: [boosts]
    depends_on: [agent-orchestration]
    priority: 800
    unlock_bonus: 15
    infra_reuse_bonus: 20
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/boosts/
      data: src/data/models/boosts.rs
      handlers: src/handlers/boost_handler.rs
    branch: feature/boosts
    pr: null
    roadmap: null
    notes: "Extends BoostHandler."

  - id: welcome-goodbye
    category: servers
    title: Welcome and goodbye messages
    commands: [welcome, goodbye]
    depends_on: [agent-orchestration]
    priority: 800
    unlock_bonus: 15
    infra_reuse_bonus: 20
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/welcome/
      data: src/data/models/welcome.rs
      handlers: src/handlers/member_handler.rs
    branch: feature/welcome-goodbye
    pr: null
    roadmap: null
    notes: "Extends MemberHandler. Prefer after message-foundation if sticky shared later."

  # --- Wave 3: message features + light utils ---
  - id: snipe
    category: snipe
    title: Message snipe
    commands: [snipe]
    depends_on: [message-foundation]
    priority: 700
    unlock_bonus: 5
    infra_reuse_bonus: 10
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/snipe/
      data: null
      handlers: src/handlers/message_router.rs
    branch: feature/snipe
    pr: null
    roadmap: null
    notes: ""

  - id: stickymessage
    category: servers
    title: Sticky messages
    commands: [stickymessage]
    depends_on: [message-foundation]
    priority: 700
    unlock_bonus: 5
    infra_reuse_bonus: 10
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/stickymessage/
      data: src/data/models/sticky.rs
      handlers: src/handlers/message_router.rs
    branch: feature/stickymessage
    pr: null
    roadmap: null
    notes: ""

  - id: firstmessage
    category: servers
    title: First message link
    commands: [firstmessage]
    depends_on: [agent-orchestration]
    priority: 600
    unlock_bonus: 0
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/firstmessage.rs
      data: null
      handlers: null
    branch: feature/firstmessage
    pr: null
    roadmap: null
    notes: "Small utility; low coupling."

  - id: pin-utils
    category: servers
    title: Pin and unpin
    commands: [pin, unpin]
    depends_on: [agent-orchestration]
    priority: 600
    unlock_bonus: 0
    infra_reuse_bonus: 0
    blocked_penalty: 0
    status: pending
    blocked_reason: null
    paths:
      commands: src/commands/pin.rs
      data: null
      handlers: null
    branch: feature/pin-utils
    pr: null
    roadmap: null
    notes: ""

  # --- Explicitly blocked examples ---
  - id: music-core
    category: music
    title: Music playback
    commands: [play, skip, stop, queue]
    depends_on: [agent-orchestration]
    priority: 200
    unlock_bonus: 0
    infra_reuse_bonus: 0
    blocked_penalty: 100
    status: blocked
    blocked_reason: "needs media stack and external APIs"
    paths:
      commands: src/commands/music/
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: ""

  - id: lastfm-core
    category: lastfm
    title: Last.fm integration
    commands: [fm, nowplaying]
    depends_on: [agent-orchestration]
    priority: 200
    unlock_bonus: 0
    infra_reuse_bonus: 0
    blocked_penalty: 100
    status: blocked
    blocked_reason: "needs Last.fm API design"
    paths:
      commands: src/commands/lastfm/
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: ""

  - id: prefix-self
    category: servers
    title: Personal prefix Tier 2
    commands: ["prefix self"]
    depends_on: [agent-orchestration]
    priority: 400
    unlock_bonus: 0
    infra_reuse_bonus: 5
    blocked_penalty: 80
    status: blocked
    blocked_reason: "needs premium tier system"
    paths:
      commands: src/commands/prefix.rs
      data: null
      handlers: null
    branch: null
    pr: null
    roadmap: null
    notes: ""
```

- [ ] **Step 2: Write `.grok/orchestration/README.md`** (STE100)

```markdown
# Suite orchestration registry

## Purpose

This folder holds the machine-readable queue for Bleed command parity work.

The human catalog is:

`.grok/roadmaps/full command list roadmap.md`

Do not treat the catalog as the build plan.

## Files

| File | Role |
|------|------|
| `suite-registry.yaml` | Suite list, deps, status, paths |

## Edit rules

1. Keep `version: 1`.
2. Use only allowed status values: `pending`, `planning`, `implementing`, `testing`, `reviewing`, `pr_open`, `done`, `blocked`.
3. When `status` is `blocked`, set a non-empty `blocked_reason`.
4. Prefer suite groups (40–80 suites). Do not add one row per leaf command.
5. After you merge a suite PR, run `record-merge` (see skill).

## Tool commands

From the repo root:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next 3
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge <suite-id> --write
```

## Related docs

- Design: `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md`
- Playbook: `.grok/guidelines/command-orchestration-playbook.md`
- Skill: `.grok/skills/orchestrate-commands/SKILL.md`
```

- [ ] **Step 3: Validate production registry**

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate .grok/orchestration/suite-registry.yaml
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status .grok/orchestration/suite-registry.yaml
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next .grok/orchestration/suite-registry.yaml 5
```

Expected:

- `validate` prints `OK` and suite count
- `status` shows `agent-orchestration` under `implementing`
- `next` does **not** list suites that depend on unfinished foundations (e.g. no `moderation-punish` until `moderation-foundation` is done). Ready set after F0 may be empty except suites that only depend on `agent-orchestration` **if** that suite is `done`. While `agent-orchestration` is `implementing`, suites depending only on it wait. Document that in the test run output.

**Important readiness note:** With the seed above, `next` should print `No ready suites.` until `agent-orchestration` is `done`. That is correct.

- [ ] **Step 4: Commit**

```bash
git add .grok/orchestration/suite-registry.yaml .grok/orchestration/README.md
git commit -m "feat: seed suite registry for Bleed command orchestration"
```

---

### Task 4: Write orchestration playbook (STE100)

**Files:**
- Create: `.grok/guidelines/command-orchestration-playbook.md`

**Interfaces:**
- Consumes: design sections on waves, roles, recovery
- Produces: human playbook agents and humans follow

- [ ] **Step 1: Write the playbook**

Write `.grok/guidelines/command-orchestration-playbook.md` with these sections (full prose, STE100, no placeholders):

1. **Purpose** — orchestrate Bleed parity; inventory is not the plan  
2. **Roles** — coordinator, plan, implement, test, review, human merge  
3. **Suite rules** — vertical slice DoD (copy from design Section 5 as a checklist)  
4. **Foundations** — F0 agent-orchestration, F1 moderation-foundation, F2 message-foundation  
5. **Waves** — table Wave 0–4 from design  
6. **Parallelism** — cap 3; disjoint paths; serial foundations; hot-file registration last  
7. **Git** — branch `feature/<suite-id>`, worktree, one PR, no auto-merge, `record-merge` after merge  
8. **Pipeline stages** — plan → implement → test → review → PR  
9. **Recovery** — table from design Section 7  
10. **Commands** — how to run `registry_tool.rb` and `/orchestrate-commands` modes  
11. **Out of scope** — music, lastfm, tier-2 until unblocked  

Include this vertical-slice checklist verbatim:

```markdown
## Vertical slice checklist

A suite PR is ready when:

1. Commands use Poise and project patterns.
2. Permissions match intent.
3. Data models exist only if needed (Discord IDs as i64).
4. Handlers exist only if needed.
5. Responses use ResponseHelper and EmbedColor.
6. Commands are exported and registered.
7. cargo fmt, clippy, and test pass for the change.
8. Acceptance notes list what works and what is deferred.
9. Deferred Bleed subcommands are listed, not silent gaps.
```

- [ ] **Step 2: Sanity check length and links**

```bash
test -f .grok/guidelines/command-orchestration-playbook.md
wc -l .grok/guidelines/command-orchestration-playbook.md
rg -n "record-merge|concurrency|F0|F1|F2|auto-merge" .grok/guidelines/command-orchestration-playbook.md
```

Expected: file exists; mentions concurrency, F0–F2, no auto-merge, record-merge.

- [ ] **Step 3: Commit**

```bash
git add .grok/guidelines/command-orchestration-playbook.md
git commit -m "docs: add command orchestration playbook"
```

---

### Task 5: Write coordinator skill `orchestrate-commands`

**Files:**
- Create: `.grok/skills/orchestrate-commands/SKILL.md`

**Interfaces:**
- Consumes: registry path, playbook path, `registry_tool.rb`
- Produces: agent procedure for modes in the design

- [ ] **Step 1: Write SKILL.md**

```markdown
---
name: orchestrate-commands
description: >
  Coordinate multi-agent Bleed command parity work using the suite registry,
  worktrees, and plan/implement/test/review pipelines. Use when the user runs
  /orchestrate-commands, asks to orchestrate command suites, run a command wave,
  pick the next suite, update suite status, or parallelize Bleed parity work.
---

# Orchestrate Commands

Drive DEATHxRUST Bleed parity using the suite registry. Do not implement the full catalog in one pass.

## Canonical paths

| Item | Path |
|------|------|
| Registry | `.grok/orchestration/suite-registry.yaml` |
| Inventory | `.grok/roadmaps/full command list roadmap.md` |
| Playbook | `.grok/guidelines/command-orchestration-playbook.md` |
| Tool | `.grok/skills/orchestrate-commands/scripts/registry_tool.rb` |
| Design | `docs/superpowers/specs/2026-07-19-command-list-orchestration-design.md` |
| Suite roadmaps | `.grok/roadmaps/<suite-id>-roadmap.md` |

## Hard rules

1. Registry is the queue source of truth. Inventory is human reference.
2. One suite → one branch `feature/<suite-id>` → one PR. No auto-merge.
3. Default parallel cap: 3 suites. Foundations run serial.
4. Ship-ready vertical slices (MVP OK). List deferred commands in the PR.
5. Follow project patterns: Poise, ResponseHelper, EmbedColor, i64 Discord IDs.
6. Do not commit, push, merge, or open a PR unless the user allowed it for this run.
7. When the user invokes `pipeline` or `wave` and allows PRs, you may open PRs; you still must not merge.
8. Never edit hot registration files until the end of an implement pass. Serialize if conflict.

## Modes

Parse the user message for a mode. If none, run `status` then ask which mode.

### `status`

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
```

Report buckets and ready queue to the user.

### `sync-catalog`

1. Read category index in the inventory.
2. Compare parent command families to registry `commands` / suite ids.
3. Report missing suite stubs only. Do not implement. Do not auto-add rows unless the user asks.

### `next [N]`

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next N
```

Explain why each suite is ready (deps done, score).

### `plan <id>`

1. Load suite from registry. Fail if unknown.
2. Set status to `planning` in registry (edit YAML carefully).
3. Read inventory entries for suite `commands`.
4. Read existing code paths if present.
5. Write `.grok/roadmaps/<id>-roadmap.md` with: goal, MVP command list, files to touch, data/handlers, acceptance, deferred list, branch name.
6. Set `roadmap:` field on the suite. Set status back to `pending` (plan done, not implementing) unless user said to continue.

### `implement <id>`

1. Require a roadmap path on the suite or write one first via `plan`.
2. Confirm user allows branch/worktree for this run.
3. Create worktree/branch `feature/<id>` from latest main.
4. Set status `implementing`.
5. Implement vertical slice only.
6. Register commands last.
7. Run fmt/test/clippy in the worktree.
8. Stop for test/review modes or continue if `pipeline`.

### `test <id>`

1. Set status `testing`.
2. In the suite worktree: `cargo fmt`, `cargo test`, `cargo clippy`.
3. Up to 2 fix attempts on failure, then set `blocked` with reason.
4. On success, leave status `testing` or advance to review if pipeline.

### `review <id>`

1. Set status `reviewing`.
2. Review diff for security, permissions, registration, ResponseHelper, scope honesty.
3. Critical findings: one fix loop + re-test.
4. On clean review, proceed to PR if allowed.

### `pipeline <id>`

Run plan → implement → test → review → open PR (if allowed) → set `pr_open` and `pr:` URL.

### `wave [N]`

1. Read `concurrency_cap` (default 3). `N = min(N, cap)`.
2. `next N` for ready suites.
3. Filter to **disjoint** `paths` (no two suites write the same non-null path).
4. If path conflict, drop lower score suite from this wave.
5. Run up to N pipelines **in parallel** (subagents/worktrees) only when user allows.
6. Summarize PRs and registry updates.

### `record-merge <id>`

After the user merges:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge <id> --write
```

Confirm dependents that became ready.

## Agent prompts (short)

When spawning subagents, include:

- Suite id and registry YAML excerpt
- Roadmap path
- Playbook checklist
- Hard rules above
- Exact paths to edit
- "Do not merge to main"

## F0 special case

Suite `agent-orchestration` is docs/skill/registry only. Do not change `src/**/*.rs` for F0.
```

- [ ] **Step 2: Verify frontmatter and discovery**

```bash
head -20 .grok/skills/orchestrate-commands/SKILL.md
test -f .grok/skills/orchestrate-commands/SKILL.md
```

Expected: YAML frontmatter with `name: orchestrate-commands`.

- [ ] **Step 3: Commit**

```bash
git add .grok/skills/orchestrate-commands/SKILL.md
git commit -m "feat: add orchestrate-commands Grok skill"
```

---

### Task 6: Wire project indexes

**Files:**
- Modify: `.grok/README.md`
- Modify: `AGENTS.md` (repo root)

**Interfaces:**
- Consumes: skill name `orchestrate-commands`
- Produces: discoverable docs

- [ ] **Step 1: Update `.grok/README.md` skills table**

Add a row to the Skills table:

```markdown
| `orchestrate-commands` | `/orchestrate-commands` | Multi-agent Bleed parity: registry, waves, plan/implement/test/review |
```

Add under Layout table (or new section):

```markdown
| `orchestration/` | Suite registry for multi-agent command parity |
```

- [ ] **Step 2: Update root `AGENTS.md` skills table**

Add:

```markdown
| `orchestrate-commands` | Multi-agent Bleed command parity orchestration (registry + waves) |
```

If the skills table was emptied or files deleted in the working tree, restore consistency with remaining skills and still list `orchestrate-commands`. Do not resurrect deleted skills unless the user asks.

- [ ] **Step 3: Commit**

```bash
git add .grok/README.md AGENTS.md
git commit -m "docs: index orchestrate-commands skill and registry"
```

---

### Task 7: F0 verification dry run

**Files:**
- Modify: `.grok/orchestration/suite-registry.yaml` (status of `agent-orchestration` only, at end)

**Interfaces:**
- Consumes: all prior tasks
- Produces: proof F0 success criteria from design Section 10

- [ ] **Step 1: Run tool battery**

```bash
ruby .grok/skills/orchestrate-commands/scripts/test_registry_tool.rb
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb validate
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb status
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next 3
```

Expected: tests pass; validate OK; status shows implementing/pending/blocked; next empty or only truly ready suites.

- [ ] **Step 2: Dry-run plan artifact path**

Without implementing bot code, create a short plan note that `plan agent-orchestration` would use (optional if this F0 plan already is the roadmap). Ensure registry `roadmap` for `agent-orchestration` points at this plan file.

- [ ] **Step 3: Mark F0 done in registry**

After user confirms F0 artifacts are complete:

```bash
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb record-merge agent-orchestration --write
ruby .grok/skills/orchestrate-commands/scripts/registry_tool.rb next 5
```

Expected after merge record:

- `agent-orchestration` is `done`
- `next` lists high-score suites that depend only on `agent-orchestration` (e.g. `moderation-foundation`, `message-foundation`, `boosts`, `welcome-goodbye`, `firstmessage`, `pin-utils`) and **not** `moderation-punish` (still needs F1)

- [ ] **Step 4: Final commit**

```bash
git add .grok/orchestration/suite-registry.yaml
git commit -m "chore: mark agent-orchestration suite done after F0 ship"
```

- [ ] **Step 5: Success criteria checklist**

- [ ] Coordinator skill exists with all modes  
- [ ] Registry exists with F0–F2 and Wave 1–3 seeds  
- [ ] Playbook exists  
- [ ] `status`, `next`, validate work  
- [ ] End-to-end dry run documented (tool output in commit message or session notes)  
- [ ] No `src/**/*.rs` changes in F0  

---

## Self-review (plan author)

### Spec coverage

| Spec requirement | Task |
|------------------|------|
| Suite registry YAML | Task 3 |
| Coordinator skill modes | Task 5 |
| Playbook | Task 4 |
| registry tooling status/next | Tasks 1–2 |
| record-merge | Tasks 2, 7 |
| concurrency cap 3 | Tasks 3, 5 |
| F0 no bot code | Global + Task 5 F0 special case |
| Seeds F0–F2 + Wave 1–3 | Task 3 |
| Index skill in project docs | Task 6 |
| Human merge only | Skill + playbook |
| Score formula | Task 2 |
| Success criteria dry run | Task 7 |

### Placeholder scan

None intentional. Full YAML and Ruby code included.

### Type / field consistency

Suite fields used in tool and seed: `id`, `category`, `title`, `commands`, `depends_on`, `priority`, `unlock_bonus`, `infra_reuse_bonus`, `blocked_penalty`, `status`, `blocked_reason`, `paths`, `branch`, `pr`, `roadmap`, `notes`.

---

## Execution handoff

After this plan is saved, implement with one of:

1. **Subagent-Driven (recommended)** — superpowers:subagent-driven-development  
2. **Inline Execution** — superpowers:executing-plans  

Do not start F1 moderation-foundation application code in this plan.
