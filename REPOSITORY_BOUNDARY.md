# Repository Boundary

This repository is the open-source reference implementation for `world.evr`.

It includes deterministic artifact generation, local verification, CLI verification, attestations, trust-root validation, payload binding, fixtures, must-fail cases, and reference examples needed to understand and reproduce the `world.evr` artifact model.

It does **not** include EverArcade's future commercial registry service, hosted operator platform, vault or treasury products, paid verification service, marketplace operation, enterprise integrations, pricing, partner strategy, institutional plans, or private business strategy.

## In scope

- `world.evr` artifact format and reference examples.
- Deterministic World Factory behavior.
- Local verifier and CLI verifier behavior.
- Attestation, payload binding, and trust-root validation docs and fixtures.
- RC1/RC2 independent review history and public release notes.
- Neutral local scaffolds used to exercise deterministic verification paths.

Independent reproducibility review documentation is available in:

- VERIFICATION.md

## Out of scope

Economic and treasury systems are outside the scope of this open-source reference implementation.

Implementations may attach non-authoritative economic metadata, but live payments, custody, accounting, treasury services, and commercial settlement are not implemented here.

Marketplace systems are outside the scope of this repository.

This repo may include deterministic package or capability metadata examples, but commercial marketplace operation, royalties, settlement, reputation, provider rewards, and hosted marketplace services are not implemented here.

Hosted operator infrastructure is outside the scope of this open-source reference implementation.

Any operator-related files in this repo are local/reference scaffolds and are not a production hosted platform, SLA service, or commercial operator network.

Registry material in this repository is limited to neutral world identity, metadata, and local validation examples.

Hosted discovery, ranking, reputation, curation, and commercial registry operation are outside the scope of this open-source reference implementation.

This repository includes local verification and certification primitives.

Hosted verification services, public badge programs, reviewer marketplaces, application workflows, and paid certification products are outside the scope of this open-source reference implementation.

This repository does not implement custody, wallet management, live settlement, or legal ownership certification.

XRPL/Xahau material is limited to deterministic anchoring, boundary modeling, or local test scaffolds unless explicitly stated otherwise.

Enterprise integrations and commercial partnerships are outside the scope of this repository.

This repository documents public technical compatibility only.
