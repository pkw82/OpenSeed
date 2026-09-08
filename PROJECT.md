# OpenSeed

## Project Overview

OpenSeed is a small, free and open-source offline application for
generating BIP-39 wallets.

The primary purpose of OpenSeed is to allow a user to generate a
24-word English BIP-39 mnemonic in an offline environment and safely
write it down for later wallet recovery.

OpenSeed is a security-sensitive project.

The project prioritizes:

- simplicity,
- transparency,
- minimal code,
- minimal dependencies,
- auditability,
- correct cryptography,
- no unnecessary features,
- no network communication during generation,
- no persistence of generated secrets.

The project is independent from Orion AI Trading System.

---

# Core Concept

OpenSeed generates a 24-word English BIP-39 mnemonic.

The user may choose which entropy sources are used.

There are three possible entropy sources:

1. System cryptographically secure random number generator (CSPRNG)
2. User-generated mouse/touchpad/controller movement
3. Physical dice rolls

The user must be able to choose:

- system CSPRNG only,
- mouse/touchpad/controller only,
- dice only,
- any combination of two sources,
- or all three sources.

The application must never force the user to use a particular optional
entropy source.

The available entropy sources should be treated as independent inputs
that are cryptographically combined before the final 256-bit entropy is
passed to BIP-39.

The exact cryptographic construction must be reviewed before
implementation.

Do not invent a custom cryptographic primitive.

Use established cryptographic constructions and well-established
libraries whenever possible.

---

# BIP-39

The primary target is:

- 256-bit entropy
- 24-word mnemonic
- English BIP-39 wordlist

The application must generate a valid BIP-39 mnemonic with the correct
checksum.

The implementation must be tested against official BIP-39 test vectors.

The project should use a well-established BIP-39 implementation rather
than unnecessarily implementing BIP-39 from scratch.

Do not implement BIP-39 seed derivation unless explicitly requested in a
later version.

---

# BIP-39 Passphrase

OpenSeed must eventually support an optional user-defined BIP-39
passphrase.

The user may choose to enter their own passphrase.

The passphrase:

- is separate from the 24-word mnemonic,
- must never be stored,
- must never be logged,
- must never be copied automatically to the clipboard,
- must never be transmitted over the network,
- must exist only for as long as required by the application.

The application must clearly explain that the BIP-39 passphrase creates
a different wallet from the same mnemonic.

The user should be warned:

"Your mnemonic and passphrase are separate secrets."

"Loss of either one may result in permanent loss of access to the
wallet protected by them."

"Do not use a password that you use anywhere else."

"Do not use personal information, dates, names or predictable phrases."

The passphrase should be entered only after the mnemonic generation
process has completed.

Keyboard input is allowed for entering a user-selected BIP-39
passphrase.

Keyboard input must never be used as an entropy source.

---

# Entropy Sources

## 1. System CSPRNG

The system cryptographically secure random number generator is one of
the supported entropy sources.

Use the operating system's secure randomness facilities.

Do not implement a custom PRNG.

Do not use:

- system time,
- process ID,
- MAC address,
- CPU temperature,
- filesystem timestamps,
- predictable environmental values

as a primary source of cryptographic entropy.

The implementation should use a well-established cryptographic random
source provided by the operating system through a trusted library.

---

## 2. Mouse / Touchpad / Controller

The application may collect entropy from:

- mouse movement,
- touchpad movement,
- controller analog movement or equivalent pointing input.

The keyboard must not be used as an entropy source.

The application must not ask the user to type random characters.

The application must not claim that a specific number of mouse events
equals a specific number of entropy bits unless this can be
cryptographically justified.

Human-generated movement entropy is difficult to quantify reliably.

If an entropy estimate is implemented, it must be conservative and
must not give the user a false sense of security.

The application may require the user to generate additional movement
until the selected entropy collection method satisfies the project's
defined security requirements.

If sufficient entropy cannot be established reliably, the application
must not falsely claim that sufficient entropy has been collected.

---

## 3. Dice

The application may support physical dice as an additional entropy
source.

Dice input must be treated as user-provided entropy.

The application should support standard physical dice, for example:

- six-sided dice,
- multiple dice rolls.

The implementation must define how dice results are converted into
unbiased entropy.

Do not assume that simply concatenating decimal dice results is
automatically an optimal entropy encoding.

Avoid modulo bias where applicable.

Dice are completely optional.

The user may choose dice as:

- the only entropy source,
- one of multiple entropy sources,
- or not use dice at all.

---

# Entropy Source Selection

The final application must allow the user to select the entropy
sources.

Examples:

SYSTEM RNG

SYSTEM RNG + MOUSE

SYSTEM RNG + DICE

MOUSE + DICE

SYSTEM RNG + MOUSE + DICE

MOUSE ONLY

DICE ONLY

The application must not force the system RNG, mouse, or dice to be
used if the user intentionally chooses another supported configuration.

However, the application must clearly explain the security properties
and limitations of each configuration.

A configuration must never be presented as secure merely because it
contains many inputs.

The cryptographic combination of the selected sources must be designed
so that the security properties of the combination are clearly
understood.

The combiner must be reviewed before implementation.

---

# Offline Requirement

OpenSeed is an offline-oriented seed-generation application.

The application should inform the user when local network interfaces appear
to be active and recommend disconnecting before generation. This status is
informational; the application must not block generation based on it.

Before allowing generation, the application must check the local
network state.

The application must not perform an external network request in order
to determine whether the Internet is available.

It must not use:

- HTTP,
- HTTPS,
- DNS,
- external APIs,
- cloud services,
- telemetry endpoints,
- analytics services,
- remote servers.

The application should determine network state using local operating
system information.

If an active network connection is detected, the user should receive a clear
warning that disconnecting is recommended before generation.

The application should not attempt to disable the network itself.

The user is responsible for disconnecting the computer.

The network check is a safety mechanism, not proof that a compromised
operating system cannot access the network.

---

# No Persistence

OpenSeed must never intentionally save generated wallet secrets.

The following must never be written to disk:

- mnemonic,
- BIP-39 passphrase,
- derived seed,
- raw entropy,
- intermediate secret material.

The application must not:

- create automatic backups,
- write logs containing secrets,
- write secrets to temporary files,
- store secrets in application caches,
- send secrets over the network,
- automatically copy secrets to the clipboard.

The mnemonic must not be recoverable from OpenSeed after the generation
session has ended.

The application should attempt to clear sensitive material from
memory when it is no longer required.

The documentation must clearly state that memory zeroization cannot
protect against a fully compromised operating system, debugger,
malware, crash dump or similar privileged attacker.

---

# User Warnings

After generating the mnemonic, OpenSeed must clearly warn the user that
the displayed words are extremely sensitive.

The application should display warnings similar to:

"THIS MNEMONIC WILL NOT BE SAVED."

"OpenSeed does not store your mnemonic."

"After closing this application, OpenSeed cannot recover these words."

"Write the words down before continuing."

"Store the backup in a secure place."

"Do not take a screenshot."

"Do not photograph the screen."

"Do not copy the mnemonic to the clipboard."

The exact wording may be improved during UX development, but the meaning
must remain clear.

---

# Wallet Recovery Recommendation

OpenSeed should strongly recommend that the user independently verify
the written backup before depositing significant funds.

The application should recommend:

1. Write down the 24 words.
2. Verify the written words.
3. Close OpenSeed.
4. Independently restore the wallet using the written mnemonic.
5. Confirm that the wallet can be recovered correctly.
6. Only then consider using the wallet for significant funds.

The application should explain that a backup that has never been tested
should not be assumed to be reliable.

A future version may provide a dedicated mnemonic verification mode.

---

# Loss of Access

OpenSeed must clearly explain that losing the mnemonic may result in
permanent loss of access to the wallet.

If a BIP-39 passphrase is used, losing the passphrase may also result in
permanent loss of access to the wallet protected by that passphrase.

OpenSeed cannot recover lost mnemonic words or passphrases.

There must be no implication that OpenSeed can restore lost secrets.

---

# Liability / Disclaimer

OpenSeed should include a clear disclaimer.

The application and documentation should communicate that the author
and contributors are not responsible for:

- lost mnemonic words,
- lost BIP-39 passphrases,
- lost wallet access,
- lost cryptocurrency,
- incorrect backups,
- user mistakes,
- hardware failures,
- operating-system compromise,
- malware,
- misuse of the software,
- losses resulting from incorrect wallet recovery.

The exact legal wording should be reviewed before final release.

Do not make claims that exceed what can reasonably be supported.

---

# Open Source

OpenSeed must be open-source software.

The complete source code should be publicly available.

The project should not depend on proprietary cloud services.

Users should be able to inspect the source code and, where practical,
build the software themselves.

The project should prioritize reproducible and verifiable builds where
practical.

The final release should include appropriate integrity information such
as cryptographic hashes.

Code signing should be considered for platform releases.

---

# Voluntary Bitcoin Donations

OpenSeed is free to use.

There must be:

- no purchase price,
- no subscription,
- no required donation,
- no premium version,
- no features locked behind donations.

Donations are completely voluntary.

If the user does not donate anything, OpenSeed must work exactly the
same way.

OpenSeed should provide two voluntary support methods.

## Bitcoin on-chain

Address:

bc1qwqeegd32w5wrgwx05t6ag6902egrmr7j22hku8

Network:

Bitcoin mainnet

## Bitcoin Lightning

Lightning Address:

burlyfoxglove7800@cake.cash

These addresses are static project information.

They must not be downloaded from the Internet.

They must not be obtained from an API.

They must not be dynamically replaced by a remote server.

The application must not track whether a donation was made.

The application must not identify donors.

The application must not send donation notifications.

The donation system must have no relationship to the cryptographic
generation process.

Preferred terminology:

- Donation
- Voluntary donation
- Support OpenSeed
- Support development

Avoid:

- Payment
- Purchase
- Fee
- Required contribution
- Subscription

Suggested message:

"OpenSeed is free and will always be free."

"If you find it useful and would like to support development, you can
donate a few sats."

"No donation is required."

"Don't have anything to give? That's perfectly fine — use OpenSeed and
stay safe."

---

# Development Philosophy

OpenSeed must be developed incrementally.

Do NOT implement the complete application in one step.

Each stage must produce a working and testable result before the next
stage begins.

The main development principle is:

"A small amount of boring, auditable code is better than a large amount
of clever code when the program generates private keys."

Do not implement features "for the future".

Do not build the GUI before the underlying cryptographic functionality
has been tested.

Do not add unnecessary dependencies.

Do not add unnecessary abstractions.

---

# Development Roadmap

## Stage 1 — Minimal CLI

First implement the smallest working version.

Requirements:

- Rust project
- OS CSPRNG
- 256-bit entropy
- BIP-39
- English wordlist
- 24-word mnemonic
- terminal output
- no persistence
- no clipboard
- no network communication
- tests

The initial version should use only the system CSPRNG.

Do not implement mouse entropy.

Do not implement dice.

Do not implement GUI.

Do not implement donations.

Do not implement additional wallet features.

The purpose of Stage 1 is to verify the fundamental cryptographic
pipeline.

---

# Stage 2 — Offline Status Information

After the basic mnemonic generation is tested, implement an informational
offline-status check.

Requirements:

- local network-interface inspection,
- no external connectivity test,
- no DNS,
- no HTTP,
- no API,
- display an `ONLINE` or `OFFLINE` status without blocking generation.

Add automated tests where practical.

Document the limitations of the network check.

---

# Stage 3 — Basic GUI

After the CLI implementation and offline mechanism are working and
tested, introduce a minimal graphical interface.

The GUI should initially provide only the functionality already
validated by the CLI.

Do not introduce new cryptographic functionality at this stage.

The GUI must preserve:

- offline operation,
- no persistence,
- no clipboard,
- clear warnings,
- safe display of the mnemonic.

---

# Stage 4 — Mouse / Touchpad / Controller Entropy

After the GUI and basic generation system are stable, add optional
user-generated movement entropy.

The user must be able to choose whether to use it.

Possible configurations include:

- system RNG only,
- mouse/touchpad/controller only,
- system RNG + movement,
- other supported combinations.

Do not make unsupported claims about entropy quantity.

Do not display an "entropy percentage" unless a defensible entropy
estimation method has been established.

The user must be able to restart the collection process if the
collected input does not satisfy the required criteria.

Do not silently pretend that insufficient user input is sufficient.

---

# Stage 5 — Dice Entropy

After the movement entropy implementation has been tested, add physical
dice as another optional entropy source.

The user must be able to choose:

- system RNG,
- movement,
- dice,
- any supported combination.

Dice processing must avoid statistical bias.

Add appropriate tests.

---

# Stage 6 — Passphrase

Add BIP-39 passphrase support.

The user may:

- enter their own passphrase,
- or use a future secure passphrase-generation mechanism.

The passphrase must never be stored or transmitted.

The application must clearly explain that the passphrase creates a
different wallet.

---

# Stage 7 — Verification

Add a dedicated wallet mnemonic verification workflow.

The verification process must not persist the entered mnemonic.

The user should be able to verify that their handwritten backup can be
used to reconstruct the intended wallet.

---

# Stage 8 — Release Preparation

Only after the functionality has been implemented and independently
reviewed should the project prepare public releases.

Release preparation should include:

- complete automated test suite,
- security review,
- dependency review,
- source review,
- documentation,
- build instructions,
- release hashes,
- code signing where practical,
- reproducible-build documentation where practical.

---

# Development Rule

The current stage is the only stage that should be implemented unless
the user explicitly requests progression to the next stage.

When a stage is complete:

1. Run the relevant tests.
2. Review the implementation.
3. Update `.project/status.md`.
4. Record important architectural decisions in
   `.project/decisions.md`.
5. Stop.

Do not automatically continue to the next stage.

---

# Current Development Stage

Stage 1 — Minimal CLI.

The immediate task is to create a small, working and tested Rust CLI
that generates a 24-word English BIP-39 mnemonic using only the
operating system's cryptographically secure random number generator.

Do not implement Stage 2 or later.

Do not implement mouse entropy.

Do not implement dice.

Do not implement GUI.

Do not implement donations.

Do not implement passphrase support yet.

Do not implement future functionality "in preparation".

---

# Definition of Done — Stage 1

Stage 1 is complete only when:

- the Rust project builds successfully,
- 256-bit entropy is generated using the OS CSPRNG,
- a valid 24-word English BIP-39 mnemonic is generated,
- official BIP-39 test vectors are covered where applicable,
- the mnemonic is displayed to the user,
- the mnemonic is not written to disk,
- the mnemonic is not copied to the clipboard,
- no network operation is performed,
- relevant tests pass,
- the code is reviewed for unnecessary complexity,
- documentation explains the current limitations,
- `.project/status.md` is updated.

After completing these requirements, stop and wait for further instructions.

---

# Important Security Principle

Never claim that OpenSeed can protect secrets against a fully
compromised operating system.

OpenSeed is designed to minimize unnecessary exposure and persistence
of wallet secrets.

It cannot protect a user against malware, a compromised kernel,
malicious firmware, a malicious hypervisor, or an attacker who can
observe the screen or memory.

Security claims must always be proportional to what the implementation
can actually guarantee.

# Target Platforms

OpenSeed is intended to be a cross-platform application.

Final supported platforms:

- macOS
- Windows
- Linux

The application should provide equivalent core functionality on all
three platforms.

Platform-specific implementation details are acceptable where required,
but the core cryptographic logic should remain platform-independent.

Avoid unnecessary platform-specific code.

---

# Cross-Platform Development

The project should be developed with cross-platform compatibility in
mind from the beginning.

Do not implement functionality that works only on macOS unless it is
explicitly part of a temporary development step.

When platform-specific functionality is required, isolate it behind a
small and clearly defined abstraction.

Pay particular attention to platform differences involving:

- secure random number generation,
- network-interface detection,
- mouse and touchpad input,
- controller input,
- secure memory handling,
- GUI implementation,
- filesystem behavior,
- clipboard access,
- process termination,
- application packaging.

Do not assume that an API available on macOS is available on Windows
or Linux.

---

# Development Platform

Initial development may be performed on macOS.

The first prototype does not need to provide release binaries for all
platforms.

However, code should be written so that Windows and Linux support can
be added without redesigning the core architecture.

---

# Final Release Targets

The final OpenSeed release should provide builds for:

## macOS

Supported architectures should be determined during release
preparation, with Apple Silicon treated as an important target.

## Windows

Provide a native Windows build.

## Linux

Provide a native Linux build.

The exact distribution formats should be decided during the release
preparation stage.

Possible formats may include appropriate native or portable binaries,
but no specific packaging format is required at the current stage.

---

# Cross-Platform Testing

Before public release, OpenSeed must be tested on:

- macOS
- Windows
- Linux

Testing should cover at minimum:

- entropy generation,
- BIP-39 mnemonic generation,
- network isolation behavior,
- mnemonic display,
- absence of persistence,
- user input handling,
- GUI functionality,
- mouse/touchpad/controller entropy,
- dice input,
- passphrase handling.

Platform-specific failures must not be hidden or ignored.

---

# Development Roadmap — Platform Rule

The staged development process remains unchanged.

Stage 1 may be developed and tested primarily on the current development
machine.

Do not block Stage 1 by attempting to build the complete application for
all three platforms.

However, avoid architecture decisions that would unnecessarily prevent:

- macOS support,
- Windows support,
- Linux support.

Cross-platform release builds and full cross-platform testing belong to
the release-preparation stage.
