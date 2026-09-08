# OpenSeed release verification

This file records the local Stage 8 verification evidence. It is not a public
release announcement. The macOS package is ad-hoc signed for local integrity
verification, not signed with an Apple Developer certificate.

## Verification performed

Environment:

- macOS Apple Silicon
- `rustc 1.97.1`
- `cargo 1.97.1`
- Rust release profile with LTO, one codegen unit, and stripped symbols

Commands:

```sh
cargo fmt --check
CARGO_NET_OFFLINE=true cargo test --release
CARGO_NET_OFFLINE=true cargo build --release
target/release/openseed --help
codesign --verify --verbose=2 target/release/openseed
plutil -lint target/release/macos-arm64/OpenSeed.app/Contents/Info.plist
codesign --verify --deep --strict --verbose=2 target/release/macos-arm64/OpenSeed.app
```

Results:

- formatting check passed,
- 9 release tests passed,
- release binary built successfully,
- CLI help completed successfully,
- macOS ad-hoc signature verified,
- macOS application bundle metadata and signature verified.

## Local artifact

The locally verified artifact is:

```text
target/release/openseed
```

The prepared macOS application package is:

```text
target/release/macos-arm64/OpenSeed.app
target/release/OpenSeed-macos-arm64.zip
```

Architecture:

```text
Mach-O 64-bit executable arm64
```

SHA-256:

```text
71ab7166ac194732462f9ad72e78263c32fa0eb6885b9e2f2b43707e3317726c  target/release/macos-arm64/OpenSeed.app/Contents/MacOS/openseed
c689cc621bbdea94b3b3d11d404927b502339d4066fc062433cd4d79a952e96d  target/release/OpenSeed-macos-arm64.zip
```

The signature is ad-hoc (`TeamIdentifier` is not set). Code signing with a
developer certificate, notarization, cross-platform artifacts, and public
distribution are not completed. The source code is licensed under MIT; the
software is provided "AS IS" and the user is responsible for its use, as
described in `LICENSE-MIT` and `README.md`.

## Security and release limitations

- `cargo-audit` and `cargo-deny` were not installed in the local environment;
  dependency review was performed with the locked Cargo dependency tree.
- The local network indicator is a heuristic based on active non-loopback
  interfaces. It does not prove that a compromised system has no network path.
- Movement-only entropy is experimental and has no defensible bit estimate.
- Dice-only entropy depends on fair physical dice and honest recording.
- Secrets are not intentionally persisted, logged, transmitted, or copied by
  application actions. Memory zeroization is not claimed against a fully
  compromised operating system, debugger, malware, or crash dump.
- Independent security review remains required before public release.
