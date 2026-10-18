# Existing-tool assessment

Research date: 2026-10-18. This is a documented capability comparison, not a
security audit or a successful integration qualification.

## fnox

[fnox](https://github.com/jdx/fnox) is an existing MIT-licensed Rust secret manager.
Its documented capabilities substantially overlap with the proposed product:
encrypted local values through age, native macOS Keychain and Linux Secret
Service, multiple remote providers, offline encrypted caching, profiles and
subprocess credential injection. Its proxy issues placeholders to the child and
substitutes real credentials in approved HTTPS requests using domain, header,
method and path rules. The proxy explicitly does not claim same-user OS isolation.

Important distinctions to test before choosing a storage implementation:

- age-encrypted values live in TOML configuration. That does not establish that
  all metadata is encrypted, as required for the proposed vault.
- Documented sync pulls provider values into a local encrypted cache; it is not
  evidence for multi-destination vault backup or concurrent multi-writer merge.
- The keychain provider documents local OS storage; optional iCloud-synchronizable
  unlock and its exact signing/packaging requirements are not qualified here.
- Structured provider scopes, creation/modification/expiry and lifecycle semantics
  must be compared against the complete proposed record schema.
- MCP `get_secret` returns raw values. MCP `exec` can return transformed or
  transmitted secrets despite literal output redaction. Neither meets the
  restricted broker guarantee. The credential proxy is the relevant comparison.

A bounded evaluation should use synthetic credentials and isolated configuration
to test local encryption/metadata exposure, native-store behavior, offline
recovery, child-process injection, provider sync semantics and proxy bypass
cases. Its result may justify using fnox directly, contributing upstream, building
an adapter, or retaining only requirements for which a verified gap exists.
No fnox source is incorporated into this repository by this research.

Sources:

- [Provider catalog](https://fnox.jdx.dev/providers/overview)
- [OS keychain provider](https://fnox.jdx.dev/providers/keychain)
- [Age provider](https://fnox.jdx.dev/providers/age.html)
- [Sync semantics](https://fnox.jdx.dev/guide/sync)
- [Credential proxy and limits](https://fnox.jdx.dev/guide/proxy)
- [MCP access model](https://fnox.jdx.dev/guide/mcp.html)

## Other approaches

[KeePassXC](https://keepassxc.org/docs/KeePassXC_GettingStarted) provides a local
encrypted KDBX database with metadata, cross-platform applications and a CLI.
Storing a database on cloud storage does not by itself qualify our concurrent
sync and recovery requirements.

[1Password CLI](https://developer.1password.com/docs/cli/secrets-scripts) resolves
vault references and injects credentials into subprocess environments. Its vault
service is a different storage/deployment model from an independently managed
local vault with selectable backup destinations.

[Bitwarden Secrets Manager CLI](https://bitwarden.com/help/secrets-manager-cli/)
supports secret/project CRUD, machine-account access and subprocess injection.
It likewise provides a service-oriented alternative rather than proof of the
proposed local-first backup architecture.

## Decision boundary

Complete the authorized design/CLI/CI bootstrap. Before implementing a new vault
format or cryptographic persistence, review an evidence-based fnox gap assessment
with the owner. The current design remains a requirements baseline; it does not
establish that developing a new vault is preferable to reuse.
