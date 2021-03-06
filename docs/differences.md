# Implementation differences

Lexing:

- Encoding
  - UTF-8 (wspace)
- LF
  - LF (wspace)
  - LF, CRLF
  - LF, CRLF, CR
- Encoding error laziness
  - lazy (wspace)
  - eager (Nebula, BlueSpace)

Parsing:

- Number `0`
  - requires sign and bits (conrad)
  - requires sign (wspace)
  - may omit sign (Nebula)
- Label `0`
  - requires bits (conrad)
  - empty allowed (wspace)
- Label leading zeros
  - leading zeros are unique (wspace)
  - leading zeros are ignored (Nebula)
- Syntax error laziness
  - lazy (wspace)
  - eager (Nebula, BlueSpace)
- Duplicate labels
  - error (Nebula)
  - use first (wspace)
  - use last (BlueSpace)
  - use next
  - use previous
  - use arbitrary

Assembly:

- Label value assignment
  - ASCII big-endian (wspace examples)
  - UTF-8 big-endian
  - Definition order
  - Usage order (tetris.ws)
  - Frequency order
- Label assignment includes leading zeros
  - yes
  - no
- Label character set
  - `\.?[A-Za-z_$][A-Za-z0-9_$]*|[0-9]+` (rdebath)

Bounds:

- Number bounds
  - arbitrary precision (wspace)
  - 64-bit integer (Nebula)
  - 32-bit integer
  - 64-bit float (wsjq)
- Label bounds
  - arbitrary length (wspace, Nebula)
  - 64-bit integer
  - 32-bit integer
  - 64-bit float

Heap:

- `store` at negative address
  - allowed
  - error
  - non-terminating (Nebula)
- `retrieve` at negative address
  - allowed
  - error
  - lazy error (wspace)
- `retrieve` at unset address
  - 0 (Nebula, wsjq)
  - 0 if under highest-set address; lazy error otherwise (wspace)
  - error

I/O:

- EOF behavior
  - error (wspace)
  - 0
  - -1
- Flushing
  - unbuffered, e.g. pi.ws (wspace)
  - flush before read, e.g. calc.ws
  - buffered (LOLCODE)

`readc`:

- Encoding
  - raw bytes ([wspace 0.3 binary])
  - UTF-8, error on invalid (wspace with modern GHC)
  - UTF-8, U+FFFD replacement char for invalid
  - UTF-8, -1 for invalid
- Line breaks
  - CRLF is collapsed to LF (wsjq on Windows)

`printc`:

- Encoding:
  - raw bytes ([wspace 0.3 binary])
  - UTF-8, error on invalid (wspace with modern GHC)
  - UTF-8, U+FFFD replacement char for invalid
- Valid ranges:
  - 0 to 255 ([wspace 0.3 binary])
  - 0 to U+D7FF and U+E000 to U+10FFFF (wspace with modern GHC)
  - 0 to U+10FFFF

`readi` number format:

- Leading space
  - allowed (wspace)
  - disallowed
- Trailing space
  - allowed (wspace)
  - disallowed
- Space between sign and number
  - allowed (wspace)
  - disallowed (BlueSpace)
- Positive `+` sign
  - allowed (BlueSpace)
  - disallowed (wspace)
- Hex with `0x` prefix
  - allowed (wspace)
  - disallowed
- Octal with `0o` prefix
  - allowed (wspace)
  - disallowed
- Binary with `0b` prefix
  - allowed
  - disallowed (wspace)
- `0` prefix
  - decimal (wspace)
  - octal
- Thousands `,` separator
  - allowed
  - disallowed (wspace)
- `_` separator
  - allowed
  - disallowed (wspace)
- Exponential notation with `e`
  - allowed ([wspace 0.3 binary])
  - disallowed (wspace with modern GHC)
- Floating point
  - allowed
  - disallowed (wspace)
- On bad format
  - error (wspace)
  - 0
- Number delimiter
  - LF (wspace)
  - whitespace

`printi` number format:

- Large numbers
  - exact (wspace)
  - exponential notation (wsjq)
  - rounded
- Signed zero
  - positive zero
  - positive zero and negative zero

Division and modulo:

- Zero divisor
  - error (wspace)
  - undefined behavior
- Division rounding
  - truncated
  - floored (wspace)
  - Euclidean
  - ceiling
  - rounding

[wspace 0.3 binary]: https://web.archive.org/web/20150717140342/http://compsoc.dur.ac.uk:80/whitespace/downloads/wspace
