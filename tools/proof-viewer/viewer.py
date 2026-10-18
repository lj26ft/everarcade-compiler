#!/usr/bin/env python3
"""Render explicit local JSON as inert HTML; never verify or accept proofs."""
import argparse
import html
import json
import sys
from pathlib import Path

MAX_BYTES = 2 * 1024 * 1024
NOTICE = ('UNVERIFIED — display only. No proof verification, reference calculation, '
          'checkpoint authentication, independent acceptance or production claim.')
FIELDS = ('version', 'commitment_profile', 'world_id', 'receipt_index',
          'receipt_count', 'leaf_hash', 'receipt_root', 'checkpoint_hash',
          'siblings', 'peaks')


def escaped(value):
    return html.escape(json.dumps(value, ensure_ascii=True, indent=2), quote=True)


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('duplicate JSON field')
        result[key] = value
    return result


def reject_constant(value):
    raise ValueError('non-finite JSON number')


def render(raw):
    """Return HTML and display-read status, which never signifies verification."""
    status = 'REFUSED: malformed or unsupported input; raw text retained below.'
    sections = []
    try:
        value = json.loads(raw, object_pairs_hook=unique_object,
                           parse_constant=reject_constant)
        if not isinstance(value, dict):
            raise ValueError('expected JSON object')
        version = value.get('version')
        if version == 'world.evr.receipt_mmr_vectors.v1' and isinstance(value.get('vectors'), list):
            status = 'UNVERIFIED: synthetic vector collection; expectations are source labels only.'
            for row in value['vectors']:
                if not isinstance(row, dict):
                    raise ValueError('expected vector object')
                title = row.get('name', '(unnamed)')
                note = ('DESCRIPTIVE NEGATIVE ONLY: not a serialized hostile proof test.'
                        if row.get('expected') == 'fail' else 'UNVERIFIED fixture data')
                sections.append('<section><h2>' + escaped(title) + '</h2><p>' + note + '</p>')
                # Preserve the existing fixture wire format; do not infer proof counts or verdicts.
                sections.append('<pre>' + escaped(row) + '</pre></section>')
        elif version == 'world.evr.receipt_proof.v1' and value.get('commitment_profile') == 'world.evr.commitment.v1':
            status = 'UNVERIFIED: recognized proof labels; fields have not been validated.'
            sections.append('<dl>')
            for field in FIELDS:
                sections.append('<dt>' + field + '</dt><dd><pre>' +
                                (escaped(value[field]) if field in value else 'MISSING') + '</pre></dd>')
            sections.append('</dl>')
    except (ValueError, RecursionError):
        sections = []
        status = 'REFUSED: malformed or unsupported input; raw text retained below.'
    document = ('<!doctype html><html lang="en"><head><meta charset="utf-8">'
                '<meta name="viewport" content="width=device-width, initial-scale=1">'
                '<meta http-equiv="Content-Security-Policy" content="default-src \'none\'; '
                'base-uri \'none\'; form-action \'none\'">'
                '<title>EverArcade local proof fixture viewer</title></head><body>'
                '<h1>Local proof fixture viewer</h1><p>' + NOTICE + '</p><p>' + status + '</p>' +
                ''.join(sections) + '<details><summary>Original input text (unverified)</summary><pre>' +
                html.escape(raw, quote=True) + '</pre></details></body></html>')
    return document, status


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('input', type=Path, help='explicit local UTF-8 JSON file, at most 2 MiB')
    args = parser.parse_args(argv)
    try:
        # Only this operator-selected path is read. Embedded paths/URLs are inert strings.
        with args.input.open('rb') as source:
            data = source.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES:
            raise ValueError('input exceeds 2 MiB limit')
        raw = data.decode('utf-8')
    except (OSError, ValueError) as error:
        print('Input refused: ' + str(error), file=sys.stderr)
        return 2
    document, status = render(raw)
    sys.stdout.write(document + '\n')
    print(status, file=sys.stderr)
    return 0 if status.startswith('UNVERIFIED:') else 1


if __name__ == '__main__':
    sys.exit(main())
