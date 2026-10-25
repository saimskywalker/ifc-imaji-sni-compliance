# Security policy

## Supported versions

`imaji-sni-compliance` is not yet published to crates.io. Only the latest `0.1.x`
on `main` gets security fixes.

| Version | Supported |
|---|---|
| 0.1.x (`main`) | yes |

## Reporting a vulnerability

Please do not open a public issue for a vulnerability.

Report it privately through GitHub's private vulnerability reporting: open the
[Security tab](https://github.com/saimskywalker/ifc-imaji-sni-compliance/security)
and choose "Report a vulnerability". Only the maintainer can see the report.

This crate is meant to read IFC models that may come from someone else, so
the interesting cases are input that makes it panic, loop, or allocate
without bound, and input that makes a rule report a pass for something it did
not evaluate.

Please include:

- the smallest input or code snippet that shows it
- what happened (the panic and backtrace, or how long it ran) and what you
  expected
- the crate version or commit, and `rustc --version`

You should hear back usually within a week. Once a fix is ready it is merged
and the advisory is published, with credit to you unless you prefer otherwise.
