# Cargo Quest

Learn Rust through small coding **quests** in the terminal.

## Status

Early development — CLI supports list, start, verify (compile + output checks), and profile. See the [development plan](.cursor/rules/CargoQuest_Development_Plan.md) for milestone details.

## Prerequisites

- [Rust](https://rustup.rs/) (2024 edition toolchain)

## Build

```bash
cargo build
```

## Run

```bash
cargo run -- quest-details variables_001
cargo run -- list
cargo run -- start variables_001
cargo run -- verify
cargo run -- profile
```

After installation:

```bash
cargo install --path .
cargo-quest list
```

## MVP (target)

```bash
cargo-quest list
cargo-quest start variables_001
cargo-quest verify
cargo-quest profile
```

`verify` runs every step in the quest’s `verify` list (compile, then output checks). If your code compiles but prints the wrong text, verification fails with an expected-vs-actual message and you do not earn XP. XP and completion are recorded only when all steps pass.

## Quest YAML schema

Quest **definitions** live under `src/quests/` as `{quest_id}.yaml` (for example `src/quests/variables_001.yaml`). They describe authored content—not the learner’s generated project (see [`CONTEXT.md`](./CONTEXT.md)).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | yes | Stable **quest id** (must match the filename stem, e.g. `variables_001`). |
| `title` | string | yes | Short display name shown to the learner. |
| `zone` | string | yes | Themed region (e.g. `Village of Variables`). |
| `instructions` | string | yes | What the learner should do; use YAML `\|` for multiple lines. |
| `xp` | integer | yes | Experience points awarded on first **completion**. |
| `starter` | string | yes | Rust source template written into the **active workspace** when the quest is started. |
| `victory_message` | string | yes | Flavor text printed on successful `verify` (zone-themed completion message). |
| `verify` | list | yes | Ordered verification steps run by `cargo-quest verify` (see below). |

### `verify` steps

Each item in `verify` is a test with:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | yes | How to check: `Compile`, `Output`, or `Behavior` (see table). |
| `expected` | string | yes | Meaning depends on `kind` (label for compile, exact stdout for output). |

| `kind` | What it does |
|--------|----------------|
| `Compile` | Runs `cargo check` in the active workspace. `expected` is a human-readable label (e.g. `App Compiles`); only success/failure matters. |
| `Output` | Runs `cargo run`, trims stdout, and compares it to `expected`. Fails with an expected-vs-actual message on mismatch. |
| `Behavior` | Runs `cargo test` and compares trimmed stdout to `expected`. Reserved for quests that ship tests in the workspace; most quests use `Compile` + `Output` only. |

Steps run **in order**. Verification succeeds only when every step passes. XP is awarded once per quest id (see `profile`).

### Example

```yaml
id: variables_001
title: "Hello, variables"
zone: "Village of Variables"
instructions: |
  Declare a variable `name` and print a greeting.
xp: 10
victory_message: "The village welcomes your first binding — variables are yours to name."
starter: |
  fn main() {
      // your code here
  }
verify:
  - kind: Compile
    expected: "App Compiles"
  - kind: Output
    expected: "Hello, John!"
```

Load and display one quest from the repo root:

```bash
cargo run -- quest-details variables_001
```

## Docs

- Domain glossary: [`CONTEXT.md`](./CONTEXT.md)
- Development plan: [`.cursor/rules/CargoQuest_Development_Plan.md`](./.cursor/rules/CargoQuest_Development_Plan.md)

## License

TBD
