# Contributing

All repository content, commits, issues and pull requests are written in English.
Use Conventional Commits (`feat:`, `fix:`, `docs:`, `ci:`, etc.) and branch names
such as `feat/local-vault` or `docs/recovery`. Commit subjects are limited to 100
characters. Do not add AI attribution trailers or AI author identities.

## Setup and checks

Use rustup and the pinned toolchain. Install lefthook, gitleaks, cargo-nextest,
cargo-llvm-cov, cargo-deny and actionlint through their official distributions.
Python 3.11+ is required for release-policy tests. `make setup` installs hooks.

```sh
make setup
make check
```

Pre-commit checks inspect staged content, scan for secrets and check formatting.
Pre-push runs a bounded secret scan and the measured fast Rust checks. Cold
compilation can exceed the interactive budget: run `make check` first to warm the
cache rather than bypassing hooks. Full coverage, dependency checks and release
qualification belong in CI. Exceptions must be documented and tested; never
disable a gate merely to obtain a green run.

Use synthetic credentials only. Tests must not touch a developer's real Keychain,
Secret Service collection, cloud account or vault. Native store integration tests
will require explicit isolated test enrollment when implemented.

## Planning and evidence

Substantial changes follow [the versioned initiative plan](docs/plans/001-foundation.md).
The plan owns design, dependencies and acceptance criteria; the epic links
milestone issues, which own current progress and evidence. Do not copy acceptance
checklists into issues or close a milestone because a partial implementation PR
merged. Record actual revisions, platforms, negative cases and limitations.

Every new security or release gate needs a valid input and a deliberately invalid
isolated counter-probe. Keep expensive native qualification at named integration
points and qualify the exact release candidate before publishing it.

## Pull requests and releases

Use one kind of change per PR. CI checks the Conventional Commit PR title and
prohibits AI attribution in commits and PR bodies. The required aggregate check
must pass against current main; integrate serially where no qualified merge queue
exists. Release Please owns version/changelog changes and production tags.

Before merging a release PR, compare user-facing changes since the previous tag
with README and run its examples. Land README corrections in a separate docs PR.
Creating a release candidate or passing CI does not authorize publication.
