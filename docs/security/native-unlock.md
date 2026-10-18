# Native unlock decision

Decision date: 2026-10-18. Implementation status: planned; no OS-store enrollment
is performed by the bootstrap binary.

## Common contract

Native unlock is an optional alternative to entering a passphrase on each use.
Each vault has an independent high-entropy convenience unlock key and an
authenticated slot wrapping its DEK. The OS-store item contains that convenience
key, not the passphrase. Matching uses an opaque vault ID and key generation.
Recovery/passphrase slots remain independently usable, including on Linux without
a desktop. No cloud backup access credential lives only inside its own backup.

Expose capabilities separately: provider compiled, provider reachable, enrolled
mode, local item retrievable, and synchronization qualification. Never interpret
the existence of a keychain API as successful enrollment, or local enrollment as
evidence of another device receiving an item. Distinguish missing item, denied
access, cancelled prompt, unavailable service and unsupported configuration.

The default is no native enrollment. After independent vault unlock and recovery
setup, the user may explicitly select local or synchronizable enrollment. Never
silently change the chosen mode or store a fallback in a plaintext file. A native
store may prompt; noninteractive use must fail with a bounded error rather than
wait indefinitely. Transient keys and errors must not enter logs or arguments.

## macOS

Apple distinguishes the legacy file-based keychain from the Data Protection
Keychain. The former can provide local convenience using native Security APIs
and appropriately scoped application access. The latter is selected by
`kSecUseDataProtectionKeychain`, or by `kSecAttrSynchronizable` for synchronizable
items. iCloud Keychain synchronization requires the user's enabled account and
the correct login context; a daemon outside that context is not equivalent.

Synchronizable items cannot use `ThisDeviceOnly` accessibility classes or legacy
`kSecAttrAccess` ACLs. Use the supported access-group/access-control model instead.
Do not persist a keychain item reference as a cross-device identity; look it up
using the vault ID and generation. Do not put service names, accounts, record
aliases or comments into keychain item labels.

Data Protection Keychain access groups depend on signing entitlements authorized
by a provisioning profile. An ordinary standalone executable cannot embed that
profile in the same way as an app bundle. A Developer ID signature and
notarization do not themselves grant restricted keychain entitlements.

Therefore the initial implementation target is local Keychain unlock. A bounded
native spike must choose and verify a supported packaging model for optional
iCloud access, potentially a minimal signed app-like bundle/helper with an
authorized shared access group and a CLI entry point. If introduced, the helper
is an additional trusted process and needs authenticated, bounded IPC; it must
not become a general raw-key retrieval socket. Sharing an access group with
unrelated applications is outside the design.

The spike must test install, upgrade, signing identity, provisioning profile,
keychain choice, consent prompts and removal using the exact distribution. A
two-physical-Mac test then transfers only the encrypted vault through its normal
backup/sync route and retrieves the independently synchronized unlock item. The
test must not silently copy the passphrase or recovery material. Record delay,
offline behavior and unsupported configurations; do not promise immediate sync.

Deleting a synchronizable item can propagate to replicas, but cannot retract keys
already cached or copied. Account compromise and authorized device enrollment
expand the trust boundary. A lost device may require account remediation and a
new DEK; old snapshots remain decryptable with old material. Recovery must work
without access to the original Apple account.

Requiring user presence/biometric authentication on each retrieval is a separate
policy with different automation behavior. It must not be advertised as silent
noninteractive unlock.

## Linux

Use the freedesktop Secret Service API through the user's session D-Bus. Qualify
GNOME Keyring and a KWallet configuration implementing that API separately.
Collection locking, automatic login unlock and prompt behavior are service
decisions. A generic KWallet installation must not be assumed compatible without
checking its actual Secret Service interface.

Secret Service lookup attributes and labels are not secret and may be persisted
unencrypted. Store only an application namespace, opaque vault ID and key
generation there. Use the protocol's supported encrypted session where available;
it does not establish protection against a compromised session bus or same-user
process. Do not overstate the provider's application isolation.

The standard defines no portable device-synchronization or equivalent to iCloud
Keychain. For a second Linux or cross-platform device, restore/sync the encrypted
vault, unlock independently with the passphrase or offline recovery secret, and
explicitly enroll that device's local store. Copying an entire keyring file into a
cloud folder is not a supported synchronization mechanism.

No D-Bus, locked collections, rejected prompts and missing providers are ordinary
conditions. Headless operation must retain passphrase/recovery without installing
a desktop keyring. TPM-backed or hardware-token unlock may be considered later;
it is not part of the current security claim or recovery dependency.

## Qualification authority and sources

[M2 in the acceptance plan](../plans/001-foundation.md#m2-native-convenience-unlock-and-optional-apple-device-synchronization)
owns acceptance; this decision does not report successful implementation or a
two-device test.

- [Apple TN3137: On Mac keychain APIs and implementations](https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains)
- [Apple: kSecAttrSynchronizable](https://developer.apple.com/documentation/security/ksecattrsynchronizable)
- [Apple: kSecUseDataProtectionKeychain](https://developer.apple.com/documentation/security/ksecusedataprotectionkeychain)
- [Apple: Creating distribution-signed code for the Mac](https://developer.apple.com/documentation/xcode/creating-distribution-signed-code-for-the-mac/)
- [Apple: Signing a daemon with a restricted entitlement](https://developer.apple.com/documentation/xcode/signing-a-daemon-with-a-restricted-entitlement)
- [Apple: Sharing access to keychain items](https://developer.apple.com/documentation/security/sharing-access-to-keychain-items-among-a-collection-of-apps)
- [freedesktop Secret Service specification](https://specifications.freedesktop.org/secret-service/latest-single/)
- [GNOME libsecret Service](https://gnome.pages.gitlab.gnome.org/libsecret/class.Service.html)
- [KWallet introduction](https://docs.kde.org/stable_kf6/en/kwalletmanager/kwalletmanager/introduction.html)
