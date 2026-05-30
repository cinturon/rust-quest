# Cargo Quest

Learn Rust through small coding **quests** in the terminal.

## Status

Early development — [Milestone 0](https://linear.app/jibjack/issue/JIB-132) (project setup).

## Prerequisites

- [Rust](https://rustup.rs/) (2024 edition toolchain)

## Build

```bash
cargo build
```

## Run

```bash
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

## Docs

- Domain glossary: [`CONTEXT.md`](./CONTEXT.md)
- Development plan: [`.cursor/rules/CargoQuest_Development_Plan.md`](./.cursor/rules/CargoQuest_Development_Plan.md)

## License

TBD
