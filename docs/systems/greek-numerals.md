# Greek numerals

Backs `hc-i18n`'s numbering systems `grek` and `greklow`, written and read
back.

## What it is

Greek writes numbers with the letters of its alphabet, each standing for a
value, the values added [wikipedia-greek-stigma]. The letters are those of
the alphabet in order, with three that fell out of it as letters and stayed
as numerals: the digamma for 6, the koppa for 90 and the sampi for 900. A
mark after the letters, the *keraia* ʹ, shows that they are a number, and a
lower mark before a letter, ͵, multiplies it by a thousand.

The 6 is written three ways. In antiquity it was the digamma Ϝ, and a
special numeral form came to replace the letter's. By the Byzantine era
that form, called the *episemon*, had merged with the ligature of σ and τ,
the *stigma* ϛ. The decline of ligatures in the twentieth century means
that in Greece today "the letter sequence ΣΤʹ or στʹ is often used in lieu
of ϛʹ itself to write the number 6" [wikipedia-greek-stigma].

CLDR carries the system as two numbering systems of the algorithmic type,
`grek` in capitals and `greklow` in small letters, whose rules are the
`greek-upper` and `greek-lower` rule sets of `common/rbnf/root.xml`
(`supplemental/numberingSystems.xml`, [cldr48-supplemental]). `el.xml`
names `grek` as Greek's `traditional` system.

## How it works

CLDR 48's `%%greek-numeral-minuscules` and `%%greek-numeral-majuscules`
rules [cldr48-rbnf]:

- 1 to 9 are α β γ δ ε ϝ ζ η θ, the tens ι κ λ μ ν ξ ο π ϟ, the hundreds
  ρ σ τ υ φ χ ψ ω ϡ; a number below a thousand is its hundreds', tens' and
  units' letters in that order, any of them left out where it is zero.
- A thousand to 9 999 is the lower numeral sign ͵ (U+0375) and the
  thousands' unit letter, then the rest: ͵β for 2000.
- From ten thousand the number is counted in myriads: the count, then μ
  (Μ in capitals), then a space and the rest. 10⁸ is μμ, 10¹² μμμ and
  10¹⁶ μμμμ; from 10¹⁸ the rules write decimal digits.
- Zero is 𐆊 (U+1018A), a negative number takes the minus sign −, and the
  whole number is followed by the numeral sign, which the rules write as
  the acute accent ´ (U+00B4).

**Worked example.** 2026 is two thousands, two tens and six: ͵β for the
thousands, κ for twenty, ϝ for six, and the sign, ͵βκϝ´. In capitals it is
͵ΒΚϜ´. 12 345 is one myriad and 2 345: αμ, a space, ͵βτμε, the sign,
αμ ͵βτμε´.

**Reading back.** The same letters are read in the same order. The sign
may be the acute accent, the Greek *keraia* ʹ (U+0374), the modifier prime
ʹ (U+02B9) it decomposes to, an apostrophe, or absent. The 6 may be the
digamma, the stigma ϛ (U+03DB, Ϛ U+03DA in capitals), or the two letters
στ (ΣΤ), as Greece writes it today: ͵βκϛʹ and ͵βκστʹ are 2026 too. The
pair is not ambiguous, since σ (200) before τ (300) spells no other number;
σστ is 206.

## What is carried

- `grek` and `greklow`, written by CLDR 48's rules from 𐆊 to 10¹⁸ − 1 and
  refused from 10¹⁸, where the rules turn to decimal digits.
- Reading back, of every number the writer writes, with the other forms of
  the sign and of 6 above.
- Not carried: the rules' `x.x` form for a fraction, since a calendar field
  is an integer.

## Accuracy

| Check | Source | Test | Result |
| --- | --- | --- | --- |
| 0, 6, 99, 666, 1000, 2026, 10 000, 12 345, 40 000, 400 000, 10⁸, −7 | [cldr48-rbnf], worked by hand from the rules | `the_rules_spell_these_numbers` (`numbering::greek`) | every value |
| Every number to a million, and three beyond, read back | the writer | `every_number_to_a_million_reads_back_and_some_beyond` | every value |
| ϛʹ, στʹ, ΣΤʹ, ͵βκϛʹ, ͵βκστʹ, σϛ, σστ, ͵στ; στα, ϛϛ and ϛι refused | [wikipedia-greek-stigma] | `six_reads_as_the_stigma_and_as_sigma_tau` | agrees |

## Sources

| Key | What it gives | Read |
| --- | --- | --- |
| [cldr48-rbnf] | `%greek-upper`, `%greek-lower` and their private rule sets | Yes, 2026-09-29 |
| [cldr48-supplemental] | `numberingSystems.xml`'s `grek` and `greklow` | Yes, 2026-09-29 |
| [wikipedia-greek-stigma] | The digamma, the episemon, the stigma, and στ for 6 in Greece today (secondary) | Yes, 2026-09-29 |

## Code

- `crates/hc-i18n/src/numbering/greek.rs`, the letters, the writer and
  the reader; `crates/hc-i18n/src/numbering.rs` registers `grek` and
  `greklow`.
