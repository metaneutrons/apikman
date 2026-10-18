# Security policy

This project is an early development scaffold, not a qualified credential vault.
Do not put real credentials in fixtures, issues, logs or pull requests.

## Reporting

Use [GitHub private vulnerability reporting](https://github.com/metaneutrons/apikman/security/advisories/new).
Include affected version/commit, a minimal reproduction using synthetic data,
impact and platform information. Do not include live credentials or recovery keys.
The maintainer aims to acknowledge reports within seven days; this is not a
service-level guarantee.

No stable version is currently supported. Once releases exist, the latest stable
release will receive security fixes; older releases must be upgraded unless an
explicit maintenance branch is announced.

## Design and qualification

Read the [threat model](docs/security/threat-model.md) and the
[acceptance plan](docs/plans/001-foundation.md). An unlocked same-user process,
trusted subprocess or compromised host can disclose supplied credentials. OS
key-store unlock is convenience, not proof of a human-approved tool operation.

Release verification must bind signatures and attestations to this repository,
the reviewed release workflow and the exact immutable tag. A matching checksum
alone does not establish publisher identity. Release verification instructions
will accompany the first separately authorized distribution.
