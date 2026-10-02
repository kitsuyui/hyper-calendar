#!/usr/bin/env python3
"""Regenerate crates/hc-astro/src/vsop87_jupiter/data.rs from VSOP87B.jup.

The heliocentric longitude, latitude and radius of Jupiter, in the dynamical
ecliptic and equinox of J2000.0, as the Bureau des longitudes' VSOP87
solution, version B2 (Bretagnon and Francou, "Planetary theories in
rectangular and spherical variables: VSOP87 solutions", Astronomy and
Astrophysics 202, 309-315, 1988). The file is `VSOP87B.jup` of the IMCCE's
distribution, <https://ftp.imcce.fr/pub/ephem/planets/vsop87/VSOP87B.jup>
(CDS catalogue VI/81 keeps a copy). The workspace does not fetch it: this
script reads the copy a person has saved and says what it found.

    python3 scripts/vsop87-jupiter.py PATH/VSOP87B.jup            # rewrite the file
    python3 scripts/vsop87-jupiter.py PATH/VSOP87B.jup --check    # exit 1 if it is stale
    python3 scripts/vsop87-jupiter.py PATH/VSOP87B.jup --stats    # print the sizes

The file is 484 519 bytes, 3 625 terms in 18 blocks: variables 1, 2 and 3
(longitude, latitude, radius) times the powers 0 to 5 of tau, the time in
Julian millennia from J2000.0, each block a sum of terms A cos(B + C tau).
Its sha256 is held below, and the generated file records it, so a different
file is refused rather than quietly turned into data.

A term line of the file, in Fortran, is
`(1x, 4i1, i5, 12i3, f15.11, 2f18.11, f14.11, f20.11)`: the version, the
body, the variable and the power, the term's number, the twelve integer
multipliers of the planets' mean longitudes and the Delaunay arguments, then
S, C, the amplitude A, the phase B and the frequency C. Only A, B and C are
read, as 11-decimal integers (units of 1e-11), which is exactly the file's
own precision: each integer divided by 1e11 is the nearest f64 to the
decimal the file prints, so nothing is rounded. S and C are used once, to
check that A is their hypotenuse, to the file's last digit. B is not their
angle: S and C belong to the argument of the planets' mean longitudes, and B
is that argument's value at J2000.0 less their phase.

Each term is stored in 13 bytes, little-endian: A in six bytes (the largest
is the mean motion, 529.69 rad per millennium, the one term of the T^1
block of the longitude), B in five (it is under 2 pi) and an index in two into the
table of the 953 distinct frequencies, which many terms share.
"""

import hashlib
import math
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / "crates/hc-astro/src/vsop87_jupiter/data.rs"

SHA256 = "408b5938ab39e661940b4253579f4a7a65f206926d98bec20e46aebb06cdd159"
BYTES = 484519
TERMS = 3625
SCALE = 10**11
TERM_BYTES = 13
HEADER = re.compile(
    r"^ VSOP87 VERSION B2\s+JUPITER\s+VARIABLE (\d) \(LBR\)\s+\*T\*\*(\d)\s+(\d+) TERMS\s+"
    r"HELIOCENTRIC DYNAMICAL ECLIPTIC AND EQUINOX J2000"
)
# The columns of a term line: S, C, A, B, frequency.
COLUMNS = ((46, 61), (61, 79), (79, 97), (97, 111), (111, 131))


def units(text):
    """A decimal with 11 places as an integer of 1e-11, refusing any other."""
    whole, _, fraction = text.strip().partition(".")
    if len(fraction) != 11:
        raise SystemExit(f"not an 11-decimal number: {text!r}")
    sign = -1 if whole.startswith("-") else 1
    return sign * (abs(int(whole)) * SCALE + int(fraction))


def parse(raw):
    if len(raw) != BYTES or hashlib.sha256(raw).hexdigest() != SHA256:
        raise SystemExit(
            f"this is not the VSOP87B.jup the generator was written against: "
            f"{len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}"
        )
    blocks = []
    for line in raw.decode("ascii").splitlines():
        header = HEADER.match(line)
        if header:
            blocks.append((int(header[1]), int(header[2]), int(header[3]), []))
            continue
        if not blocks or len(line) < 131:
            raise SystemExit(f"a line that is neither header nor term: {line!r}")
        s, c, a, b, f = (units(line[lo:hi]) for lo, hi in COLUMNS)
        if abs(math.hypot(s, c) - a) > 2 or a < 0:
            raise SystemExit(f"A is not the hypotenuse of S and C: {line!r}")
        if not 0 <= b < 2 * math.pi * SCALE or f < 0 or a >= 1 << 48:
            raise SystemExit(f"a value out of the stored range: {line!r}")
        blocks[-1][3].append((a, b, f))
    order = [(v, p) for v in (1, 2, 3) for p in range(6)]
    if [(v, p) for v, p, _, _ in blocks] != order:
        raise SystemExit("the blocks are not variables 1-3 times powers 0-5, in order")
    for v, p, count, terms in blocks:
        if len(terms) != count:
            raise SystemExit(f"block {v}/{p} has {len(terms)} terms, its header says {count}")
    if sum(len(b[3]) for b in blocks) != TERMS:
        raise SystemExit("the file does not have 3625 terms")
    return blocks


def literal(data, width=26):
    """A byte string literal, `width` bytes to a source line."""
    lines = []
    for start in range(0, len(data), width):
        chunk = data[start : start + width]
        lines.append("".join(f"\\x{byte:02x}" for byte in chunk))
    return 'b"\\\n' + "\\\n".join("        " + line for line in lines) + '"'


def render(raw):
    blocks = parse(raw)
    frequencies = sorted({f for _, _, _, terms in blocks for _, _, f in terms})
    index = {f: i for i, f in enumerate(frequencies)}
    if len(frequencies) >= 1 << 16:
        raise SystemExit("too many frequencies for a u16 index")
    out = []
    out.append("// @generated by scripts/vsop87-jupiter.py from VSOP87B.jup; do not edit.\n")
    out.append("//\n")
    out.append("// Source: P. Bretagnon and G. Francou, \"Planetary theories in rectangular and\n")
    out.append("// spherical variables: VSOP87 solutions\", Astronomy and Astrophysics 202,\n")
    out.append("// 309-315 (1988), `bretagnon1988`. Version B2, heliocentric spherical\n")
    out.append("// variables, dynamical ecliptic and equinox of J2000.0. The IMCCE's file\n")
    out.append("// `VSOP87B.jup`, https://ftp.imcce.fr/pub/ephem/planets/vsop87/VSOP87B.jup\n")
    out.append(f"// ({BYTES} bytes, {TERMS} terms in 18 blocks), sha256\n")
    out.append(f"// {SHA256}.\n")
    out.append("//\n")
    out.append("// Each term is 13 bytes, little-endian: the amplitude A in six bytes, the phase\n")
    out.append("// B in five, both in units of 1e-11 (the file's own precision, so nothing is\n")
    out.append("// rounded), and a u16 index into FREQUENCIES. A term of a block is\n")
    out.append("// A cos(B + C tau), tau in Julian millennia from J2000.0 TDB.\n\n")
    out.append("/// The SHA-256 of the IMCCE file `VSOP87B.jup` these tables were read from.\n")
    out.append(f'pub const SOURCE_SHA256: &str = "{SHA256}";\n\n')
    out.append("/// The number of terms of the file, all 18 blocks together.\n")
    out.append(f"pub const SOURCE_TERMS: usize = {TERMS};\n\n")
    out.append("/// The bytes of one stored term.\n")
    out.append(f"pub(super) const TERM_BYTES: usize = {TERM_BYTES};\n\n")
    out.append("/// The distinct frequencies C of the file, in radians per Julian millennium.\n")
    out.append(f"pub(super) const FREQUENCIES: [f64; {len(frequencies)}] = [\n")
    for f in frequencies:
        out.append(f"    {f // SCALE}.{f % SCALE:011d},\n")
    out.append("];\n\n")
    out.append("/// The blocks of the file: the variable (1 longitude, 2 latitude, 3 radius),\n")
    out.append("/// the power of tau, and the packed terms.\n")
    out.append("pub(super) const BLOCKS: [(u8, u8, &[u8]); 18] = [\n")
    for v, p, count, terms in blocks:
        data = bytearray()
        for a, b, f in terms:
            data += a.to_bytes(6, "little") + b.to_bytes(5, "little") + index[f].to_bytes(2, "little")
        assert len(data) == TERM_BYTES * count
        out.append(f"    // Variable {v}, tau^{p}: {count} terms.\n")
        out.append(f"    ({v}, {p}, {literal(bytes(data))}),\n")
    out.append("];\n")
    return "".join(out), blocks, frequencies


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    flags = {a for a in sys.argv[1:] if a.startswith("--")}
    if len(args) != 1 or flags - {"--check", "--stats"}:
        raise SystemExit(__doc__)
    text, blocks, frequencies = render(Path(args[0]).read_bytes())
    if "--stats" in flags:
        print(f"{TERMS} terms, {len(frequencies)} distinct frequencies")
        print(f"packed terms {TERMS * TERM_BYTES} bytes, frequencies {8 * len(frequencies)} bytes")
        print(f"source text {len(text.encode())} bytes")
    if "--check" in flags:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            print(f"{OUTPUT.relative_to(ROOT)} is stale: run scripts/vsop87-jupiter.py", file=sys.stderr)
            sys.exit(1)
        print("up to date")
        return
    if "--stats" not in flags:
        OUTPUT.parent.mkdir(parents=True, exist_ok=True)
        OUTPUT.write_text(text)
        print(f"wrote {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
