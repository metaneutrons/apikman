# Repository standard

Profile: hardened. Language: Rust, edition 2024. Toolchain: 1.99.1, selected only
through `rust-toolchain.toml`. License: GPL-3.0-or-later. Owner: Fabian Schmieder,
GitHub namespace `metaneutrons`. Initial package channel: GitHub Releases only.
The crate is not published to crates.io (`publish = false`).

## Development gates

The workspace forbids project unsafe code and inherits Clippy all, pedantic,
nursery and cargo lint groups. CI denies warnings, uses locked dependency
resolution, nextest and an 80% minimum line-coverage gate. cargo-deny checks
advisories, sources and licenses. Hooks provide fast feedback; required CI and
PR-only protected integration enforce the checks. Secret scanning covers the full
Git history. Actions are pinned to commit SHAs and updated through Dependabot.

The initial native qualification matrix covers macOS arm64/x86_64 and Linux GNU
arm64/x86_64 because this bootstrap changes shared toolchain/build inputs. Routine
portable development should use the cheapest sufficient Linux gate; changing the
selection requires tested component classification and a stable aggregate check.
Native OS-store enrollment and physical two-device sync require separate tests.

## Release contract

Release Please is the sole automatic production-tag source. It uses an explicitly
repository-scoped GitHub App token in the protected `release` environment. Version
changes update the workspace version and lockfile on the release PR branch;
drafts prevent premature public visibility. The bootstrap manifest is 0.0.0 so
the first release PR can propose 0.1.0; qualification builds use the Cargo version.
No seed crate has been published.

The hardened pipeline is dispatch-only. Its default mode qualifies the source
commit without creating tags or GitHub Releases. Publication requires explicit
enablement, an immutable version tag and the protected environment. Exact native
payloads must pass extraction smoke, dependency-complete SPDX SBOM checks,
keyless Cosign verification, GitHub provenance verification and candidate
inventory/checksum checks. Linux compile containers are digest-pinned; packaging
and signing run outside them. Public release bytes must match the admitted
candidate before stable promotion. No optional channel credentials are configured.

macOS unsigned qualification does not qualify Developer ID signing, notarization,
restricted entitlements, local Keychain access or iCloud synchronization.
Production macOS distribution must meet its separately enforced signing inputs.
No production tag, public artifact release or package publication is authorized by
the bootstrap request.

## Provider and policy limitations

Personal-account GitHub plans do not expose non-provider secret-pattern and
validity-check features available through organization Secret Protection.
Document those as unavailable, not enabled based on a successful PATCH response.

The existing shared Release Please App can mint credentials for other installed
repositories. Each workflow token must be narrowed to `apikman`, but that does
not narrow the reach of the long-lived App private key itself. Its custody is an
explicit existing-infrastructure dependency; a dedicated App would reduce that
trust boundary and requires its own setup decision.

## Evidence

Actual local, GitHub and release-qualification results are recorded in
[the bootstrap report](bootstrap-report.md) and the execution issues linked from
[the initiative plan](plans/001-foundation.md). Passing a local checker does not
establish native CI, publication or real Keychain/iCloud behavior.
