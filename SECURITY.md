# Security policy

Please report a vulnerability privately, through GitHub's
[private vulnerability reporting](https://github.com/rsmb7z/javaid/security/advisories/new), not
in a public issue. Reports are answered as soon as possible; a fix ships as a new patch release on
crates.io, and the advisory is published once it is out.

Fixes go into the latest release only.

## How releases are published

Every version on crates.io is built and published by `.github/workflows/release.yml` from a `v*`
tag on `main`, through crates.io trusted publishing (no long-lived token exists), after a
maintainer approves the run. crates.io records the repository, workflow run and commit of each
version, so a published crate can be traced to the exact source it was built from.
