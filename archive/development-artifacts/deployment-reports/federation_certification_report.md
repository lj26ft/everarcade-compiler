# Federation Certification Report

## evidence
- Two-node join certification and runtime federation tests executed by this script.

## test coverage
- implemented: in-process checkpoint join, reconstruction, replay/root comparison, peer health equivalence, checkpoint distribution, corruption detection.
- scaffold: distributed transport routing, replay stream propagation, federation topology recovery.
- placeholder: public federation, EverNode operator registration, live WAN fault injection.

## known limitations
- Certification is deterministic and local; no public federation or production deployment is claimed.

## remaining scaffolds
- External peer discovery.
- Multi-operator trust policy.
- Live federation transport and storage.

## next risks
- Federation status must remain honestly classified until cross-host evidence exists.
