# Cargo Quest Development Plan

## Project Goal

Build a Rust CLI/TUI application that teaches Rust through small coding
quests.

The first version should let a learner:

1.  View available quests.
2.  Start a quest.
3.  Edit generated Rust code.
4.  Run verification.
5.  Complete the quest.
6.  Earn XP.
7.  Save progress.

------------------------------------------------------------------------

# Milestone 0: Project Setup

-   Create repository
-   Add README
-   Add dependencies (clap, serde, serde_yaml, anyhow)
-   Create basic CLI commands

# Milestone 1: Quest Format

-   Create quest YAML format
-   Create Quest struct
-   Load quest from YAML
-   Display quest details

# Milestone 2: Quest Listing

-   Read all quest files
-   Parse quests
-   Sort quests
-   Display available quests

# Milestone 3: Start a Quest

-   Generate active workspace
-   Create Cargo.toml
-   Create main.rs
-   Save active quest metadata
-   Display next steps

# Milestone 4: Verify Must Compile

-   Read active quest
-   Run cargo check
-   Capture output
-   Report success/failure

# Milestone 5: Save Progress

-   Create profile structure
-   Load/save profile
-   Award XP
-   Prevent duplicate rewards
-   Display profile

# Milestone 6: Better Tests

-   Add test enum
-   Capture stdout
-   Validate output
-   Show useful failure messages

# Milestone 7: First Quest Pack

Village of Variables: - variables_001 - variables_002 - variables_003 -
variables_004 - variables_005

Forest of Ownership: - ownership_001 - ownership_002 - ownership_003 -
ownership_004 - ownership_005

# Milestone 8: Game Flavor Layer

-   Zones
-   Levels
-   Titles
-   Completion messages

# Milestone 9: Error Explanation v1

-   Detect common Rust error codes
-   Friendly explanations
-   Preserve compiler output

# Milestone 10: TUI Prototype

-   Add Ratatui
-   Profile panel
-   Quest list
-   Navigation
-   Quest details

# Milestone 11: TUI Quest Flow

-   Start quests
-   Verify quests
-   Show results
-   Refresh XP

# Milestone 12: Campaigns

-   Campaign files
-   Campaign progress
-   Campaign command

# Milestone 13: Boss Quests

-   Boss quest type
-   Unlock zones
-   Larger rewards

# Milestone 14: Project Polish

-   Better errors
-   Reset command
-   Clean command
-   Doctor command

# Milestone 15: Release v0.1

Include: - CLI - Quest loading - Verification - Progress saving -
Beginner quest pack - Friendly compiler explanations

## Recommended Build Order

1.  Create Rust project
2.  Add clap
3.  Create list command
4.  Create hardcoded quest
5.  Print quest
6.  Move quest into YAML
7.  Parse YAML
8.  Create active workspace
9.  Generate main.rs
10. Run cargo check

## MVP Definition

The MVP is complete when:

``` bash
cargo-quest list
cargo-quest start variables_001
cargo-quest verify
cargo-quest profile
```

works successfully and awards XP.
