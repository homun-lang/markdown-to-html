# Dev Rules

don't touch /tmp/, use ./.tmp/

## On Start — Read These First

1. `README.md` — project overview, architecture, tech stack
2. `.tmp/llm.plan.status` — ticket list and current status (pick `[ ]` tickets to work on)
3. `.tmp/llm.working.log` — abstract of recent completed work
4. `.tmp/llm.working.notes` — detailed working notes (if exists, read for more context)
5. Any `.tmp/llm*md` files — design docs, API specs, references

## Project Overview

markdown-to-html is a Markdown-to-HTML compiler/converter written in Homun language (.hom) + Rust.

**Tech stack**: Rust (Cargo project), Homun language (compiled via `homunc`)

Key directories:
```
src/           — Source code (.hom + .rs)
tests/         — Test suite
_site/         — Documentation site
```

### Homun Build System

`.hom` files are the source of truth, compiled at build time to `.rs` by `homunc`.

- `build.rs` auto-discovers/downloads `homunc` and compiles `src/*.hom` → `target/OUT_DIR/*.rs`
- Generated `.rs` files are NOT committed (they are build artifacts)
- Bootstrap homunc: `build.rs` checks `.tmp/homunc`, then PATH, then downloads from GitHub releases
- **Always update `Cargo.toml` version** when updating CHANGELOG.md (keep them in sync)

## Work Cycle

### Step 1: Clean Slate
```bash
git status
# If there are uncommitted changes → git reset --hard HEAD
# Start every session with a clean working tree
```

### Step 2: Pick ONE Ticket
- Read `.tmp/llm.plan.status`
- Find the first `[ ]` (unchecked) ticket
- Work on ONLY that ticket — one ticket per session

### Step 3: Implement
- Make the smallest possible change to complete the ticket
- Stay in scope — don't refactor unrelated code
- Don't add features beyond what the ticket asks

### Step 4: Test
Auto-detect project type and run tests:
- `Cargo.toml` → `cargo test`
- `package.json` → `npm test`
- `pyproject.toml` or `setup.py` → `pytest`
- `go.mod` → `go test ./...`
- `Makefile` with test target → `make test`

All tests MUST pass before proceeding.

### Step 5: Format + Lint
Auto-detect and run formatters/linters:
- JS/TS → `npx prettier --write .` and `npx eslint --fix .`
- Rust → `cargo fmt` and `cargo clippy -- -D warnings`
- Python → `ruff format .` and `ruff check --fix .`
- Go → `gofmt -w .` and `golangci-lint run`

### Step 6: Git Commit
dont commit `.claude/` `.tmp`

```bash
# Acquire lock (if multi-worker)
while ! mkdir _git.lock 2>/dev/null; do sleep 2; done

# Release lock
rmdir _git.lock
```

### Step 7: Update Status
1. Mark the ticket `[x]` in `.tmp/llm.plan.status`
2. Append a summary to `.tmp/llm.working.log`:
   ```
   [W{id}] <what was done> — <files changed>
   ```

## Temporary Files

- **All temp/scratch work MUST go in `./.tmp/`** (project-local), never `/tmp/`.
- `.tmp/` should be in .gitignore — safe for intermediate outputs, downloads, generated files, build artifacts, etc.
- Create `.tmp/` if it doesn't exist before writing to it.

## Autonomous Agent Teams

Use `/claude-bot` to set up autonomous agent teams that work while you're away.

1. **Plan**: Run `/claude-bot` and discuss your project — Claude breaks work into tickets and designs custom runner scripts at `.tmp/claude-bot/`
2. **Launch**: `bash .tmp/claude-bot/start.sh` — workers start solving tickets in tmux
3. **Walk away**: Go eat lunch, take a break — agents work autonomously
4. **Check results**: `tmux attach -t <project-folder-name>` or read `.tmp/llm.working.log`

See `.claude/skills/claude-bot/` for the full skill, example scripts, and planning workflow.

## Changelog

- Maintain `CHANGELOG.md` at the project root.
- Use **vMajor.Minor** format only (e.g., `v1.0`, `v1.1`, `v2.0`) — no patch level.
- Versions may jump (e.g., `v1.1` → `v1.5` or `v1.1` → `v3.0`) — a version jump signals a huge change.
- Each entry: version, date, and bullet list of what changed in short; not all details.

## Rules

- **ONE ticket per session.** Small steps. Do not batch multiple tickets.
- **Never ask questions.** Make reasonable decisions and document them in the commit message.
- **Stay in your assigned scope.** Don't touch files outside your task boundary.
- **If stuck after 3 attempts:** `git stash`, write BLOCKED to the trigger file, stop.
- **All tests must pass** before committing. If tests fail, fix them or stash and report BLOCKED.
- **Don't break existing tests.** If your change breaks unrelated tests, investigate before committing.
- **Commit messages matter.** Use format: `ticket: <verb> <what>` (e.g., `ticket: add markdown parser`)
