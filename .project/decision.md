# Architecture Decisions

## ADR-001 — Rust

Decision:

Use Rust as the implementation language.

Reason:

The project generates cryptographic material and benefits from
memory safety, explicit ownership and strong compile-time guarantees.

---

## ADR-002 — OS CSPRNG

Decision:

Use the operating system's cryptographically secure random number
generator.

Do not implement a custom PRNG.

---

## ADR-003 — BIP-39 library

Decision:

Use a well-established BIP-39 implementation rather than implementing
the standard from scratch.

---

## ADR-004 — Minimal v0.1

Decision:

v0.1 will remain CLI-only.

No GUI, dice entropy, mouse entropy, passphrase support or donation
interface will be implemented unless explicitly approved.

---

## ADR-005 — No persistence

Decision:

The mnemonic must never be intentionally written to:

- disk
- logs
- clipboard
- network
- application cache

---

## ADR-006 — Local offline enforcement (superseded)

Decision:

Before generating a mnemonic, inspect local network interfaces with
`getifaddrs`. Refuse generation when a non-loopback interface is up and has
an IPv4 or IPv6 address.

The check must not perform DNS, HTTP, API calls, telemetry, or any other
external connectivity test. It is intentionally conservative and cannot
prove that the computer has no possible network path or that the operating
system is uncompromised.

This decision was superseded by ADR-008 at the user's explicit request.

---

## ADR-007 — Minimal GUI

Decision:

Use Slint with the software renderer for the first desktop GUI. Its
declarative layout provides system-font rendering, standard controls, and
responsive sizing without custom pixel drawing. The GUI displays warnings and
the generated mnemonic, but provides no text input, clipboard action, logging,
or storage.

The existing terminal workflow remains available through `--cli`.

---

## ADR-008 — Do not block generation when online

Decision:

Do not inspect local network interfaces or refuse mnemonic generation merely
because networking is enabled. The application still performs no network
operations and retains all other security behavior.

This explicitly supersedes ADR-006.

The decision applies to both the CLI and the GUI. It does not authorize any
network communication; OpenSeed continues to perform no network operations.

---

## ADR-009 — Stage 3 boundary

Decision:

Stage 3 is complete with the Slint GUI and responsive layout. No Stage 4
functionality is to be started implicitly. Mouse, touchpad, controller and
other user-generated entropy remain outside the current implementation until
explicitly requested.

---

## ADR-010 — 6×4 mnemonic grid and Rust-side labels

Decision:

Display the 24-word mnemonic as 24 individual cells arranged in six columns
and four rows. Format each cell label in Rust, including its two-digit position
and BIP-39 word, before passing the data model to Slint.

Reason:

The complete mnemonic must remain visible and readable in the GUI. Rust-side
formatting avoids rendering Slint binding expressions as literal text and
keeps the presentation change separate from mnemonic generation and BIP-39
validation.

---

## ADR-011 — Stage 4 pointer movement input

Decision:

Support optional mouse/touchpad pointer movement as a user-selected input
source in the GUI. The user may select system RNG, pointer movement, or both.
Collect at most 1024 distinct rounded pointer positions, require all 1024
samples when movement is selected, and provide a reset action.

Selected inputs are combined with a domain-separated SHA-256 digest before
32-byte entropy is passed to the existing BIP-39 implementation. The system
RNG remains the default selection.

Reason:

This provides the smallest auditable Stage 4 implementation while avoiding a
false claim that a count of human pointer events proves a specific amount of
entropy. Movement-only mode is therefore explicitly experimental. Controller
input is not claimed because no controller-specific backend has been added.

---

## ADR-012 — Honest movement feedback

Decision:

Do not display a numeric value described as collected entropy bits for pointer
movement. Instead, display detected movement, the number of distinct samples,
and a progress indicator toward the technical minimum of 1024 samples.

Reason:

Pointer-event counts and observed coordinate variation do not by themselves
provide a defensible lower bound on min-entropy. A bit count would be
misleading without a validated statistical model. The GUI therefore gives
clear operational feedback without overstating the security properties of
human-generated movement.

---

## ADR-013 — Explicit entropy-source explanations

Decision:

Label the two Stage 4 source controls explicitly as `SYSTEM SECURE RANDOM
GENERATOR` and `MOUSE / TOUCHPAD MOVEMENT`. Display a short explanation below
each control, mark the system CSPRNG as recommended, and identify movement as
experimental.

Reason:

The source controls must be understandable without relying on context or
technical knowledge. Users must know which source is selected and that
movement-only generation does not provide a defensible measurable entropy
guarantee.

---

## ADR-015 — Static voluntary donation information

Decision:

Place a `SUPPORT OPENSEED` button below `GENERATE MNEMONIC`. It expands static
Bitcoin mainnet and Lightning donation addresses taken from `PROJECT.md`.
The GUI must not fetch, replace, copy, transmit, or track donation information.

Reason:

The project requirements define voluntary donations as independent from seed
generation. Static display keeps the feature offline and avoids introducing
network, clipboard, payment, or donor-tracking behavior.

---

## ADR-016 — Copying public donation addresses only

Decision:

Display each public donation address in a read-only selectable `LineEdit` with
a dedicated `COPY` button. Mouse selection and `Ctrl+C` are supported for
these public addresses. No copy control or selectable text is provided for the
generated mnemonic.

Reason:

The user explicitly requested convenient copying of public donation data.
Limiting clipboard access to static public addresses preserves the security
boundary around the generated mnemonic.

---

## ADR-018 — High-contrast donation address fields

Decision:

Render donation addresses in dark, high-contrast selectable text fields. Label
the on-chain address as a Bitcoin payment in BTC and the Lightning address as
a payment in satoshi.

Reason:

The standard widget palette rendered the address text white on a white
background. A dedicated dark field preserves readability while keeping mouse
selection, `Ctrl+C`, and the explicit public-address-only copy boundary.

---

## ADR-017 — Default entropy source selection

Decision:

Select system CSPRNG, mouse/touchpad movement, and physical dice by default.
The user may uncheck any source before generating.

Reason:

The requested default makes the complete local entropy workflow visible and
requires the user to provide movement and dice input unless they intentionally
change the selections. The system CSPRNG remains selected as the recommended
source.

---

## ADR-014 — Unbiased six-sided dice extraction

Decision:

Support physical six-sided dice as an optional source. Record each result via
dedicated GUI buttons rather than free-form keyboard input. Group results into
8-roll base-6 blocks. Accept only values below 2^20 and reject the remaining
range, producing 20 unbiased bits per accepted block. Require 13 accepted
blocks before producing 256 bits of dice-derived input.

Selected dice input is domain-separated and mixed with other selected sources
using SHA-256 before BIP-39. Rejected blocks are not reused; the user must make
additional physical rolls.

Reason:

Rejection sampling avoids modulo bias in the conversion from six-sided rolls.
Dedicated result buttons ensure that keyboard typing is not treated as an
entropy source. Dice-only security still depends on the physical dice being
fair and the user recording their results honestly.

---

## ADR-019 — Support/About controls below the result

Decision:

Expose the Support and Version/About buttons both in the pre-generation
controls and directly below the generated 6×4 mnemonic grid. Use one shared
details component so the two panels retain mutually exclusive visibility in
both locations.

Reason:

The actions remain discoverable after generation without duplicating panel
content or changing mnemonic handling.

---

## ADR-020 — Conditional visibility of dice input

Decision:

Show the physical dice input panel only when the physical dice entropy source
checkbox is selected. The panel includes the roll buttons, status, guidance,
and reset action.

Reason:

The interface should not display controls for an entropy source the user has
explicitly disabled, matching the existing behavior of the mouse movement
collection panel.

---

## ADR-021 — Restarting from the generated result

Decision:

Provide a centered `START OVER` button between Support and Version/About in
the generated-result view. It returns to the source-selection screen and
clears the mnemonic model, movement samples, dice rolls, counters, errors,
and open information panels.

Reason:

The user can start a new generation workflow without closing and reopening
the application, while the previous sensitive result is removed from the UI
state before the next workflow begins.

---

## ADR-022 — Informational passphrase guidance only

Decision:

Implement Stage 6 as a red `PASSPHRASE INFO` button above the mnemonic grid,
with its details panel below the result. The panel explains the difference
between a 24-word wallet and a 24-word wallet with a passphrase, the
backup-protection benefit, and the risk of permanent loss after forgetting or
mistyping the passphrase.

OpenSeed does not provide a passphrase input, store it, copy it, or derive a
seed from it. The user enters a passphrase directly into a compatible wallet
or hardware device.

Reason:

OpenSeed is intentionally limited to generating and displaying the BIP-39
mnemonic. Avoiding passphrase collection keeps the secret boundary small and
prevents the application from implying that a passphrase is a mandatory
“25th word”.

---

## ADR-024 — Separate mnemonic verification workflow

Decision:

Add a dedicated verification mode with 24 editable fields in a 6×4 layout.
Validate that every field contains one word from the official English BIP-39
list, then validate the complete mnemonic checksum using the `bip39` crate.
Show only a success or error status. The BACK action clears the entered words
and returns to the start screen.

Reason:

Verification must be separate from generation and must not create a second
copying, persistence, or logging path for the user's handwritten mnemonic.

---

## ADR-025 — Local release evidence before public distribution

Decision:

Record local Stage 8 evidence in `RELEASE.md`, including release test results,
the macOS arm64 artifact type, ad-hoc signature status, and SHA-256 hash.
Treat the artifact as a verification output, not as a public release.

Reason:

The project can verify a reproducible local build workflow without claiming
that a single-platform, ad-hoc-signed binary has completed independent
security review, vulnerability auditing, notarization, or public release
requirements.

---

## ADR-026 — MIT license and explicit user responsibility

Decision:

Publish the project under the MIT license in `LICENSE-MIT`. State in the
README that the software is provided "AS IS", without a warranty, and that
the user is responsible for the operating environment, network isolation,
mnemonic backup, device security, and consequences of using the generated
mnemonic.

Reason:

The project should remain easy to reuse while making its lack of warranty and
the user's responsibility explicit. The notice does not claim that a license
text replaces an independent security review or legal advice.

---

## ADR-023 — Informational local network status

Decision:

Use the operating system's local interface list (`getifaddrs`) to display an
informational `ONLINE` or `OFFLINE` status in the GUI. Treat an active,
non-loopback IPv4 or IPv6 interface as `ONLINE`. Provide a manual refresh
action and recommend disconnecting before generation.

The check performs no DNS, HTTP, API, telemetry, or other external request,
and it never blocks mnemonic generation.

Reason:

The user should be informed about the local network state while retaining
control over generation. Local interface inspection is only a useful signal;
it cannot prove that a compromised operating system has no network access.
