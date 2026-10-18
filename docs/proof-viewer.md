# Local proof fixture viewer

Experimental read-only display for the existing synthetic Receipt MMR V1 vectors and
`world.evr.receipt_proof.v1` JSON. Python 3 standard library only; no installation,
server, JavaScript, outbound network, signing or production operation.

From the repository root, create a disposable output and open it in your local browser:

```sh
viewer_output=$(mktemp -d)
python3 tools/proof-viewer/viewer.py test-vectors/commitments/receipt-mmr-v1.json > "$viewer_output/index.html"
# Open "$viewer_output/index.html" with your browser's File > Open command.
python3 -m unittest discover -s tests/proof-viewer -v
```

To inspect a standalone proof, supply its explicit local UTF-8 JSON filename instead.
The tool reads only that path (maximum 2 MiB), writes HTML to stdout and a display
status to stderr. Keep output separate from the input; shell redirection onto the
input would truncate it before Python runs. Exit 0 means recognized display labels,
1 means refused/malformed/unsupported display with original text retained, and 2
means unreadable, oversized or non-UTF-8 input. None is a verification verdict.

Profile/version/world/index/count/root/siblings/peaks and checkpoint data are shown
as supplied, including missing fields. Unknown data stays visible as escaped original
text. Duplicate keys, invalid JSON and non-finite numbers receive the refused label.
All imported text is escaped; there are no links, scripts, imported assets or automatic
file/URL lookups. The HTML Content Security Policy prohibits resource loading. Input
bytes and repository fixtures are never rewritten. Raw original text is shown in a
collapsed details panel; expand it to inspect unknown fields.

Pretty-printed numbers may normalize their spelling. Numbers that overflow,
underflow, or change decimal value in the display representation are refused;
their original text remains available. This bounds display fidelity and does not
validate receipt field types or proof semantics.

## Limits and source contract

Every page is **UNVERIFIED**. Recognizing labels does not check field types, hashes,
root mathematics, world/tick/sequence binding, peak placement, checkpoint custody or
any other verifier rule. No reference helper is invoked. Full verification is blocked
on a separately reviewed implementation of the existing normative rules and an
accepted authenticated checkpoint policy; this viewer defines neither.

The source `expected: pass` labels are not viewer verdicts. The eight negative vector
rows are descriptive placeholders, not eight executable hostile proof cases. The
empty vector is a display fixture, not a verification result. The viewer is bounded
to local synthetic development; do not upload secrets or private/production receipts.

Existing source authority remains [Proof Format V1](specs/PROOF_FORMAT_V1.md),
[Receipt MMR V1](specs/RECEIPT_MMR_V1.md) and
[Commitment Architecture V1](specs/COMMITMENT_ARCHITECTURE_V1.md). The fixture is
`test-vectors/commitments/receipt-mmr-v1.json` at source checkpoint
`33aa49b51d2095d2154049761d4b1734d9f85fc7`. Do not run the reference generator to view
fixtures: its main rewrites the tracked vectors even with `--help`.
