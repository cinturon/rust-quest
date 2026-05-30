# Cargo Quest

Learn Rust through small coding **quests** in the terminal.

## Status

Early development — [Milestone 1](https://linear.app/jibjack/issue/JIB-134) (quest format) in progress; [Milestone 0](https://linear.app/jibjack/issue/JIB-132) complete.

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

## Quest YAML schema

Quest **definitions** live under `src/quests/` as `{quest_id}.yaml` (for example `src/quests/variables_001.yaml`). They describe authored content—not the learner’s generated project (see [`CONTEXT.md`](./CONTEXT.md)).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | yes | Stable **quest id** (must match the filename stem, e.g. `variables_001`). |
| `title` | string | yes | Short display name shown to the learner. |
| `zone` | string | yes | Themed region (e.g. `Village of Variables`). |
| `instructions` | string | yes | What the learner should do; use YAML `\|` for multiple lines. |
| `xp` | integer | yes | Experience points awarded on first **completion**. |
| `starter` | string | yes | Rust source template written into the **active workspace** when the quest is started (Milestone 3+). |
| `verify` | string | yes | Verification label for now (e.g. `App Compiles`); structured checks come in later milestones. |

### Example

```yaml
id: variables_001
title: "Hello, variables"
zone: "Village of Variables"
instructions: |
  Declare a variable `name` and print a greeting.
xp: 10
starter: |
  fn main() {
      // your code here
  }
verify: "App Compiles"
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
