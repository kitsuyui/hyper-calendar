# Switzerland's federal day and the cantons' holidays

Switzerland has one federal holiday and twenty-six cantonal lists. This
document covers `hc-holiday`'s `SWITZERLAND` table: the days every
canton keeps, carried nationwide, and each canton's other days, carried as
rules scoped to its ISO 3166-2 code.

## What it is

The Arbeitsgesetz, Art. 20a Abs. 1, says that the Bundesfeiertag, 1 August,
is equal to Sunday, and that the cantons may equate at most eight other
days a year with Sundays [ch-arg-20a]. A day equal to Sunday is one on
which work is forbidden as on a Sunday, save the exceptions the law allows;
it is a day off. The cantons set their days in their own laws: a
Ruhetagsgesetz, a loi sur les jours fériés, an introductory law to the
Arbeitsgesetz or an ordinance. Many also keep public rest days that they do
not equate with Sunday, on which shops close and noise is restricted but
work is not forbidden. A few keep a day only in part of the canton: the
Catholic and the Reformed communes of Fribourg, the districts of Aargau,
the inner part of Appenzell Innerrhoden, all of Solothurn but the
Bucheggberg.

## How it works

Good Friday is on the lists of twenty-four cantons and not on Ticino's or
the Valais's: Ticino's LALL, art. 6, and its law on official holidays
equate, besides the four days every canton keeps, Epiphany, Easter Monday, the Assumption, All Saints' and St Stephen's
Day with Sundays and keep St Joseph's Day, 1 May, Whit Monday, Corpus
Christi, Saints Peter and Paul and the Immaculate Conception as official
holidays "non parificati alle domeniche" [ch-ti-law]. So on Friday
3 April 2026 `SWITZERLAND` asked for `CH-BE` gives Good Friday, a day
off; asked for `CH-TI` or `CH-VS` it gives nothing. On Thursday
19 March 2026 `CH-TI` gives San Giuseppe as `Kind::Observance`, and the
day is a business day. Asked nationwide, the table gives only New Year's
Day, Ascension, 1 August and Christmas Day, which every canton keeps.

## What is carried

- **Nationwide**: 1 August, equal to Sunday by the Arbeitsgesetz's
  Art. 20a, inserted by the Act of 20 March 1998 and in force from
  1 August 2000 [ch-arg-20a]; the years before are a gap, the Verordnung
  of 30 May 1994 über den Bundesfeiertag not having been read. And New
  Year's Day, Ascension and Christmas Day, which every canton's law read
  keeps: nationwide from 2023, the first year of Jura's law of 2022, the
  latest of the cantons' texts read, a gap before; and in
  each canton from the first year its own text read gives, a gap before
  (the three days are the canton's rules there, `KEPT_EVERYWHERE`). So
  asked for no canton, 25 December 2022 is a gap; asked for Bern, whose
  law is in force from 1 May 1997, it is Christmas Day from 1998.
- **The local names of those three days** are the cantonal law's own: German
  in the German cantons; in Geneva art. 1 of the Loi sur les jours fériés
  ("1er Janvier", "Ascension", "Noël"), in Jura arts. 3 and 4 of its law
  ("Nouvel-An", "l'Ascension", "Noël"), in Neuchâtel art. 3 ("le 1er
  janvier", "l'Ascension", "le jour de Noël") and in Ticino art. 6 of the
  LALL ("Capodanno", "Ascensione", "Natale"), each read in HTML on
  2026-10-03. The French texts of Fribourg, Vaud and Valais are served as
  scripted pages that were not read, so those cantons carry no local name
  for the three and the English name stands, not the German. The nationwide
  trio keeps the German names.
- **Each canton's other days**, scoped to its code, from the first year the
  text read was in force on the day, and the years before a gap: every law
  read replaced an older one that was not read, and the day is usually
  older than the law (Vaud's 2 January and Whit Monday, added in 2007, are
  absent in 2006 and 2007, whose text was read, and a gap before 2006). A day the law makes
  equal to Sunday or declares a public holiday is `Kind::Public`, a public
  rest day it does not make equal to Sunday `Kind::Observance`. 144 rules, and Solothurn's half day.
  Good Friday, Easter Monday, Whit Monday and St Stephen's Day are the
  cantons' days, carried in the cantons whose laws keep them, and are not
  nationwide.
- **Half days**: Solothurn's 1 May "ab 12 Uhr", equal to Sunday from noon
  (§ 46 Abs. 1 a), as `Kind::HalfDay` from 2016, a gap before.
- **Conditional days**: Appenzell Ausserrhoden's 26 December, not kept when
  Christmas is a Monday or a Friday; Neuchâtel's 2 January and 26 December,
  kept only when 1 January or Christmas is a Sunday; Appenzell
  Innerrhoden's 26 December, whose condition the law and the Canton's own
  list read differently, a gap in the years in question; Glarus's
  Fahrtsfest, the first Thursday of April or the Thursday after when that
  falls in Holy Week; Geneva's Jeûne genevois; Vaud's lundi du Jeûne fédéral.
- **Read in a copy**: Jura, Schwyz and Zurich publish their current laws
  as PDF only. Their texts were read in LexFind's copies, the official
  wording as LexFind extracts it from the cantons' publications, marked
  secondary in the entries, and Zurich's version of 2000 to 2004, whose
  § 1 is the current one's, also in ZH-Lex's own HTML. Jura's days are
  carried from 2023, Schwyz's from 2002 and Zurich's from 2001 (St
  Stephen's Day from 2000), the years before a gap. Jura's earlier law, of
  1978, and Zurich's, of 1971, were read too but are not carried: which of
  Jura's days were equal to Sunday from 1979 to 2022 a decree of 1979 set,
  which was not found, and the text of Zurich's law of 1971 read does not
  give the date it came into force.
- **Not yet carried, and why:** the days a law keeps in part of a canton,
  which need a scope finer than a canton; days a commune declares; and the days the last column of the table
  names. The communes' own days are not yet carried: a commune is below
  ISO 3166-2, the finest scope the engine takes, and no commune's
  instrument was read.

### The twenty-six

Read on 2026-09-29 in each canton's systematic collection, or its data
service, unless the row says otherwise. The instrument, its number and the
version read are in the entry. "Rest day" marks a day carried as
`Kind::Observance`.

| Code | Canton | Days carried beyond the four nationwide | Instrument | What it does | First year, and the years before | Not carried |
| --- | --- | --- | --- | --- | --- | --- |
| CH-AG | Aargau | Good Friday (Karfreitag) | [ch-ag-law] | equal to Sunday (§ 6) | 2013; earlier years: gap | Berchtoldstag, Easter Monday, Whit Monday, Corpus Christi, the Assumption, All Saints', the Immaculate Conception and St Stephen's Day, each kept in the districts and communes § 6 names and not in the whole canton |
| CH-AR | Appenzell Ausserrhoden | Good Friday (Karfreitag); Easter Monday (Ostermontag); Whit Monday (Pfingstmontag); St Stephen's Day (zweiter Weihnachtstag) | [ch-ar-law] | equal to Sunday | 1966, 1967; earlier years: gap |  |
| CH-AI | Appenzell Innerrhoden | Good Friday (Karfreitag); Easter Monday (Ostermontag); Whit Monday (Pfingstmontag); Corpus Christi (Fronleichnam); St Stephen's Day (Stephanstag); Assumption (Maria Himmelfahrt) (rest day); All Saints' Day (Allerheiligen) (rest day); Immaculate Conception (Maria Empfängnis) (rest day) | [ch-ai-law] | equal to Sunday (Art. 2 lit. b); local public rest days (lit. c) | 1982, 2011; earlier years: gap | St Maurice, 22 September, kept only in the inner part of the canton; St Stephen's Day in the years whose Christmas is a Monday or a Friday, a gap |
| CH-BL | Basel-Landschaft | Good Friday (Karfreitag); Easter Monday (Ostermontag); Labour Day (1. Mai); Whit Monday (Pfingstmontag); St Stephen's Day (Stephanstag) | [ch-bl-law] | equal to Sunday (§ 10) | 2011; earlier years: gap |  |
| CH-BS | Basel-Stadt | Good Friday (Karfreitag); Easter Monday (Ostermontag); Labour Day (1. Mai); Whit Monday (Pfingstmontag); St Stephen's Day (Stephanstag) | [ch-bs-law] | equal to Sunday (§ 9) | 1994; earlier years: gap |  |
| CH-BE | Bern | Berchtold's Day (der 2. Januar); Good Friday (Karfreitag); Easter Monday (Ostermontag); Whit Monday (Pfingstmontag); St Stephen's Day (der 26. Dezember) | [ch-be-law] | public holidays; no clause equating them with Sunday was found | 1997, 1998; earlier years: gap |  |
| CH-FR | Fribourg | Good Friday (Vendredi-Saint) | [ch-fr-law] | equal to Sunday (art. 49) | 2011; earlier years: gap | Corpus Christi, the Assumption, All Saints' and the Immaculate Conception in the Catholic-majority communes, and 2 January, Easter Monday, Whit Monday and St Stephen's Day in the Reformed-majority communes, which the law does not list |
| CH-GE | Geneva | Good Friday (Vendredi saint); Easter Monday (Lundi de Pâques); Whit Monday (Lundi de Pentecôte); Geneva Fast (Jeûne genevois); Restoration of the Republic (31 Décembre, anniversaire de la restauration de la République) | [ch-ge-law] | declared public holidays | 1991; earlier years: gap | St Stephen's Day, which the law does not keep |
| CH-GL | Glarus | Näfels Pilgrimage (Fahrtsfest); Good Friday (Karfreitag); Easter Monday (Ostermontag); Whit Monday (Pfingstmontag); All Saints' Day (Allerheiligen); St Stephen's Day (Stephanstag) | [ch-gl-law] | equal to Sunday (Art. 2 Abs. 5) | 2012, 2013; earlier years: gap |  |
| CH-GR | Graubünden | Good Friday (Karfreitag); Easter Monday (Ostermontag); Whit Monday (Pfingstmontag); St Stephen's Day (Stefanstag) | [ch-gr-law] | equal to Sunday | 2006; earlier years: gap | the communes' confessional days |
| CH-JU | Jura | Good Friday (Vendredi-Saint); Easter Monday (Lundi de Pâques); Labour Day (1er mai); Whit Monday (Lundi de Pentecôte); Corpus Christi (Fête-Dieu); Berchtold's Day (2 janvier) (rest day); Assumption (Assomption) (rest day); All Saints' Day (Toussaint) (rest day); 23 June (le 23 juin) (rest day) | [ch-ju-law-2022] | equal to Sunday (art. 4); jours fériés officiels not equated with Sunday (art. 3 lit. b) | 2023; earlier years: gap |  |
| CH-LU | Lucerne | Good Friday (Karfreitag); Corpus Christi (Fronleichnam); Assumption (Mariä Himmelfahrt); All Saints' Day (Allerheiligen); St Stephen's Day (Stefanstag); Immaculate Conception (Mariä Empfängnis) (rest day) | [ch-lu-law] | equal to Sunday (§ 1a); a public rest day | 1997, 1998; earlier years: gap | St Joseph's Day and the patronal feasts, kept only where a commune declares them |
| CH-NE | Neuchâtel | 2 January (le 2 janvier); 1 March (le 1er mars); Labour Day (le 1er mai); Good Friday (Vendredi Saint); St Stephen's Day (le 26 décembre) | [ch-ne-law] | equal to Sunday | 2010; earlier years: gap | Corpus Christi in Le Landeron (RSN 941.020); Easter Monday and Whit Monday, which the law does not keep |
| CH-NW | Nidwalden | St Joseph's Day (Josefstag) (rest day); Good Friday (Karfreitag); Corpus Christi (Fronleichnam); Assumption (Maria Himmelfahrt); All Saints' Day (Allerheiligen); Immaculate Conception (Maria Empfängnis) | [ch-nw-law] | equal to Sunday; a public rest day | 2005, 2006; earlier years: gap | Easter Monday, Whit Monday and St Stephen's Day, which the law does not keep |
| CH-OW | Obwalden | Good Friday (Karfreitag); Corpus Christi (Fronleichnam); Assumption (Mariä Himmelfahrt); All Saints' Day (Allerheiligen); Immaculate Conception (Mariä Empfängnis); St Nicholas of Flüe (Bruderklausenfest) (rest day) | [ch-ow-law] | equal to Sunday; a public rest day | 2007, 2008; earlier years: gap | Easter Monday, Whit Monday and St Stephen's Day, which the law does not keep |
| CH-SH | Schaffhausen | Good Friday (Karfreitag); Easter Monday (Ostermontag); Labour Day (1. Mai); Whit Monday (Pfingstmontag); St Stephen's Day (Stephanstag) | [ch-sh-law] | equal to Sunday (§ 7) | 2011; earlier years: gap |  |
| CH-SZ | Schwyz | St Joseph's Day (Josefstag); Good Friday (Karfreitag); Corpus Christi (Fronleichnam); Assumption (Mariä Himmelfahrt); All Saints' Day (Allerheiligen); Epiphany (Heilige Drei Könige) (rest day); Easter Monday (Ostermontag) (rest day); Whit Monday (Pfingstmontag) (rest day); Immaculate Conception (Mariä Empfängnis) (rest day); St Stephen's Day (Stephanstag) (rest day) | [ch-sz-law-2001] | equal to Sunday (§ 2 Abs. 2); public rest days (§ 2 Abs. 1) | 2002; earlier years: gap | the days the communes' voters declare (§ 2 Abs. 1 Ziff. 4) |
| CH-SO | Solothurn | Good Friday (Karfreitag); 1 May from noon, "der 1. Mai (ab 12 Uhr)", a half day | [ch-so-law] | equal to Sunday (§ 46) | 2016; earlier years: gap | Corpus Christi, the Assumption and All Saints', kept except in the Bucheggberg district; Easter Monday, Whit Monday and St Stephen's Day, which the law does not keep |
| CH-SG | St. Gallen | Good Friday (Karfreitag); Easter Monday (Ostermontag); Whit Monday (Pfingstmontag); All Saints' Day (Allerheiligen); St Stephen's Day (Stefanstag) | [ch-sg-law] | equal to Sunday | 2004, 2005; earlier years: gap |  |
| CH-TG | Thurgau | Berchtold's Day (2. Januar); Good Friday (Karfreitag); Easter Monday (Ostermontag); Labour Day (1. Mai); Whit Monday (Pfingstmontag); St Stephen's Day (26. Dezember) | [ch-tg-law] | public rest days, equal to Sunday under the version of 2003 | 2003; earlier years: gap |  |
| CH-TI | Ticino | Epiphany (Epifania); Easter Monday (Lunedì di Pasqua); Assumption (Assunzione); All Saints' Day (Ognissanti); St Stephen's Day (Santo Stefano); St Joseph's Day (San Giuseppe) (rest day); Labour Day (1° Maggio) (rest day); Whit Monday (Lunedì di Pentecoste) (rest day); Corpus Christi (Corpus Domini) (rest day); Saints Peter and Paul (SS. Pietro e Paolo) (rest day); Immaculate Conception (Immacolata) (rest day) | [ch-ti-law] | parificati alle domeniche (LALL art. 6); giorni festivi ufficiali non parificati | 2010, 2011, 2012; earlier years: gap | Good Friday, which neither law keeps |
| CH-UR | Uri | Good Friday (Karfreitag); Corpus Christi (Fronleichnam); Assumption (Mariä Himmelfahrt); All Saints' Day (Allerheiligen); Immaculate Conception (Mariä Empfängnis); Epiphany (Dreikönigen) (rest day); St Joseph's Day (Sankt-Josefs-Tag) (rest day); Easter Monday (Ostermontag) (rest day); Whit Monday (Pfingstmontag) (rest day); St Stephen's Day (Sankt-Stefans-Tag) (rest day) | [ch-ur-law] | equal to Sunday (KAV); public rest days (LSG) | 2002, 2003; earlier years: gap |  |
| CH-VS | Valais | St Joseph's Day (Saint-Joseph); Corpus Christi (Fête-Dieu); Assumption (Assomption); All Saints' Day (Toussaint); Immaculate Conception (Immaculée Conception) | [ch-vs-law] | assimilés aux dimanches | 2016, 2017; earlier years: gap | Good Friday, Easter Monday, Whit Monday and St Stephen's Day, which the ordinance does not keep |
| CH-VD | Vaud | Good Friday (le Vendredi-Saint); Easter Monday (le lundi de Pâques); Federal Fast Monday (le lundi du Jeûne fédéral); Berchtold's Day (le 2 janvier); Whit Monday (le lundi de Pentecôte) | [ch-vd-law] | assimilés aux dimanches | 2006; 2 January and Whit Monday from 2008, absent in 2006 and 2007 (added by the modification of 2007); earlier years: gap | St Stephen's Day, which the law does not keep |
| CH-ZG | Zug | Good Friday (Karfreitag); Corpus Christi (Fronleichnam); Assumption (Maria Himmelfahrt); All Saints' Day (Allerheiligen); Immaculate Conception (Maria Empfängnis) | [ch-zg-law] | equal to Sunday (§ 1) | 2004; earlier years: gap | Easter Monday, Whit Monday and St Stephen's Day, which the law does not keep |
| CH-ZH | Zurich | Good Friday (Karfreitag); Easter Monday (Ostermontag); Labour Day (1. Mai); Whit Monday (Pfingstmontag); St Stephen's Day (Stephanstag) | [ch-zh-law-2000] | equal to Sunday (§ 1 Abs. 3) | 2001, St Stephen's Day 2000; earlier years: gap |  |

## Accuracy

`crates/hc-holiday/tests/switzerland_cantons.rs` holds each of the 144
rules to its law: the day's date in its first year and in 2026, worked out
by a script apart from the crate; its kind and its canton; nothing the
year before, and a gap there, save Vaud's additions of 2007, absent in
2006; the four nationwide days alone for the country; Good Friday
2026 in Bern and not in Ticino or the Valais; the conditional days in the
years that test their conditions; and Ticino's and Lucerne's rest days as
business days. What a reader should know:

- Glarus's law equates nine weekdays besides 1 August with Sunday, one more
  than Art. 20a allows; the table follows the law.
- Bern's law lists its public holidays and the constitution makes them
  public rest days, but no clause equating them with Sunday was found; they
  are carried as days off.
- Thurgau's law of 2025, in force from 2026, keeps the same days but drops
  the clause of the 2003 version that made them equal to Sunday; they are
  carried as days off, the 2003 version's reading.
- Some laws name a day without its date: Glarus's Fahrtsfest, dated from
  the Canton's pages; Vaud's lundi du Jeûne, dated from the Federal Fast
  on the third Sunday of September, which the law does not state; the
  saints' days of Lucerne, Uri and Ticino, on their feasts.
- The first years are those of the text read, and the years before are
  gaps, not years without the day. Many of the days are older; Geneva's law
  is of 1951, amended in 1957 and 1966 in articles not named.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ch-arg-20a] | Art. 20a Abs. 1: 1 August, and the cantons' eight days | Yes, 2026-09-29, Fedlex |
| [ch-ag-law] … [ch-zg-law] | Each canton's days, their effect and the version in force | Yes, 2026-09-29 |
| [ch-ju-law-2022], [ch-sz-law-2001], [ch-zh-law-2000] | Jura's, Schwyz's and Zurich's days, their effect and the version in force | Yes, 2026-09-29, in LexFind's copies of the official texts, and Zurich's of 2000 in ZH-Lex's HTML |
| [ch-ju-law], [ch-sz-law], [ch-zh-law] | The secondary sources these three cantons' days were first carried from | Yes, 2026-09-29; superseded by the laws |

## Code

`crates/hc-holiday/src/countries/switzerland.rs`: the four nationwide
rules first in `CH_OWN_RULES`, then each canton's, built by `canton` and
`canton_rest_day` with `read_from` the first year of the text read, and
`KEPT_EVERYWHERE`, the three days every canton keeps, nationwide and in
each canton, joined into `CH_RULES`; with `GENEVA_FAST`, `FEDERAL_FAST_MONDAY` and the
functions of the conditional days; one region and one citation constant
per canton, `CH_AG` and `CH_AG_LAW` to `CH_ZH` and `CH_ZH_LAW`. The
exchange `XSWX` keeps its own list. Anchors: the tests in
`crates/hc-holiday/tests/switzerland_cantons.rs`, and
`the_low_countries_and_the_alps` in `crates/hc-holiday/tests/countries.rs`.
