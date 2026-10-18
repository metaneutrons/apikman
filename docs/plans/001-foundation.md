# Initiative plan: a recoverable credential vault with controlled tool access

Epic: tracking setup pending repository creation.
Decision state: initial design and bootstrap authorized by Fabian on 2026-10-18.
Product milestones below specify acceptance; their implementation and publication
are separate deliveries. This plan is the requirements authority, not a progress
ledger.

## Outcome and baseline

Provide a Rust CLI that stores API credentials locally, recovers independently of
one device, synchronizes encrypted state without silent loss, and later offers
bounded operations to AI tools without returning raw credentials. The measured
baseline on 2026-10-18 was an empty directory with no Git repository or remote.

Initial targets: macOS arm64/x86_64 and Linux GNU arm64/x86_64. Windows, shared team
vaults, arbitrary credential-refresh integrations and clipboard support are out of
scope. macOS local Keychain and Linux Secret Service are convenience providers;
portable passphrase and recovery unlock remain mandatory. iCloud unlock is an
opt-in feature with a separate two-device qualification requirement.

## Design decisions

- [Existing tools](../security/existing-tools.md), especially fnox, substantially
  overlap with this proposal. Before implementing new vault persistence, review
  a synthetic-data evaluation and evidence-based gap assessment with the owner.
  The bootstrap is useful independently of that implementation choice.
- The [threat model](../security/threat-model.md) defines trust boundaries and
  limitations. Its security claims are requirements until evidenced.
- Start with an authenticated encrypted file, not a general-purpose SQL engine.
  Separate storage, domain records, unlock providers, transport backends and CLI.
  Cryptographic suite candidates are fixed in the threat model; a reviewed binary
  format and limits precede persistence code.
- Use stable record IDs and unique aliases. Metadata includes service, credential
  type, account/organization, optional project/environment/API base URL, structured
  scopes, tags, comment, UTC creation/modification/optional-expiry and status.
  Scopes document provider permissions; local broker policy enforces restrictions.
- OS stores hold random convenience unlock keys bound to vault ID and generation.
  They do not hold the master passphrase. Vault transfer and unlock-key transfer
  are separate flows. Do not implement a custom Linux key-sync service in v1.
- Backups are retained encrypted snapshots. Multi-writer sync initially detects
  conflicts and requires explicit resolution. No timestamp-based last-writer wins.
- Trusted subprocess injection precedes a separate restricted broker. No generic
  shell tool can satisfy the broker security claim.
- Hardened gitrepo profile, GPL-3.0-or-later, Rust edition 2024 and pinned Rust
  1.99.1. Initially configure GitHub Releases only; crates.io, Homebrew, AUR and APT
  require separate channel decisions. No production release is authorized here.

## Delivery and acceptance

### M0: Reviewable design and hardened repository foundation

Execution: tracking setup pending repository creation. Dependencies: none.

- M0-A1: Threat model covers T01–T08, explicit non-goals, recovery, metadata,
  Keychain/iCloud and Linux boundaries; review against the user requirements.
- M0-A2: Rust 1.99.1 workspace builds with a committed lockfile, inherited strict
  lint policy and unsafe-code prohibition. Help/version/status work; status and
  README clearly identify unimplemented vault and unlock capabilities. CLI
  integration tests exercise actual binaries, success and invalid usage.
- M0-A3: Format, pedantic/nursery/cargo Clippy with warnings denied, nextest,
  minimum 80% line coverage, cargo-deny, hook validation, full-history secret scan
  and workflow checks pass. A clean native CI run covers all four target hosts.
- M0-A4: GitHub settings and active branch/tag rules match the hardened standard;
  require the real successful aggregate CI check only after it is observed.
  Publication secrets are environment-scoped; Release Please uses a scoped App
  token, draft releases and synchronized manifest/lockfile versions. Read back
  settings with the gitrepo checker and document unsupported provider features.
- M0-A5: Release workflow separates unprivileged build from signing/publication,
  enforces exact candidate inventory, dependency-complete SPDX SBOMs, checksums,
  keyless signatures and attestations. Clean extraction smoke tests and local
  positive/negative gate probes pass. A nonpublishing workflow qualification
  exercises native build/sign/verify stages on the exact commit. Public staging
  and promotion remain unqualified until a separately authorized release run.
- M0-A6: README, contributor/security guidance, execution issues and bootstrap
  report distinguish verified results from pending native/platform/release gates.
  No real vault, provider credential or user Keychain item is touched by bootstrap.

### M1: Local encrypted vault and independent recovery

Execution: tracking setup pending repository creation. Dependencies: M0.

- M1-A1: Reviewed, versioned format specifies authenticated fields, suite IDs,
  KDF defaults/minimums/maximums, salts/nonces, key-slot limits, length limits and
  compatibility rules. Known-answer vectors plus wrong-key, bit-flip, truncation,
  header substitution, resource-exhaustion and fuzz cases fail safely.
- M1-A2: Init/add/list/show/update/remove support the agreed record schema and
  multiple isolated vaults. Duplicate aliases are rejected; stable identity
  survives renaming; expiry/disabled-state policy is explicit and tested.
- M1-A3: All metadata and secret fields are encrypted at rest. A synthetic
  canary is absent from persistent files and ordinary stdout/stderr/diagnostics.
  Reveal/export are explicit. Secret input uses hidden terminal input or an
  explicit stream; no secret-valued CLI argument is supported.
- M1-A4: Passphrase and offline recovery each unlock on a fresh account without
  the originating OS key store. Wrong input preserves the original state.
  Passphrase changes and DEK rotation have separate tests and documented backup
  consequences. Recovery material is never stored only in the protected vault.
- M1-A5: Local permissions, locking, symlink defenses, durable atomic replacement
  and failure recovery are verified with concurrent writers and fault injection
  on supported native filesystems. Failures yield a complete old or new vault;
  no plaintext temporary artifact is produced.
- M1-A6: Trusted exec supplies only explicitly mapped credentials to the intended
  child without shell interpolation or secret arguments; signal/exit propagation
  and environment behavior are tested. A synthetic child that echoes its input
  demonstrates the documented disclosure limitation. No AI isolation claim.

### M2: Native convenience unlock and optional Apple device synchronization

Execution: tracking setup pending repository creation. Dependencies: M1.

- M2-A1: macOS local Keychain enrollment/read/removal works through native APIs,
  scoped by vault ID/generation and the supported signed identity. Refused access,
  locked/unavailable keychain, missing entry and binary upgrades are tested.
  No broad all-applications ACL and no raw secret command arguments.
- M2-A2: Linux Secret Service enrollment/read/removal is qualified with an actual
  GNOME Keyring provider and a compatible KWallet provider. Locked collection,
  cancelled prompt, missing service and unavailable session bus fail clearly.
  Headless passphrase/recovery operation needs neither D-Bus nor a desktop.
  Provider-specific evidence is named; one provider does not qualify the other.
- M2-A3: iCloud mode is explicitly opt-in, uses supported synchronizable Keychain
  APIs and documents required signing/entitlements/access-group identity. Tests
  refuse unsupported combinations and report local enrollment separately from
  synchronization. Local-only fallback requires a distinct explicit choice.
- M2-A4: Two physical Macs with the supported signed distribution, same enabled
  iCloud Keychain account and separately transferred encrypted vault demonstrate
  retrieval/unlock without transferring the recovery secret or passphrase. Test
  delayed/offline sync and incompatible identity. Record OS/build identities and
  limitations without personal identifiers or secret data. Until this evidence
  exists, advertise iCloud mode as experimental/unqualified.
- M2-A5: Independent recovery works after loss of all enrolled devices. Enrollment
  removal, key generation change and the limits of device revocation/old backups
  are documented and tested without claiming retroactive remote erasure.

### M3: Encrypted backup and conflict-safe remote synchronization

Execution: tracking setup pending repository creation. Dependencies: M1; M2 is
not required for portable cloud backup or restore.

- M3-A1: Backend contract defines immutable writes, conditional update semantics,
  object size limits, integrity, credential bootstrap and retries. Qualify a local
  filesystem backend and one S3-compatible backend by name/version. Backends
  lacking proven compare-and-swap support cannot enable multi-writer sync.
- M3-A2: Push/pull and two offline writers never silently lose an update. Same-
  and different-record conflicts, delete-versus-edit, interruption, duplicate
  requests and stale remote heads are covered by integration tests. Preserve both
  divergent snapshots until explicit resolution; use ancestry rather than clocks.
- M3-A3: Authenticated revision ancestry and local checkpoints detect known
  rollback. Fresh-device/no-checkpoint limitations are documented and tested.
  Tombstone retention/compaction has a defined stale-device admission rule that
  prevents record resurrection.
- M3-A4: Retention and backup integrity are independent of synchronization.
  Restore to a clean account validates before activation and uses independently
  recoverable backend credentials. Tampered/missing objects preserve local state.
- M3-A5: Multiple configured destinations have independent, machine-readable
  outcomes; partial failure is a non-success exit with recoverable retry state.
  Only encrypted artifacts reach storage, logs and recorded network fixtures.

### M4: Restricted credential broker for AI tools

Execution: tracking setup pending repository creation. Dependencies: M1, M2 local
unlock paths; remote synchronization is not a prerequisite.

- M4-A1: A separate threat-model review defines the exposed transport, caller
  authentication, local approval channel and provider operation schemas. No
  arbitrary exec, raw reveal/export, unlock-key retrieval or policy-edit operation
  exists in the restricted interface.
- M4-A2: Deny-by-default policy binds operation, validated arguments, credential,
  method/path and exact HTTPS origin. Integration tests through the real
  transport reject prompt-injection attempts, caller-supplied approvals, header
  overrides, cross-origin redirects, SSRF/DNS-rebinding and policy changes through
  synced metadata. Out-of-policy operations never reach a test provider.
- M4-A3: Approvals bind to canonical requests and expire; replay or changed
  arguments fail. Use disabled/expired-key policy and provider-side scopes
  independently. Audit events contain bounded non-secret facts only.
- M4-A4: Synthetic credential canaries are absent from tool definitions, request
  transcripts, broker stdout/stderr and approved response schemas. Deliberately
  reflected credentials and malformed provider responses fail closed. Document
  residual response/content risks and same-user unrestricted-shell limitations.

## Release, migration and verification

No existing data needs migration at bootstrap. Future format changes require
read compatibility policy, encrypted backup before migration and atomic
activation; never silently migrate an unsupported format or overwrite a divergent
remote head. Older clients must reject formats they cannot preserve safely.

Use focused local tests during iteration and Linux for portable development
gates. Initial shared/toolchain setup and named qualification points cover all
four native targets. Native OS-store and physical two-Mac tests remain separate
from mocked CI. Public package publication is outside this request; configured
release paths must nevertheless fail closed until their inputs and exact
candidate qualify. No deadlines or runner-time estimates are asserted.

## Decision changes

2026-10-18: initial baseline distinguishes local native unlock from optional
iCloud synchronization and explicitly retains headless Linux recovery. GitHub
tracking links will be added after actual issue IDs exist.

2026-10-18: in response to the owner's existing-tool question, document fnox and
require a reuse/gap decision before implementing new vault persistence. No
existing product acceptance criterion is waived by that comparison.
