# apikman

A Rust CLI for a local encrypted API credential vault, recoverable cloud backups
and controlled credential use by tools.

**Development scaffold. Do not use this build to store credentials.** The current
binary reports its version and capabilities. Encryption, CRUD, native unlock,
cloud synchronization and the AI broker are specified but not implemented.

## Build and inspect

Rust 1.99.1 is pinned in `rust-toolchain.toml`; rustup selects it automatically.
There is no published release or package-manager installation yet.

```sh
cargo build --locked
cargo run --locked -- --help
cargo run --locked -- --version
cargo run --locked -- status
cargo run --locked -- status --json
```

Supported target platforms are macOS (Apple Silicon and Intel) and Linux GNU
(arm64 and x86_64). Native CI and release qualification are tracked separately
from local development checks.

## Planned security model

- Encrypt secrets and metadata together. Keep scopes separate from comments and
  from locally enforced tool policy.
- Recover using a passphrase or an independent offline recovery secret.
- Optionally enroll a random convenience unlock key in macOS Keychain or Linux
  Secret Service. Headless Linux retains passphrase/recovery operation.
- Offer opt-in iCloud Keychain unlock only after qualifying the exact signed
  distribution and a real two-Mac flow. The vault itself needs separate transfer.
  Linux Secret Service does not define a portable equivalent for device sync.
- Detect cloud conflicts; never silently choose the newest wall-clock timestamp.
- Distinguish trusted subprocess credential injection from a restricted AI broker.
  A child process receiving a credential can disclose it. Unrestricted same-user
  shell access cannot be isolated by convenience unlock.

Read the [threat model](docs/security/threat-model.md),
[native unlock decision](docs/security/native-unlock.md) and
[acceptance criteria and delivery plan](docs/plans/001-foundation.md).
These describe intended behavior; `apikman status` describes this build.

[Existing-tool research](docs/security/existing-tools.md) identifies substantial
overlap with fnox. A reuse/gap assessment precedes any new vault implementation.

## Development and releases

See [CONTRIBUTING.md](CONTRIBUTING.md) for checks and hooks and
[the repository standard](docs/repository-standard.md) for hardened CI/release
policy and qualification boundaries. GitHub Releases is the initial distribution
channel; other package channels are unconfigured. Release qualification runs do
not authorize a public release.

Report vulnerabilities through the route in [SECURITY.md](SECURITY.md).

## License

Copyright (C) 2026 Fabian Schmieder. Licensed under GPL-3.0-or-later; see
[LICENSE](LICENSE).
