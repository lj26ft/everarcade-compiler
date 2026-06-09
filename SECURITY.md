# Security Policy

## Reporting vulnerabilities

Please report suspected vulnerabilities privately to the maintainers before public disclosure. If no dedicated security inbox is configured for your fork or organization, contact the repository owner/maintainer through the least-public available channel and mark the report as security-sensitive.

Include:

- affected component or path;
- reproduction steps;
- expected and actual impact;
- whether secrets, keys, settlement records, replay evidence, or generated packages are involved;
- suggested mitigation, if known.

## Scope

In scope:

- runtime package validation issues;
- deterministic execution or replay-verification bypasses;
- Creator SDK package-generation vulnerabilities;
- dependency/vendor supply-chain risks;
- scripts that mishandle secrets or generated evidence;
- documentation that could cause unsafe production claims.

Out of scope for v0.1 readiness claims:

- production hosting guarantees;
- public-testnet uptime;
- commercial payment operations;
- XRPL/Xaman/GPU/marketplace production behavior, which remains scaffold-level unless separately proven.

## Response expectations

Maintainers should acknowledge credible reports within a reasonable period, triage severity, identify affected versions or commits, and coordinate a fix or mitigation before public details are published. Because this repository is currently an open-source candidate and not production ready, response timelines are best-effort rather than SLA-backed.

## Safe handling

Do not submit secrets, private keys, wallet credentials, or production operator configuration in issues, pull requests, reports, or validation artifacts.
