# Cargo Quest

Cargo Quest is the domain of teaching Rust through small, verifiable coding challenges. The glossary names what learners, quests, and progress mean in this product—not how they are implemented.

## Language

### Core learning loop

**Learner**:
The person using Cargo Quest to practice Rust. Progress and XP belong to one learner profile on a machine.
_Avoid_: Player, user (in product copy), student

**Quest**:
A single teachable challenge with an id, instructions, starter code, and verification rules. One quest maps to one exercise the learner completes once for full credit.
_Avoid_: Lesson, exercise, challenge, level (when meaning a single challenge)

**Quest id**:
Stable string identifier for a quest (e.g. `variables_001`). Used in CLI commands, on-disk quest files, and profile completion records.
_Avoid_: Slug (alone), name (when ambiguous with display title)

**Start**:
The action that selects a quest as current and materializes an **active workspace** for editing. Only one quest should be active at a time in MVP.
_Avoid_: Open, launch, play

**Verification**:
Checking whether the learner’s code in the **active workspace** meets the quest’s requirements (compile, then optional output or behavior checks).
_Avoid_: Grading, test run (alone), check (alone)

**Completion**:
A quest is complete for a learner when verification succeeds and the quest has not already been rewarded on their **profile**.
_Avoid_: Win, pass (alone), solved

**XP**:
Experience points awarded once per quest completion. Duplicate completions must not grant XP again.
_Avoid_: Points, score, coins

### Quest content

**Quest definition**:
The authored description of a quest—typically YAML—including id, narrative, starter template, and verification spec. Not the learner’s edited Rust project.
_Avoid_: Quest file (when meaning the learner’s `main.rs`), metadata (alone)

**Quest pack**:
A curated set of related quests shipped together (e.g. beginner fundamentals). Packs group content for listing and narrative; they are not the same as a **campaign**.
_Avoid_: Module, course, bundle

**Starter code**:
The initial Rust source generated when a quest is started. The learner edits this in the **active workspace**.
_Avoid_: Template (alone), scaffold, boilerplate

**Zone**:
A themed region in the game layer that contains one or more quests (e.g. Village of Variables, Forest of Ownership). Zones organize flavor and unlock rules; each quest belongs to one zone.
_Avoid_: World, area, chapter (when meaning zone)

**Boss quest**:
A capstone quest type with stricter prerequisites, often gating a **zone**, and a larger **XP** reward than normal quests.
_Avoid_: Final, boss level, exam

### Progression and profile

**Profile**:
Persistent learner state: completed quests, **XP**, **titles**, and later **campaign** progress. One profile per installation path unless the product later defines otherwise.
_Avoid_: Save, account, user data

**Title**:
An earned label on the profile reflecting milestones (e.g. after enough XP or zone completion). Cosmetic recognition, not a permission.
_Avoid_: Rank, badge, achievement (alone)

**Level**:
A step within progression flavor (often tied to **XP** or zone progress). Not the same as a single **quest**.
_Avoid_: Stage, tier (when meaning quest difficulty)

**Campaign**:
A narrative track spanning multiple quests with tracked **campaign progress** on the **profile**. Stronger sequencing story than a **quest pack** alone.
_Avoid_: Quest pack (when meaning ordered story arc), storyline (alone)

### Workspaces and tooling

**Active quest**:
The quest currently in progress for the learner—the one whose **active workspace** exists and whose definition drives **verification**.
_Avoid_: Current mission, selected lesson

**Active workspace**:
A generated Cargo project directory where the learner edits code for the **active quest**. Verification runs against this tree, not against the Cargo Quest application crate.
_Avoid_: Sandbox, project folder, repo (learner’s solution)

**Compiler output**:
Raw diagnostics from `rustc` / `cargo` when verification fails. Always preserved for the learner even when a **friendly explanation** is shown.
_Avoid_: Error log (alone), stack trace (unless actually a panic trace)

**Friendly explanation**:
A short, learner-oriented hint mapped from common Rust error patterns. Supplements; does not replace **compiler output**.
_Avoid_: AI fix, simplified error (alone)

**Cargo Quest**:
The CLI/TUI application that lists quests, starts them, verifies solutions, and updates the **profile**. Distinct from any **active workspace** the app generates.
_Avoid_: rust-quest (package name), game engine

### Flagged ambiguities

- **Workspace**: In this context, almost always means **active workspace** (learner’s quest project). Say “Cargo Quest app” or “tooling crate” when referring to the main repository binary.
- **Verify / verification**: The product command is `verify`; the domain concept is **verification**. Do not use “test” alone when meaning the full verify pipeline (compile + quest-defined checks).
- **Quest pack vs campaign**: A pack is content grouping; a campaign is progress-tracked narrative. A campaign may draw quests from one or more packs.

## Example dialogue

**Dev:** When the learner runs start on `variables_003`, we copy the **starter code** into an **active workspace** and set **active quest** metadata.

**Expert:** Right—the **quest definition** stays in the pack; only the generated project is editable. **Verification** runs `cargo check` there first; later milestones add output checks from the definition.

**Dev:** If verify passes and the **profile** doesn’t already list that **quest id** as complete, we award **XP** once.

**Expert:** That’s **completion**. Re-running verify on the same quest must not duplicate **XP**. **Friendly explanation** can annotate **compiler output**, but we never hide rustc.

**Dev:** Village of Variables is a **zone** in the first **quest pack**; Forest of Ownership is another **zone**. **Boss quests** unlock after normal quests in a zone.

**Expert:** And in v0.1 we won’t confuse **campaign** progress with pack listing—campaigns come later as ordered tracks on the **profile**.
