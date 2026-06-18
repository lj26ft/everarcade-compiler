# UTF-8 Byte-Lexicographic Ordering Fixture

Canonical dynamic object keys are sorted by raw UTF-8 bytes, not by locale, ICU collation, UTF-16 code units, case folding, or natural-language ordering.

## Fixture keys

| Key | UTF-8 bytes | UTF-16 code units | Note |
| --- | --- | --- | --- |
| `A` | `41` | `0041` | Uppercase ASCII. |
| `a` | `61` | `0061` | Lowercase ASCII. |
| `é` | `c3 a9` | `00e9` | Precomposed Latin small e with acute. |
| `` | `ee 80 80` | `e000` | Private-use BMP code point. |
| `𐀀` | `f0 90 80 80` | `d800 dc00` | Supplementary-plane Linear B code point. |

## Expected canonical order

```json
["A","a","é","","𐀀"]
```

## Why this differs from other sorting modes

* Locale sorting may place `é` near `e` or treat accents as secondary weights.
* ICU collation can vary by locale and collation strength.
* Case-folded sorting may treat `A` and `a` as equal or reorder them by tiebreaker rules.
* UTF-16 sorting may place `𐀀` before `` because its first surrogate `d800` is less than `e000`, while UTF-8 byte ordering places `` (`ee 80 80`) before `𐀀` (`f0 90 80 80`).

The fixture `canonical-fixtures/fixture-003-state.json` includes these keys in `metadata.extensions` and in an entity `attributes` object. A conforming canonicalizer emits them in the expected canonical order above.
