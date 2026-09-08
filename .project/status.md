# Project Status

## Current phase

Stage 8 — local release preparation (in progress)

## Completed

- [x] Project requirements reviewed
- [x] Security design reviewed
- [x] Rust selected
- [x] Minimal architecture defined

## In progress

- [x] Initial Rust implementation
- [x] BIP-39 integration
- [x] Test suite

## Not started

- [x] Informational local network status check without generation blocking
- [x] Basic GUI implementation
- [x] GUI layout modernization and responsive layout
- [x] 24-word mnemonic displayed in a visible 6×4 grid
- [x] GUI cell labels formatted as numbered BIP-39 words
- [x] Optional mouse/touchpad movement collection
- [x] Explicit source selection and movement reset workflow
- [x] SHA-256 mixing of selected entropy inputs before BIP-39
- [x] Visible movement activity, sample counter, and technical threshold progress
- [x] Explicit labels and security explanations for both entropy-source checkboxes
- [x] Optional physical six-sided dice input
- [x] Unbiased dice block extraction with rejection sampling
- [x] Dice counters, accepted-block status, and reset workflow in GUI
- [x] Optional static donation information below the generate button
- [x] Copy buttons and mouse/Ctrl+C selection for public donation addresses
- [x] High-contrast donation address fields with BTC/satoshi labels
- [x] Version/about information panel
- [x] All three entropy sources selected by default
- [x] Support/About controls available below the generated mnemonic grid
- [x] Dice input panel hides when physical dice entropy is deselected
- [x] START OVER action returns from the result view to source selection
- [x] Passphrase information panel below the mnemonic grid
- [x] Dedicated 24-word mnemonic verification workflow
- [x] Release profile build and local SHA-256 artifact hash
- [x] Release CLI and ad-hoc signature verification
- [x] MIT license and user-responsibility notice
- [x] macOS ARM64 application bundle and ZIP package
- [ ] Security review
- [ ] Independent dependency vulnerability database audit
- [ ] Cross-platform release builds and distribution signing
- [x] Final documentation

## Current task

Stage 3 basic GUI, its layout modernization, and the 6×4 mnemonic grid are
implemented and tested. Stage 4 movement collection is implemented for the
mouse/touchpad pointer path, with explicit source selection, a reset action,
bounded samples, visible activity feedback, explicit source descriptions, and
no claimed entropy-bit estimate. The technical collection threshold is 1024
distinct samples. A controller-specific input backend has not been
implemented. Stage 5 dice input is implemented with rejection sampling and
13 accepted blocks required for 256 bits before source mixing.
The GUI exposes copying only for public donation addresses; mnemonic copying
remains unavailable. All three entropy sources are selected by default.

## Known issues

The local Rust toolchain is installed through Homebrew for build verification.
The v0.1 CLI intentionally supports only system CSPRNG entropy. Network-based
generation blocking was explicitly disabled; the application itself performs
no network operations. The Slint software-rendered GUI uses a responsive,
system-font layout, displays all 24 words in a 6×4 grid, and has no persistence
or clipboard action. The GUI recommends the system CSPRNG and explains that
movement-only mode is experimental because pointer movement does not provide a
defensible measurable entropy estimate. The
project documentation in PROJECT.md still contains older Stage 1 roadmap text
in places; the current implementation state is recorded here. Stage 5 uses
8-roll dice blocks, rejection sampling, and 13 accepted blocks for 256 bits
before source mixing. Stage 6 adds an informational passphrase panel below
the mnemonic grid; passphrase input and seed derivation remain intentionally
out of scope.
Support/About controls are also available below the generated 6×4 mnemonic
grid, using a shared mutually-exclusive details panel.
The dice input panel is also conditionally hidden when its entropy source is
deselected.
The result view includes a centered START OVER action that clears the displayed
mnemonic, collected source data, counters, and open information panels.
Stage 6 is implemented as informational guidance only: OpenSeed does not ask
for, store, derive, or copy a passphrase. The red PASSPHRASE INFO button is
shown above the mnemonic grid, while its details panel opens below the result.
Stage 2 now displays a local ONLINE/OFFLINE status and recommends disconnecting
from the Internet, but generation remains a user-controlled action.
Stage 7 provides a separate verification screen with 24 input fields, English
BIP-39 word-list validation, checksum validation, and a BACK action that clears
the entered words.

## Next recommended action

Perform the final security review before treating Stage 8 as release-ready,
especially for movement-only and dice-only modes. Stage 8 release preparation
has produced a local macOS arm64 artifact and verification report, but public
release remains blocked on independent security review, dependency database
audit, cross-platform builds, and distribution signing/notarization.
