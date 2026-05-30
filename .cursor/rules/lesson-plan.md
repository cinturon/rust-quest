# Cargo Quest — Lesson Plan

## Project goal

Build a Rust **CLI/TUI** application that teaches Rust through small coding **quests**.

The first version should let a learner:

1. View available quests
2. Start a quest
3. Edit generated Rust code
4. Run verification
5. Complete the quest
6. Earn XP
7. Save progress

Authoritative milestone ordering and scope live in [`CargoQuest_Development_Plan.md`](./CargoQuest_Development_Plan.md). This document maps each milestone to **learning objectives**, **Rust concepts**, and **acceptance criteria** for agents and humans.

The project is a learning vehicle: each milestone should introduce Rust ideas in context, not as isolated exercises.

---

## Learning objectives

By completing Cargo Quest, you will gain experience with:

- Ownership and borrowing (quest workspaces, strings, paths)
- Structs and enums (quests, tests, game state)
- Error handling (`Result`, `anyhow`, user-facing messages)
- Serialization (YAML quests, JSON/TOML profiles)
- File I/O and project layout (quest packs, active workspace, profiles)
- Running external tools (`cargo check`, capturing stdout/stderr)
- CLI design (`clap` subcommands)
- Terminal UI (Ratatui, later milestones)
- Modular application design
- Testing and refactoring

---

## Recommended tech stack

Introduce crates when the milestone that needs them begins—not all at once.

| Crate | Role | Typical milestone |
|-------|------|-------------------|
| `clap` | CLI subcommands | 0 |
| `serde` | Serialize/deserialize | 1 |
| `serde_yaml` | Quest definitions | 1 |
| `anyhow` | Ergonomic errors in the app | 0–1 |
| `ratatui` | TUI | 10 |
| `crossterm` | Terminal I/O (with Ratatui) | 10 |

Defer `thiserror`, heavy generics, async, and extra crates until a milestone clearly needs them.

**Domain terms** (**Quest**, **Zone**, **Profile**, **Campaign**, **Verification**, **XP**, etc.) are defined in [`CONTEXT.md`](../../CONTEXT.md) at the repo root.

---

## MVP definition

MVP is complete when these commands work end-to-end and award XP:

```bash
cargo-quest list
cargo-quest start variables_001
cargo-quest verify
cargo-quest profile
```

That corresponds through **Milestone 5** (save progress) plus quest content from **Milestone 7** for `variables_001`.

---

## Recommended build order

Short path from empty repo to compiling verification (from the development plan):

1. Create Rust project
2. Add `clap`
3. Create `list` command
4. Create hardcoded quest
5. Print quest
6. Move quest into YAML
7. Parse YAML
8. Create active workspace
9. Generate `main.rs`
10. Run `cargo check`

---

## Module growth (incremental)

Grow layout by milestone—avoid a big-bang restructure.

| Stage | Suggested layout |
|-------|------------------|
| 0–2 | Thin `main.rs`, `cli/`, `quest/` (load + list) |
| 3–4 | `workspace/` (active quest), `verify/` (`cargo check`, later tests) |
| 5+ | `profile/` (XP, progress) |
| 10+ | `tui/` (panels, navigation), `app.rs` orchestration |
| 12+ | `campaign/` as needed |

Keep **verification** and **quest loading** independent of CLI/TUI so both interfaces can call the same core logic.

---

## Milestones

Each section below mirrors [`CargoQuest_Development_Plan.md`](./CargoQuest_Development_Plan.md).

---

### Milestone 0: Project setup

**Goal:** Runnable binary with basic CLI scaffolding.

**Concepts:** Cargo, crates, `clap`, project layout, `anyhow` for top-level errors.

**Deliverables:**

- Repository and README
- Dependencies: `clap`, `serde`, `serde_yaml`, `anyhow`
- Basic CLI commands (stubs OK if they compile)

**Milestone:** `cargo run -- <command>` parses subcommands without panicking.

---

### Milestone 1: Quest format

**Goal:** Define and load a single quest from YAML.

**Concepts:** Structs, `serde` derives, YAML schema, `impl` for display helpers.

**Deliverables:**

- Quest YAML format
- `Quest` struct
- Load quest from file
- Display quest details (title, instructions, etc.)

**Milestone:** One quest file loads and prints human-readable details.

---

### Milestone 2: Quest listing

**Goal:** Discover and show all quests in a pack or directory.

**Concepts:** `Vec`, sorting, directory traversal or glob, error propagation when a file is invalid.

**Deliverables:**

- Read all quest files
- Parse each into `Quest`
- Sort (e.g. by id or zone order)
- `list` command output

**Milestone:** `cargo-quest list` shows every available quest.

---

### Milestone 3: Start a quest

**Goal:** Materialize an editable Rust workspace for the active quest.

**Concepts:** Paths (`PathBuf`), file creation, templates, metadata sidecar (active quest state).

**Deliverables:**

- Generate active workspace directory
- Write `Cargo.toml` and `main.rs` (starter from quest)
- Save active quest metadata
- Print next steps for the learner

**Milestone:** `cargo-quest start <quest_id>` creates a compilable (or intentionally incomplete) workspace.

---

### Milestone 4: Verify — must compile

**Goal:** Run the learner’s code through `cargo check` and report clearly.

**Concepts:** `std::process::Command`, stdout/stderr capture, `Result` mapping to user messages.

**Deliverables:**

- Read active quest context
- Run `cargo check` in workspace
- Capture output
- Success/failure reporting

**Milestone:** `cargo-quest verify` reports compile success or failure with compiler output.

---

### Milestone 5: Save progress

**Goal:** Persistent profile with XP and idempotent rewards.

**Concepts:** Serde to disk (JSON or similar), load/save helpers, duplicate-completion guards.

**Deliverables:**

- Profile structure
- Load/save profile
- Award XP on verified completion
- Prevent duplicate rewards for the same quest
- `profile` command

**Milestone:** Completing a quest updates XP once; `cargo-quest profile` shows progress.

---

### Milestone 6: Better tests

**Goal:** Beyond “compiles”—assert behavior via quest-defined tests.

**Concepts:** Enums for test kinds, running `cargo test` or custom runners, stdout expectations.

**Deliverables:**

- Test enum in quest format
- Capture stdout (and stderr if needed)
- Validate expected output
- Actionable failure messages

**Milestone:** A quest can fail verification with a clear “expected vs actual” message.

---

### Milestone 7: First quest pack

**Goal:** Shippable beginner content aligned with Rust fundamentals.

**Content (from development plan):**

- **Village of Variables:** `variables_001` … `variables_005`
- **Forest of Ownership:** `ownership_001` … `ownership_005`

**Concepts:** Reinforce ownership, bindings, mutability, moves, and borrows through quest narrative.

**Milestone:** All ten quests list, start, verify, and award XP appropriately.

---

### Milestone 8: Game flavor layer

**Goal:** Make progression feel like a game, not a checklist.

**Concepts:** Optional structs for zones/levels, string formatting, completion side effects.

**Deliverables:**

- Zones and levels
- Titles
- Completion messages

**Milestone:** Finishing quests updates flavor text (zone progress, titles) in CLI output.

---

### Milestone 9: Error explanation v1

**Goal:** Friendly layer on top of rustc diagnostics.

**Concepts:** Pattern matching on error snippets, optional lookup table, preserving original compiler text.

**Deliverables:**

- Detect common Rust error codes (E0382, E0599, etc.)
- Short explanations for learners
- Full compiler output still visible

**Milestone:** A typical beginner mistake shows a hint plus the real `rustc` message.

---

### Milestone 10: TUI prototype

**Goal:** Ratatui shell without full quest flow yet.

**Concepts:** Ratatui layout, `crossterm` events, render loop, separating view from domain.

**Deliverables:**

- Add Ratatui
- Profile panel
- Quest list
- Navigation
- Quest detail view

**Milestone:** TUI launches and navigates mock or live data without crashing.

---

### Milestone 11: TUI quest flow

**Goal:** Parity with core CLI actions inside the TUI.

**Deliverables:**

- Start quest from UI
- Verify from UI
- Show results
- Refresh XP after completion

**Milestone:** A learner can complete a quest entirely from the TUI.

---

### Milestone 12: Campaigns

**Goal:** Group quests into narrative campaigns with tracked progress.

**Concepts:** Nested data (campaign → quests), campaign files, aggregate progress.

**Deliverables:**

- Campaign file format
- Campaign progress in profile
- `campaign` command (and TUI entry if applicable)

**Milestone:** Campaign progress persists across sessions.

---

### Milestone 13: Boss quests

**Goal:** Capstone quests with gates and larger rewards.

**Concepts:** Quest type enum variants, unlock rules, conditional listing.

**Deliverables:**

- Boss quest type in format
- Unlock zones or prior quests
- Larger XP rewards

**Milestone:** Boss quests hidden until prerequisites met; completion grants bonus XP.

---

### Milestone 14: Project polish

**Goal:** Operability and recovery for real users.

**Deliverables:**

- Better errors (consistent anyhow context chains)
- `reset` command
- `clean` command
- `doctor` command (environment checks)

**Milestone:** Common failure modes (missing active quest, bad paths, no `cargo`) have clear fixes.

---

### Milestone 15: Release v0.1

**Goal:** Tagged release with documented scope.

**Includes:**

- CLI
- Quest loading
- Verification
- Progress saving
- Beginner quest pack (Milestone 7)
- Friendly compiler explanations (Milestone 9)

**Milestone:** Release checklist passes; MVP commands documented in README.

---

## Recommended development strategy

**Keep features small.** Add one milestone behavior at a time; keep `cargo build` green when possible.

**Commit frequently.** One logical milestone step per commit when using git-flow tickets.

**Refactor often.** Extract `quest`, `workspace`, `verify`, and `profile` modules as boundaries stabilize.

**Expect compiler errors.** Learners—and the app invoking `rustc`—will surface ownership and type errors; treat them as teaching moments (see Milestone 9).

---

## Agent alignment

When planning or implementing work:

1. Name the **milestone** from [`CargoQuest_Development_Plan.md`](./CargoQuest_Development_Plan.md).
2. Do not skip ahead (e.g. TUI before CLI verify works) unless the user explicitly asks.
3. Prefer the **recommended build order** for greenfield steps within a milestone.
4. Use [`rust-learning-development-rules.mdc`](./rust-learning-development-rules.mdc) for teaching tone and code-change guardrails.
