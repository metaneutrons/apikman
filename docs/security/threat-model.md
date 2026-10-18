# Threat model

Version: 1.0, 2026-10-18. Status: design baseline; controls described here are
requirements, not claims that the bootstrap implements a vault.

## Purpose and security claims

apikman is a local, offline-capable credential vault for a single user, with
encrypted backup and deliberate multi-device synchronization. It supports trusted
human use and, later, a restricted credential broker for AI tools. Personal,
company and institutional credentials can reside in separate vaults with separate
keys. Sharing a vault with a team is outside the initial scope.

The intended guarantees are confidentiality and integrity of secrets and metadata
at rest, recoverability without the original device, detection of conflicting
writes, and bounded credential use through a broker. An operating-system key store
provides optional convenience unlock. It does not establish who is asking an AI
tool to perform an operation.

## Assets and boundaries

| Asset | Location and protection boundary |
| --- | --- |
| Credential values and metadata | Encrypted vault payload; plaintext only within an unlocked process and approved recipients |
| Random vault data-encryption key (DEK) | Memory while unlocked; persistent copies only wrapped in authenticated key slots |
| Passphrase-derived wrapping key | Argon2id; transient memory, never an OS-key-store item |
| Recovery secret | High-entropy offline material, separate from the vault and backup access credentials |
| Convenience unlock key | Random, scoped to vault ID and key generation; held by an explicitly enrolled OS key store |
| Backup access credentials | Explicitly configured, least privilege; independent recovery route |
| Broker policy and approvals | Trusted local configuration and approval channel, never caller-supplied authority |
| Release artifacts and updates | Reviewed source, protected GitHub workflows, checksums, signatures and provenance |

The filesystem, cloud storage, OS key store, child processes, remote API services,
AI context and software supply chain are separate trust boundaries. A cloud
provider receives encrypted vault objects, not decryption material. The encrypted
vault must still be transferred to a second Mac; iCloud Keychain synchronization
of an unlock item does not synchronize the vault itself.

## Adversaries and assumptions

In scope: a thief obtaining a locked vault or cloud backup; a malicious storage
provider modifying, replaying, withholding or deleting objects; concurrent and
offline writers; malformed imports and remote objects; prompt injection attempting
to misuse the broker; accidental disclosure through diagnostics, process arguments
or tool transcripts; unauthorized release modification.

Trusted: the running OS, cryptographic random source, selected cryptographic
libraries, the explicitly enrolled key-store provider, authorized devices, and
the implementation of approved provider operations. API services necessarily
receive their own credentials when used. Their compromise and provider-side
revocation are outside local storage guarantees.

An administrator, a compromised kernel, process-memory inspection, or an
unrestricted process running as the same user may obtain unlocked secrets.
Arbitrary shell access combined with convenience unlock cannot provide strong
isolation from an AI agent. Physical memory, swap, hibernation and SSD remanence
cannot be eliminated by Rust memory zeroization. These are stated limitations,
not implied protections.

## Threats and required controls

### T01: Offline disclosure or ciphertext tampering

Encrypt the complete record collection, including service names, URLs, scopes,
comments and tombstones. Use maintained authenticated-encryption implementations;
the proposed suite is XChaCha20-Poly1305 with independently generated nonces.
Use Argon2id for passphrase wrapping, with a random salt and versioned parameters.
The exact wire format, parameter values and bounded resource limits must be
reviewed before persistence is implemented. No custom cipher construction.

Bind the vault identity, format version, revision and relevant envelope fields
cryptographically. Reject unknown mandatory algorithms, tampered headers, wrong
keys and truncated objects before releasing plaintext or replacing local state.
Bound lengths, key-slot count and KDF costs before allocation or key derivation.
All valid unlock slots give access to the same DEK: a weak passphrase slot remains
an offline attack route even if another slot uses a strong OS key store.

The outer envelope unavoidably reveals format/KDF parameters and an opaque vault
identifier. Object size, access times, traffic volume and retained snapshot count
remain observable; traffic-analysis resistance is not claimed.

### T02: Unlock loss, key confusion or convenience-store compromise

Use separate authenticated slots for passphrase, recovery and optional convenience
unlock. Bind each OS-store entry to the vault ID and key generation. Never store
the passphrase or raw record collection in an OS key store. Enrollment is explicit
and takes place only after successful independent unlock and recovery setup.
Changing a passphrase rewraps a slot; rotating a DEK is a separate operation.

On macOS, local Keychain is the initial convenience provider. Optional iCloud
Keychain enrollment uses Apple's synchronizable item APIs only where supported by
the signed distribution's keychain model, entitlements and access group. It must
report requested mode and observed local enrollment separately. Local API success
does not prove synchronization to another device. No silent downgrade from
synchronizable to local-only storage. Never grant unrestricted access to all apps.

On Linux desktops, use the freedesktop Secret Service API via the session bus
(for example GNOME Keyring or a compatible KWallet configuration). Availability,
collection locking and prompting vary by provider. Do not invoke a shell helper
with the key as an argument or silently fall back to a plaintext file. Headless
Linux must retain passphrase/recovery operation without D-Bus or a desktop. The
Secret Service contract does not provide portable cross-device synchronization.

Cloud-synchronized unlock extends trust to the Apple account, its recovery
mechanisms and every authorized device that can obtain the item. Losing a device
requires provider/account remediation and potentially DEK rotation. Removing a
local item cannot retract a secret already obtained on another device or from an
old backup. Cross-platform enrollment uses an independent recovery/passphrase
unlock, not access to Apple's synchronization internals.

### T03: Local crash, concurrent modification or filesystem attack

Restrict vault directories and files to the owning user (0700/0600 on supported
Unix filesystems). Validate ownership and file type; refuse unsafe symlink and
replacement paths. Serialize local mutations, write encrypted temporary state,
flush it, atomically replace the vault and flush its parent directory where the
platform permits. Never decrypt into a temporary file. Recovery must yield the
complete old or new state, never acknowledge an uncommitted mutation. Document
filesystems where equivalent durability cannot be established.

### T04: Cloud overwrite, replay, deletion or unavailable backend

Treat remote storage and its object names as untrusted input. Use encrypted,
immutable snapshots plus backend-specific conditional reference updates. Never
claim safe multi-writer sync for a backend without proven conditional-write
semantics. Do not compare wall-clock timestamps to select a winner.

Track authenticated revision ancestry and locally remembered high-water marks.
Detect stale ancestors and divergence without automatically discarding a side.
Concurrent edits initially require explicit conflict resolution. Deletions have
tombstones with a defined retention/compaction policy; a returning old device must
not resurrect a deleted record. Multiple destinations have independent success
and failure reporting. Sync is not a backup retention policy.

Authenticated encryption does not establish freshness on a new device with no
trusted checkpoint. After loss of all checkpoints, rollback to an old valid
snapshot may be undetectable. Retained independent backups and an offline recovery
record mitigate loss; a malicious provider can still deny service. Restoring must
not require credentials stored only inside the inaccessible vault.

### T05: Accidental secret disclosure

Read secrets through hidden terminal input or an explicit input stream, never a
secret-valued flag or argument. Normal list/show/status and structured output
contain metadata only. Reveal and plaintext export require explicit human-facing
operations; reveal is unavailable through the restricted broker. Avoid shell
expansion and interpolation. Never log secrets, authorization headers, request
URLs containing credentials, key material or plaintext vault contents. Restrict
debug formatting and minimize/zeroize owned secret buffers where feasible.

An explicit human reveal intentionally discloses to its output recipient and may
enter terminal scrollback. Clipboard support, if later added, needs a separate
decision because history managers and platform synchronization can retain copies.
It is not part of the first delivery.

### T06: AI credential extraction or excessive authority

Subprocess injection is for trusted commands. The child and its descendants can
read or disclose their supplied credentials through output, files or the network.
Environment injection is not sandboxing. The documentation must demonstrate this
limitation using harmless synthetic data. An arbitrary shell/exec interface is
never exposed as the restricted AI broker.

The later broker resolves opaque references and permits locally configured,
provider-specific operations. Bind the exact HTTPS origin, allowed method/path,
validated arguments, authentication placement and redirect policy. Reject caller
overrides of credential headers, proxy destinations and policy. Deny cross-origin
redirects and unintended loopback/private-network requests; address DNS rebinding
at the actual connection boundary. Provider response bodies remain untrusted and
may contain reflected credentials or prompt injection. Only reviewed response
schemas are exposed, and no general claim of perfect response redaction is made.

Human approvals originate in a trusted local channel, bind to the canonical
operation/arguments/credential and expire. A model-supplied `approved=true` is
never authority. Local policy changes, reveal, export, enrollment and recovery are
outside this API. Imported metadata, comments and synced files cannot expand
broker permissions. Default-deny behavior must be tested at the exposed transport,
not only in a policy helper.

### T07: Confusing storage metadata with provider enforcement

Provider scopes are structured metadata; broker restrictions are separate local
policy. Expiry may be unknown. A locally expired/disabled credential is refused by
default in use operations, but the provider may still accept it. Deleting a record
does not revoke a provider credential, erase cloud history or invalidate copies
on another device. Provider revocation and automatic token refresh are explicit,
provider-specific future work. Imported metadata never proves actual privileges.

### T08: Supply-chain compromise and false qualification claims

Pin Rust 1.99.1 in one toolchain file and commit Cargo.lock. Forbid unsafe code in
project crates by default; review narrow platform-boundary exceptions separately.
Require lint, tests, coverage, advisory/license checks, full-history secret scans
and protected PR integration. Pin Actions to commit SHAs. Release jobs have least
privilege and environment-scoped credentials; PR jobs have no publication secrets.

Build each supported native target once. Qualify the extracted distribution,
produce dependency-complete SBOMs, keyless signatures, checksums and GitHub
attestations. Verify the exact downloaded candidate before making a prerelease
visible and verify public bytes before promotion. No package channel or stable
release is implied by this bootstrap. An untested signing, restore, OS-store or
two-device path must remain explicitly unqualified.

## Recovery and operational consequences

A user must retain both an independent way to reach a backup and an independent
way to decrypt it. Test recovery on a clean account without the original Keychain
or Secret Service collection. Validate a restored snapshot before activation;
preserve the old local state until activation succeeds. Document how restoring an
older snapshot affects deleted records, credentials rotated at the provider and
known revision checkpoints.

## Review triggers and evidence

Review this model when adding an unlock provider, changing signing identity or
keychain access groups, changing the vault format/KDF, enabling a cloud backend,
adding a broker transport, or introducing provider refresh/revocation. Acceptance
criteria live in [the initiative plan](../plans/001-foundation.md). Current
execution and evidence belong in linked milestone issues and the bootstrap report.

Primary cryptographic references:
[RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html) and
[libsodium's XChaCha20-Poly1305 documentation](https://doc.libsodium.org/secret-key_cryptography/aead/chacha20-poly1305/xchacha20-poly1305_construction).
