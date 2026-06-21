# Open Source Readiness

EverArcade's documentation is public first. The source repository is planned for public release, but the repository must pass security, license, dependency, generated-artifact, and release-readiness review before it is presented as an available open-source project.

## Current public state

- The documentation portal is live now.
- The public website explains worlds, developers, operators, players, continuity, and roadmap direction.
- Direct website CTAs should describe the repository as **coming soon** until the repository is actually public.
- The repository remains an open-source candidate rather than a production-ready public release.

## Why the repository is not public yet

A public release needs more than uploading code. EverArcade must complete a readiness pass that covers:

1. security review and secret scanning,
2. license and reuse terms,
3. dependency and vendoring policy,
4. generated-artifact cleanup,
5. scaffold/runtime maturity labels,
6. contributor support boundaries,
7. release notes that match testnet readiness, and
8. clear instructions for what new contributors can safely run.

## Security audit first

The security audit is required before public repository release. It should verify that no private credentials, production secrets, unsafe defaults, misleading claims, or unsupported operational paths are being published as if they are ready for production or public testnet use.

## Timing is tied to readiness and testnet maturity

The repository release is planned, but timing should follow readiness rather than a fixed marketing date. Public release should align with the security audit, license review, dependency policy, and the maturity of the local/testnet workflows that contributors are expected to use.

## What contributors can do meanwhile

Prospective contributors can follow the public docs and roadmap while the repository is prepared:

- read the [concept documentation](/docs/concepts/what-is-a-world),
- study the [sovereign worlds concept](/docs/concepts/sovereign-worlds),
- review the [developer path](/docs/developers),
- follow the [operator path](/docs/operators), and
- track roadmap language for readiness and testnet milestones.

## Release message

EverArcade is preparing the repository for public release. The documentation portal is live first. The open-source repository will follow after the security, license, and release audit is complete.
