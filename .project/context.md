# Current Project Context

## What we are doing

OpenSeed currently consists of a Rust CLI and a minimal Slint desktop GUI.
Stage 3 is complete. Stage 4 pointer movement input and Stage 5 dice input are
implemented. Stage 6 informational passphrase guidance and Stage 7 mnemonic
verification are implemented; passphrase input and seed derivation remain out
of scope.

## Important constraints

The project is security-sensitive.

Do not trade security for convenience. The mnemonic is generated from 256-bit
OS CSPRNG entropy through the established `bip39` crate. The application does
not persist, log, copy, or transmit the mnemonic.

The application no longer blocks generation when networking is enabled, by
explicit user decision. It still performs no network operations itself.

Do not add features simply because they may be useful later.

## Current development environment

Primary development machine:

macOS

Primary development environment:

VS Code + Codex CLI

Local LLM:

LM Studio

Potential local model:

Google Gemma 4 26B A4B QAT

## Workflow

The main agent is responsible for implementation.

Security-sensitive changes should receive a security review.

Tests should be run after meaningful changes.

## User preferences

The user prefers:

- simple solutions,
- minimal unnecessary complexity,
- practical implementation,
- explicit reasoning about security,
- incremental development.

Avoid unnecessary verbosity in implementation discussions.

## Current implementation state

- Stage 1: complete — CLI, OS CSPRNG, 24-word English BIP-39 generation and
  reference validation tests.
- Stage 2: informational only — the GUI checks local non-loopback network
  interfaces and displays ONLINE/OFFLINE status with a refresh action. It does
  not perform external requests or block generation; ADR-008 records this
  decision.
- Stage 3: complete — Slint software-rendered GUI with responsive layout,
  system-font rendering, warnings, and safe mnemonic display. The 24 generated
  words are presented as individually numbered cells in a visible 6×4 grid;
  labels are formatted in Rust before being passed to Slint. There is no
  clipboard or persistence.
- Stage 4: implemented for mouse/touchpad pointer movement — the GUI lets the
  user choose system RNG, movement, or both; collects a bounded set of distinct
  pointer positions; supports reset; and hashes selected inputs with SHA-256
  before BIP-39. The GUI visibly reports movement activity, sample count, and
  progress to the technical 1024-sample threshold. It does not claim a number of
  entropy bits. The two source checkboxes have explicit labels and descriptions;
  the system CSPRNG is marked as recommended. There is no controller-specific
  backend.
- Stage 5: implemented — the GUI accepts physical six-sided die results via
  buttons 1–6, converts 8-roll blocks with rejection sampling to avoid modulo
  bias, requires 13 accepted blocks, supports reset, and combines the result
  with other selected sources before BIP-39. Dice-only security depends on the
  physical rolls being fair. Stage 6 informational guidance and Stage 7
  verification are implemented.
- Support and Version/About controls are available both before generation and
  below the generated 6×4 mnemonic grid. Their details use a shared panel,
  and opening one closes the other.
- The complete dice input panel (roll buttons, counters, explanation, and
  reset action) is visible only while physical dice entropy is selected.
- The generated-result view provides a centered START OVER button between
  Support and Version/About. It clears the mnemonic model and collected
  movement/dice state before returning to source selection.
- Stage 6 informational guidance is implemented around the mnemonic grid. A
  red PASSPHRASE INFO button is above the grid and its details panel opens
  below the result. It explains that 24 words and 24 words plus a passphrase
  represent different wallets, the protection benefit, and the recovery risk.
  OpenSeed does not collect, store, copy, or derive from a passphrase.
- Stage 7: the GUI has a separate verification mode with 24 editable fields.
  It validates each entry against the official English BIP-39 list and then
  validates the checksum through the `bip39` crate. Verification state is
  cleared when leaving the mode; no entered mnemonic is persisted, logged, or
  copied.
- Stage 8 local preparation: release tests, a macOS arm64 release build, CLI
  smoke test, ad-hoc signature verification, and SHA-256 recording are done.
  Public release is not complete because independent security review,
  dependency database audit, cross-platform builds, and distribution signing
  remain pending. See `RELEASE.md`.

The repository uses `.project/` for project metadata. There is no `.ai/project/`
directory in the current checkout. The decisions file is named
`.project/decision.md` rather than `decisions.md`.
