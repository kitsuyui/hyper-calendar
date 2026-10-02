# Japan's holiday law and its amendments

Backs the `JAPAN` table (`JP`) in `hc-holiday`, and through it the Tokyo
Stock Exchange (`XJPX`), which includes it; the prefectures' own days and
their other days, which are rules of the same table scoped to each
prefecture's ISO 3166-2 code, `JP-01` to `JP-47`; and the cities' days,
scoped to a city's code under its prefecture's, `JP-14-130` for 川崎市
([ADR 0014](../adr/0014-a-municipality-is-a-region-within-its-subdivision.md)).

## What it is

Japan's public holidays are the 国民の祝日 of one statute, the
国民の祝日に関する法律 (Act on National Holidays, 昭和23年法律第178号),
promulgated and in force on 20 July 1948, which its Article 1 explains as
days on which the people "こぞつて祝い、感謝し、又は記念する". Article 2
is the list — a name, a date and a one-line purpose for each — and
Article 3 makes them days off and adds the two derived days, the 振替休日
and the 国民の休日. There is no decree, no annual arrangement and no
designation: a year's holidays follow from the Act's text as amended,
and from one number the Act does not state, the day of each equinox
[jp-holiday-act].

**The list**, as in force [jp-holiday-act]: 元日 (1 January), 成人の日
(second Monday of January), 建国記念の日 ("政令で定める日", 11 February),
天皇誕生日 (23 February), 春分の日 ("春分日"), 昭和の日 (29 April),
憲法記念日 (3 May), みどりの日 (4 May), こどもの日 (5 May), 海の日
(third Monday of July), 山の日 (11 August), 敬老の日 (third Monday of
September), 秋分の日 ("秋分日"), スポーツの日 (second Monday of October),
文化の日 (3 November) and 勤労感謝の日 (23 November): sixteen.

**The amendments.** The Act has been amended eleven times, and three
other statutes have set holidays beside it for particular years
[jp-holiday-act, wikisource-ja-holiday-act, cao-shukujitsu-kaku]:

| In force | Law | Change |
| --- | --- | --- |
| 1948-07-20 | 昭和23年法律第178号 | The nine original holidays: 元日, 成人の日 (15 January), 春分の日, 天皇誕生日 (29 April), 憲法記念日, こどもの日, 秋分の日, 文化の日, 勤労感謝の日 |
| 1966-06-25 | 昭和41年法律第86号 | 建国記念の日 on a day to be fixed by 政令 within six months, 敬老の日 (15 September), 体育の日 (10 October). 昭和41年政令第376号 of 9 December 1966 fixed 11 February, so it was first kept in 1967 |
| 1973-04-12 | 昭和48年法律第10号 | 振替休日: 「国民の祝日」が日曜日にあたるときは、その翌日を休日とする |
| 1985-12-27 | 昭和60年法律第103号 | 国民の休日: a day whose eve and morrow are both 国民の祝日 becomes a holiday |
| 1989-02-17 | 平成元年法律第5号 | After the death of 昭和天皇 on 7 January: 天皇誕生日 to 23 December, 29 April becomes みどりの日 |
| 1996-01-01 | 平成7年法律第22号 | 海の日, 20 July |
| 2000-01-01 | 平成10年法律第141号 | ハッピーマンデー: 成人の日 to the second Monday of January, 体育の日 to the second Monday of October |
| 2003-01-01 | 平成13年法律第59号 | 海の日 to the third Monday of July, 敬老の日 to the third Monday of September |
| 2007-01-01 | 平成17年法律第43号 (promulgated 20 May 2005) | 29 April becomes 昭和の日, みどりの日 moves to 4 May, and 振替休日 is reworded: その日後においてその日に最も近い「国民の祝日」でない日 |
| 2016-01-01 | 平成26年法律第43号 | 山の日, 11 August |
| 2019-05-01 | 平成29年法律第63号, 附則第10条 | The 天皇の退位等に関する皇室典範特例法 amends the Act on the day of the accession: 天皇誕生日 to 23 February |
| 2020-01-01 | 平成30年法律第57号 | 体育の日 renamed スポーツの日, with a new purpose |

And beside the Act:

| Year | Law | Effect |
| --- | --- | --- |
| 1959, 1989, 1990, 1993 | 昭和34年法律第16号; 平成元年法律第4号; 平成2年法律第24号; 平成5年法律第32号 | One day each: the wedding of Crown Prince Akihito (10 April 1959), the state funeral of 昭和天皇 (24 February 1989), the enthronement ceremony (12 November 1990), the wedding of Crown Prince Naruhito (9 June 1993) |
| 2019 | 平成30年法律第99号 (14 December 2018) | 天皇の即位の日 (1 May) and 即位礼正殿の儀の行われる日 (22 October) as holidays, and — its Article 2 — treated as 国民の祝日 for Article 3 paragraphs 2 and 3 of the Act, which is why 30 April and 2 May became 国民の休日 |
| 2020 | 平成30年法律第55号 (20 June 2018), Article 32 of the Tokyo Olympic and Paralympic special measures act | For 2020 only, 海の日 on 23 July, スポーツの日 on 24 July, 山の日 on 10 August, around the opening and closing ceremonies |
| 2021 | 令和2年法律第68号 (4 December 2020) | The postponed Games: for 2021 only, 海の日 on 22 July, スポーツの日 on 23 July, 山の日 on 8 August |

**The equinox days.** 春分の日 and 秋分の日 are defined as 春分日 and
秋分日, the day of the equinox, and nothing more. The National
Astronomical Observatory of Japan computes the instant in Japan Standard
Time for the year after next and the Cabinet Office publishes the
resulting dates in the 官報 in the 暦要項 on the first business day of
February of the preceding year — the 暦要項 for 2027, dated 2 February
2026, gives Sunday 21 March and Thursday 23 September, with "3月22日は
振休となる". The Observatory's own FAQ says the days are "正式に" decided
only by that publication, and that its computed dates for later years are
for reference, since "地球の運行状態は常に変化している"
[nao-rekiyoko-2027, nao-faq-equinox, cao-shukujitsu].

**The prefectures' own days.** A prefecture cannot make a 国民の祝日, but
it can set a day of its own by ordinance. Eighteen of the forty-seven
have, most as a 県民の日 — Tokyo's is the 都民の日, Hokkaido's the
北海道みんなの日, Fukui's and Toyama's a ふるさとの日 — and Okinawa has its
慰霊の日; Akita and Ehime keep a day whose instrument was not found.
The ordinances follow one pattern: an article of purpose, one that fixes
the date — 「都民の日は、十月一日とする。」 — and
usually one that makes the prefecture hold events around it and one that
waives the fees of its museums and parks on the day. None of them closes
anything [jp-pref-13, jp-pref-11]. What closes is decided by two other
instruments:

- **The prefectural schools.** A rule of the board of education lists the
  schools' 休業日, and in seven prefectures it now lists the day, from a
  year the table below gives for each: Tokyo's
  都立学校の管理運営に関する規則 Article 5 names 「都民の日条例…の規定する日」,
  and Ibaraki, Gunma, Saitama, Chiba, Yamanashi and, from 2026, Kagawa do
  the same for their 県民の日, as Okinawa's does for 慰霊の日. Aichi's
  県立高等学校学則 makes one day of the week 21–27 November, which the board
  sets each year, a day without classes, rather than 27 November itself
  [jp-pref-23].
- **The prefecture's offices.** Under 地方自治法 Article 4-2 a local
  government's 休日 are set by ordinance, and paragraph 2 limits them to
  Sundays and Saturdays, the 国民の祝日 and days of the year's end and
  beginning. Paragraph 3 allows one more kind: a day of "特別な歴史的、社会的
  意義", kept by the residents as a whole, after consulting the minister;
  and paragraph 4 moves a deadline for an application to the government's
  offices that falls on such a day to the next day [jp-local-autonomy-act].
  Every prefecture has a 休日条例, and of the forty-seven only Okinawa's
  uses paragraph 3: its Article 1 item 4 is 「６月23日（沖縄県慰霊の日を
  定める条例…第２条に規定する慰霊の日）」, the day its ordinance sets to
  console the dead of the Second World War [jp-pref-47,
  jp-pref-holiday-ordinances].

Neither makes the day a public holiday. The national government's offices
are open, a bank's holidays are the national ones of 銀行法施行令 Article 5
[jp-bank-act-order], and no private employer is bound.

**慰霊の日's history** is older than its ordinance. The 琉球政府's
住民の祝祭日に関する立法 of 1961 made 22 June a holiday, and an amendment of
1965 moved it to 23 June; after the reversion of 1972 the prefecture set
the day by 沖縄県慰霊の日を定める条例 (昭和49年沖縄県条例第42号, in force
21 October 1974). The 1988 amendment of 地方自治法 that created Article 4-2
(昭和63年法律第94号) had no room for it, and kept the existing 休日 only
until each government made its 休日条例; 平成3年法律第24号 added paragraph
3 on 2 April 1991, and Okinawa's 休日条例 (平成3年沖縄県条例第15号) followed
on 24 May 1991 with 23 June in it, in force from 26 May [jp-pref-47,
jp-local-autonomy-act, okinawa-archives-irei]. The 1961 and 1965 acts themselves were not read.
What the prefecture kept in 1972, 1973 and 1974, between the reversion
(15 May 1972) and the ordinance, no instrument read says, so those years
are a gap with 1975–1990, as `Rule::UNREAD` carries them (audit 10 a7); before 1972 the islands were not a
prefecture of Japan.

**The prefectures' other days.** Beside its own day, a prefecture sets
days for a cause by ordinance, in the same pattern: an article of purpose,
one that fixes the date, and usually a duty on the prefecture to make
efforts. Shimane's 竹島の日を定める条例 fixes 「竹島の日は、2月22日とする。」;
Shizuoka and Yamanashi each fix 富士山の日 on 23 February; Shiga's
環境基本条例 Article 8 fixes びわ湖の日 on 1 July; eleven prefectures fix
an 教育の日 on 1 November, most with a week or a month of education
around it, and five more set one by a 告示, a 要綱 or a decision of the
board; there are memorial days for the Great East Japan Earthquake, the
Tokyo air raids and the 1978 Miyagi earthquake, Okinawa's
しまくとぅばの日 and 琉球歴史文化の日, and Gifu's 飛騨・美濃じまんの日.
None closes anything, and two waive fees: Fukushima's education week
and Okinawa's 琉球歴史文化の日 [jp-pref-education-days,
jp-pref-ordinance-days].

**The municipalities.** A city sets days by the same means as a
prefecture, and more often by a 告示 than an ordinance. Of the twenty
政令指定都市, さいたま, 千葉, 横浜, 川崎, 浜松, 名古屋, 京都, 堺, 神戸, 岡山,
広島 and 熊本 set one, 静岡 a day for its tea; 札幌, 仙台, 新潟, 大阪, 北九州
and 福岡 set none that was found, and 相模原's is named without a date.
Three cities' schools close on the day, by the city's school rule:
さいたま市民の日 (1 May), 横浜's 開港記念日 (2 June) and 川崎's 市制記念日
(1 July). One city's offices close: 広島 made 6 August, the 平和記念日,
a day off for its offices in 1947, by 広島市役所事務休停日条例, and since
1991 its 休日条例 lists it, the only 休日条例 of the twenty with a day
beyond the national ones, as Article 4-2 paragraph 3 allows.
長崎's ながさき平和の日 of 9 August is set by ordinance, and its 休日条例
does not list it [jp-city-designated].
## How it works

**Two rules of Article 3.** Paragraph 2, the 振替休日: a 国民の祝日 that
falls on a Sunday is kept on the nearest following day that is not a
国民の祝日. Before 2007 it read simply "その翌日", the following day; the
rewording was needed because moving みどりの日 to 4 May made 3, 4 and
5 May three 祝日 in a row, so that a Sunday 3 May would have sent its
substitute onto a day that was already one. Paragraph 3, the 国民の休日: a
day that is not a 祝日 and whose previous and following days both are
becomes a holiday. A Sunday cannot be one — it is a day off already — and
neither can a 振替休日, because a 振替休日 is a 休日 and not a 祝日, and a
day between a 祝日 and a 振替休日 has only one 祝日 neighbour
[jp-holiday-act, jp-holiday-act-amendment-2005].

**The table.** Every holiday is a rule with the years it held: 成人の日 as
15 January for 1949–1999 and the second Monday from 2000; 天皇誕生日 as
29 April to 1988, 23 December for 1989–2018 and 23 February from 2020,
with no rule at all for 2019; 海の日, 山の日 and スポーツの日 each with a
rule for 2020 alone and one for 2021 alone. The equinox days are
`Rule::SolarTerm` at `Meridian::JAPAN`, so the crate computes the day
from the instant as the Observatory does rather than carrying a table —
nobody legislates one. The 1948 rules that fall before 20 July carry
`valid_from` 1949, and 建国記念の日 carries 1967. Everything is data;
there is no function in the module.

**The policies.** 振替休日 is two `SubstitutionPolicy` entries, trigger
Sunday, direction `Forward`: 1973–2006 with `skip_occupied` false, so the
walk names one day and stops (and would produce nothing if that day were
taken, which never arose), and from 2007 with `skip_occupied` true.
建国記念の日 alone carries `substituted_from(1974)`, because the 1973 act
came into force on 12 April, after that year's 11 February had fallen on
a Sunday: 12 February 1973 was an ordinary Monday, and the first 振替休日
was 30 April 1973. 国民の休日 is a `BridgePolicy` with `max_gap` 1,
`exclude_weekdays` Sunday, from 1986. The engine evaluates the bridges
against the base holidays — the 祝日, not the substitutes — and refuses a
candidate that is already occupied by a 祝日 or a substitute, which is the
paragraph's two exclusions.

**The exchange.** `TOKYO_STOCK_EXCHANGE` includes `JAPAN` and adds 2 and
3 January and 31 December, the exchange's own closures.

**The prefectures.** Each prefectural day is a rule of `JAPAN` with its
prefecture's code in `regions`, a `FixedGregorian` date from its first
year, no substitution, and the kind its instruments give it:
`Kind::School` where the schools close, from the first year a school rule
read shows it, `Kind::Government` for 慰霊の日, `Kind::Observance` where
nothing closes and before a school rule's first year read. A day whose
kind changed is two rules, one for each span, as a holiday whose date
changed is. `is_day_off` is false for all
three, so business-day arithmetic is the same in every prefecture. A
calendar built for `JAPAN` with no region has none of them; one built in
`JP-13` has the nationwide days and 都民の日. Okinawa's rule for 1972–1990
is `Rule::UNREAD`: the ordinance set the day from 1975, and the prefecture
was restored in 1972, but no instrument read says what the day was for
the prefecture's offices before the 休日条例, so those years are a gap
rather than a guess (policy §4). The other days
are observances of the same shape, 東京 and 山形's education days
`Rule::NthWeekday` Saturdays, and 山形's `read_from` 2026, the year of the
page read, since its 要綱's year is not known. Aichi's school holiday is
`Rule::UNREAD` of `Kind::School` from 2023: each school or municipal board
chooses its day among the candidates, and the days chosen are published
only as PDF lists. A caller asks for a prefecture by its code as a
region, in either case; `JAPAN.regions()` lists the prefectures and
cities that have a day, and the boundary's `hc_holiday_tables` names them
in the `JP` row.

**The municipalities.** Each city's day is a rule scoped to the city's
code: the prefecture's ISO 3166-2 code, a hyphen, and the three-digit
市区町村コード of JIS X 0402, the 全国地方公共団体コード without the
prefecture's digits and the check digit — さいたま市, 11100 with check
digit 7, is `JP-11-100` [jis-x0402-cities]. A calendar built in
`JP-11-100` has the nationwide days, Saitama's and the city's own; one
built in `JP-11` has none of the city's (ADR 0014). The designated cities
that set no day are listed among the subdivisions read, and answer with
their prefecture's days and no gap; any other city — 鎌倉市, `JP-14-204` —
keeps its prefecture's days, and its own are the gap
`UNREAD_SUBDIVISION`, since no instrument of it was read. A day whose
instrument names it with its date but does not set it, 浜松's 市制記念日
in the fee ordinance of 1997 and 堺's 開庁記念日 in the award ordinance of
1971, is `read_from` that instrument's first year, and the years before
are gaps; 横浜's 開港記念日 is `read_from` 2021, the year of the last
amendment of the school rule's article, because the city assembly's
resolutions that set it from 1928 were not read. 相模原's 市制施行記念日,
named without a date, is `Rule::UNREAD` in every year. 京都's
伝統産業の日 is 春分の日, and is `Rule::SolarTerm` as the national day is.

The cities' names are `hc-i18n`'s `municipal_names`, not CLDR's, which
names no municipality: 川崎市 in `ja`, `Kawasaki-shi` in `ja-Latn` and
`Kawasaki` in `en`, from the instruments this section reads and JIS X 0402's
list [jis-x0402-cities], and from English and Japanese Wikipedia for the
romanised and English forms. `hc_place_name` and `hc_subdivisions` write
them, and a test holds every municipality a table lists to a name; the
sources and what is not carried are in
[place-names.md](place-names.md).

**Worked example: 2 June 2026 in Yokohama.** 2 June is a Tuesday. Built
for `JAPAN` in `JP-14-100` and 2026, the calendar has one entry on it,
開港記念日, `Kind::School`, citing 横浜市開港記念日条例 (令和7年横浜市条例第21号)
and the school rule's Article 4, whose item 7 reads 「開港記念日　6月2日」.
Built in `JP-14`, Kanagawa, or in `JP-14-130`, Kawasaki, it has nothing:
the day is the city's. `is_business_day` is true in all three: the city's
schools close, and nothing else does. For 2020 the calendar in
`JP-14-100` reports the day as a gap, the article's text before 2021 not
having been read. The boundary writes the 2026 entry of
`hc_holidays_in_year` with `JP-14-100` in the region column, and on
14 November, asked in `JP-11-100`, Saitama's 県民の日 with `JP-11`, the
prefecture's code, since the city's calendar has it only through its
prefecture's.

**Worked example: 1 October 2026 in Tokyo.** 1 October is a Thursday.
Built for `JAPAN` and 2026 with no region, the calendar has nothing on it.
Built in `JP-13` it has one entry, 都民の日, `Kind::School`, with the
instrument "都民の日条例 (昭和27年東京都条例第75号); a 休業日 of
東京都立学校の管理運営に関する規則 … Article 5, as the article reads since
平成14年東京都教育委員会規則第19号, in force 1 April 2002". Built for 2001 the
same entry is `Kind::Observance` citing the ordinance alone: the article's
notes list five amendments from 1962 to 2002, and which of them, if any,
added the item for 都民の日 was not read. `is_holiday` is false and
`is_business_day` true in both, because the ordinance closes nothing and
the school rule closes only the metropolitan schools: an office worker in
Shinjuku works, a pupil of a 都立高校 does not. The ordinance came into
force on that same day in 1952, so 1952 is the first year and 1951 has no
entry.

**Worked example: Golden Week 2019.** The accession made 1 May 2019 a
holiday treated as a 祝日. The year's late-April and May days:

| Date | Day | What | Why |
| --- | --- | --- | --- |
| 27 April | Sat | weekend | |
| 28 April | Sun | weekend | |
| 29 April | Mon | 昭和の日 | Article 2 |
| 30 April | Tue | 国民の休日 | between 29 April and 1 May, both 祝日 for Article 3 |
| 1 May | Wed | 天皇の即位の日 | 平成30年法律第99号 |
| 2 May | Thu | 国民の休日 | between 1 and 3 May |
| 3 May | Fri | 憲法記念日 | Article 2 |
| 4 May | Sat | みどりの日 | Article 2; a Saturday, no substitute |
| 5 May | Sun | こどもの日 | Article 2 |
| 6 May | Mon | 振替休日 | for Sunday 5 May, the nearest following non-祝日 |

Ten days without a working day, and a calendar built for `JAPAN` and 2019
says so: `name_on` gives the names in the table for 29 April to 5 May,
`on` for 6 May returns "Children's Day" with `observed_for` 5 May,
`is_business_day` is false from 27 April to 6 May, and `add_business_days`
from Friday 26 April by one is Tuesday 7 May. The engine's order produced
it: the base rules (29 April, 1, 3, 4, 5 May), then the substitute for
5 May, walking from Monday 6 May, which is free; then the bridges, seeing
30 April and 2 May each between two base days and neither a Sunday nor
occupied.

A second shape, the same rule on the equinox: in 2026 敬老の日 is Monday
21 September and 秋分の日 Wednesday 23 September, so Tuesday 22 September
is a 国民の休日 — the "Silver Week" that appears whenever the third
Monday and the equinox fall two days apart, as in 2015 and, per the
Cabinet Office's list, 2026 [cao-shukujitsu]. And the 2007 rewording:
3 May 2015 was a Sunday; 4 and 5 May are 祝日, so the 振替休日 was
Wednesday 6 May, which the old wording could not have reached.

## What is carried

- **Every 国民の祝日 from 20 July 1948**, with the year range of each date
  and name, the eleven amendments and the two Olympic years, as rules.
- **The four imperial one-offs and the two accession days of 2019.**
- **振替休日** in its two wordings and **国民の休日** from 1986.
- **The equinox days**, computed, for any year the solar model covers.
- **The prefectures' own days**: every day a prefecture set by ordinance
  as its own — a 県民の日, 都民の日 or the like, named for the prefecture,
  its people or its home — and every day a 休日条例 makes a 県の休日 beyond
  the national ones, each scoped to its prefecture, in the table below.
- **The prefectures' other days**: the days set by ordinance for a cause
  that the survey met, and every 教育の日 a prefecture set by an
  instrument of its own, in the second table below.
- **The designated cities' days**: every day the twenty 政令指定都市 set by
  an instrument of their own, with 長崎's ながさき平和の日, in the third
  table below, and 相模原's as a gap.
- **Not carried, and why:**
  - Holidays before 20 July 1948, under the pre-war 休日ニ関スル件 of
    1927 that the Act repealed: the Act's own history begins there, and
    the table with it. 1948 is a half year: 春分の日, 天皇誕生日, 憲法記念日
    and こどもの日 had already passed when the Act came into force, and
    only 秋分の日, 文化の日 and 勤労感謝の日 were holidays that year.
  - The 年末年始 closure of government offices, 29 December to 3 January
    under the 行政機関の休日に関する法律, which is not a 国民の祝日; the
    exchange carries its own version of it.
  - The prefectural days found without a readable instrument, Akita's
    県の記念日 and Ehime's 県政発足記念日, which are gaps in every year, and
    Aichi's school holiday, whose days are in PDF lists; see the tables.
    The prefectures where no ordinance was found are listed among the
    table's subdivisions read, from the year of their 休日条例, 1989, and
    asked for, answer with the national days and no gap from then; the
    years before are a gap, since the ordinances read are those in force
    now and the regime before the 休日条例 was not read (see the
    paragraph after the table of prefectures).
  - Not yet carried: the days listed under "Not yet carried" after the
    tables, and every municipality's days beyond the twenty designated
    cities and 長崎, which no survey has read yet. Asked for, such a city
    answers with its prefecture's days and a gap for its own.
  - The 暦要項's tabulated equinox dates as such: the crate computes the
    same quantity the Observatory computes, and the published dates are
    the check, not the source.

### The forty-seven prefectures

Read on 2026-09-28, each in the prefecture's own 例規集 or, where the
例規集 answers only a submitted form, in the copy the 条例Webアーカイブ
of Doshisha University took of it in July 2026 [doshisha-jorei-archive]
(marked †). For every prefecture the 休日条例 was read, and none but
Okinawa's has a day beyond Sundays, Saturdays, the 国民の祝日 and
29 December to 3 January [jp-pref-holiday-ordinances]. "Schools" is the
prefectural high schools' rule of the board of education, and "school from
YYYY" is the first year a text of it read shows the day among its 休業日:
the effective date of the last amendment the article's notes list, or,
where the text shows no amendment dates, the date of the earliest copy
read [doshisha-jorei-archive]. Before that year the day is carried as the
observance its ordinance makes it, and whether the schools closed on it is
a gap waiting on the rule's earlier texts. The first year
is the first year the ordinance was in force on the day. The English names
are this crate's translations; none is an official one. "No ordinance
found" means no ordinance, rule or notice setting a day of the
prefecture's own was found by the search described after the table.

| Code | Prefecture | Day | Instrument | What it does | Schools | First year | Kind |
| --- | --- | --- | --- | --- | --- | --- | --- |
| JP-01 | 北海道 | 北海道みんなの日 (Hokkaido Everyone's Day), 17 July | 北海道みんなの日条例, 平成29年北海道条例第39号, in force 2017-03-31 [jp-pref-01] | events, fees waived | not listed | 2017 | observance |
| JP-02 | 青森県 | no ordinance found | | | | | |
| JP-03 | 岩手県 | no ordinance found | | | | | |
| JP-04 | 宮城県 | no ordinance found † | | | | | |
| JP-05 | 秋田県 | 県の記念日, 29 August, which the prefecture's page says was set in 1965: instrument not found | | fees waived, by the page | not listed | | not carried |
| JP-06 | 山形県 | no ordinance found | | | | | |
| JP-07 | 福島県 | 福島県民の日 (Fukushima Citizens' Day), 21 August | 福島県民の日条例, 平成9年福島県条例第61号, in force 1997-07-11 [jp-pref-07] † | events, fees waived | not listed | 1997 | observance |
| JP-08 | 茨城県 | 県民の日 (Ibaraki Citizens' Day), 13 November | 県民の日を定める条例, 昭和43年茨城県条例第3号, in force 1968-03-30 [jp-pref-08] | events | school from 2011 (茨城県県立学校管理規則 Article 8, as amended by 平成23年教委規則第8号, in force 2011-07-01) | 1968 | observance to 2010, school from 2011 |
| JP-09 | 栃木県 | 県民の日 (Tochigi Citizens' Day), 15 June | 栃木県県民の日に関する条例, 昭和60年栃木県条例第27号, in force 1985-09-30 [jp-pref-09] | events, fees waived | not listed | 1986 | observance |
| JP-10 | 群馬県 | 群馬県民の日 (Gunma Citizens' Day), 28 October | 群馬県民の日を定める条例, 昭和60年群馬県条例第5号, in force 1985-04-01 [jp-pref-10] | events, fees waived | school from 2014 (群馬県立高等学校管理に関する規則 Article 5, as amended by 平成26年教委規則第9号, in force 2014-04-01) | 1985 | observance to 2013, school from 2014 |
| JP-11 | 埼玉県 | 県民の日 (Saitama Citizens' Day), 14 November | 県民の日を定める条例, 昭和46年埼玉県条例第58号, in force 1971-10-15 [jp-pref-11] | events, fees waived | school from 2017 (埼玉県立高等学校通則 Article 7, in force by 2017-01-19, the earliest copy read) | 1971 | observance to 2016, school from 2017 |
| JP-12 | 千葉県 | 県民の日 (Chiba Citizens' Day), 15 June | 県民の日を定める条例, 昭和59年千葉県条例第3号, in force 1984-03-26 [jp-pref-12] | events, fees waived | school from 2016 (県立高等学校管理規則 Article 7, in force by 2016-06-06, the earliest copy read) | 1984 | observance to 2015, school from 2016 |
| JP-13 | 東京都 | 都民の日 (Tokyo Citizens' Day), 1 October | 都民の日条例, 昭和27年東京都条例第75号, in force 1952-10-01 [jp-pref-13] | facilities opened, events, fees reduced | school from 2002 (東京都立学校の管理運営に関する規則 Article 5, as amended by 平成14年教委規則第19号, in force 2002-04-01) | 1952 | observance to 2001, school from 2002 |
| JP-14 | 神奈川県 | no ordinance found | | | | | |
| JP-15 | 新潟県 | no ordinance found | | | | | |
| JP-16 | 富山県 | 県民ふるさとの日 (Toyama Hometown Day), 9 May | 県民ふるさとの日を定める条例, 平成25年富山県条例第9号, in force 2013-04-01 [jp-pref-16] † | events, fees waived | not listed | 2013 | observance |
| JP-17 | 石川県 | no ordinance found | | | | | |
| JP-18 | 福井県 | ふるさとの日 (Fukui Hometown Day), 7 February | ふるさとの日に関する条例, 昭和57年福井県条例第1号, in force 1982-03-23 [jp-pref-18] | events | not listed | 1983 | observance |
| JP-19 | 山梨県 | 県民の日 (Yamanashi Citizens' Day), 20 November | 県民の日条例, 昭和61年山梨県条例第1号, in force 1986-03-26 [jp-pref-19] | events, fees waived | school from 2002 (山梨県立学校管理規則 Article 3, as amended by 平成14年教委規則第2号, in force 2002-04-01) | 1986 | observance to 2001, school from 2002 |
| JP-20 | 長野県 | no ordinance found | | | | | |
| JP-21 | 岐阜県 | no ordinance found | | | | | |
| JP-22 | 静岡県 | 県民の日 (Shizuoka Citizens' Day), 21 August | 静岡県県民の日条例, 平成8年静岡県条例第23号, in force 1996-03-28 [jp-pref-22] † | events, fees waived | not listed | 1996 | observance |
| JP-23 | 愛知県 | あいち県民の日 (Aichi Citizens' Day), 27 November | あいち県民の日条例, 令和4年愛知県条例第50号, in force 2022-12-23 [jp-pref-23] | events 21–27 November, fees may be waived | a day of 21–27 November each school chooses: a gap | 2023 | observance; the school day a gap |
| JP-24 | 三重県 | 県民の日 (Mie Citizens' Day), 18 April | 県民の日条例, 昭和51年三重県条例第2号, in force 1976-03-29 [jp-pref-24] | events | not listed | 1976 | observance |
| JP-25 | 滋賀県 | no ordinance found | | | | | |
| JP-26 | 京都府 | no ordinance found | | | | | |
| JP-27 | 大阪府 | no ordinance found | | | | | |
| JP-28 | 兵庫県 | no ordinance found | | | | | |
| JP-29 | 奈良県 | no ordinance found † | | | | | |
| JP-30 | 和歌山県 | ふるさと誕生日 (Wakayama Hometown Birthday), 22 November | ふるさと誕生日条例, 平成元年和歌山県条例第38号, in force 1989-07-10 [jp-pref-30] | events | not listed | 1989 | observance |
| JP-31 | 鳥取県 | とっとり県民の日 (Tottori Citizens' Day), 12 September | とっとり県民の日条例, 平成10年鳥取県条例第13号, in force 1998-06-26 [jp-pref-31] | events, fees waived | not listed | 1998 | observance |
| JP-32 | 島根県 | no ordinance found † | | | | | |
| JP-33 | 岡山県 | no ordinance found | | | | | |
| JP-34 | 広島県 | no ordinance found | | | | | |
| JP-35 | 山口県 | no ordinance found † | | | | | |
| JP-36 | 徳島県 | no ordinance found | | | | | |
| JP-37 | 香川県 | 香川県民の日 (Kagawa Citizens' Day), 3 December | 香川県民の日条例, 令和8年香川県条例第1号, in force 2026-03-19 [jp-pref-37] | events 1–7 December, fees waived | school from 2026 (県立学校学則 Article 5, as amended on 2026-03-30) | 2026 | school |
| JP-38 | 愛媛県 | 県政発足記念日, 20 February, kept with a governor's award since 1973: instrument not found | | | | | not carried |
| JP-39 | 高知県 | no ordinance found | | | | | |
| JP-40 | 福岡県 | no ordinance found † | | | | | |
| JP-41 | 佐賀県 | no ordinance found | | | | | |
| JP-42 | 長崎県 | no ordinance found † | | | | | |
| JP-43 | 熊本県 | no ordinance found | | | | | |
| JP-44 | 大分県 | no ordinance found † | | | | | |
| JP-45 | 宮崎県 | no ordinance found | | | | | |
| JP-46 | 鹿児島県 | 県民の日 (Kagoshima Citizens' Day), 14 July | 鹿児島県県民の日を定める条例, 平成30年鹿児島県条例第45号, in force 2018-12-25 [jp-pref-46] | events, fees waived | not listed | 2019 | observance |
| JP-47 | 沖縄県 | 慰霊の日 (Okinawa Memorial Day), 23 June | 沖縄県慰霊の日を定める条例, 昭和49年沖縄県条例第42号, in force 1974-10-21; 沖縄県の休日を定める条例, 平成3年沖縄県条例第15号, Article 1 item 4, in force 1991-05-26 [jp-pref-47] | a 県の休日: the prefecture's offices closed | 休業日 | 1991 as a 県の休日; 1972–1990 a gap (the prefecture was restored on 15 May 1972; before it the islands were the 琉球政府's) | government |

The two amendments found change neither a date nor a name: 平成19年静岡県条例
第42号 reworded Shizuoka's Article 3, and 栃木県 amended its fee article in
1995 and 2005. No other carried ordinance shows an amendment.

**The years the prefectures with no day were read for.** The
ordinances read are the ones in force now, and the 例規集 does not show a
day a prefecture set and later repealed. Each prefecture's 休日条例 of
1989 began the regime the 例規集 shows, so a prefecture with no day of its
own is answered from the year of its 休日条例 and is a gap before it
(`Subdivisions::ReadFrom`, ADR 0013; audit 10 a4). The 公布 dates, from the
archive's copies of the 例規集, which give each ordinance's number and
promulgation date, read 2026-10-03: the archive's certificate had expired,
so they were read over that connection without the certificate's check,
through a parser and never saved, and these dates were not checked against
each prefecture's own page, as the others, which agree with every official
page also read, were:

| Region | 休日条例 | 公布 |
| --- | --- | --- |
| JP-02 青森県 | 青森県の休日に関する条例, 平成元年条例第3号 | 1989-03-23 |
| JP-03 岩手県 | 岩手県の休日に関する条例, 平成元年条例第1号 | 1989-03-11 |
| JP-04 宮城県 | 宮城県の休日を定める条例, 平成元年条例第10号 | 1989-02-28 |
| JP-06 山形県 | 山形県の休日を定める条例, 平成元年条例第10号 | 1989-03-22 |
| JP-14 神奈川県 | 神奈川県の休日を定める条例, 平成元年条例第12号 | 1989-03-28 |
| JP-15 新潟県 | 新潟県の休日を定める条例, 平成元年条例第5号 | 1989-03-24 |
| JP-17 石川県 | 石川県の休日を定める条例, 平成元年条例第16号 | 1989-03-24 |
| JP-20 長野県 | 長野県の休日を定める条例, 平成元年条例第5号 | 1989-03-27 |
| JP-21 岐阜県 | 岐阜県の休日を定める条例, 平成元年条例第5号 | 1989-03-28 |
| JP-25 滋賀県 | 滋賀県の休日を定める条例, 平成元年条例第10号 | 1989-03-30 |
| JP-26 京都府 | 京都府の休日を定める条例, 平成元年条例第4号 | 1989-03-30 |
| JP-27 大阪府 | 大阪府の休日に関する条例, 平成元年条例第2号 | 1989-03-27 |
| JP-28 兵庫県 | 兵庫県の休日を定める条例, 平成元年条例第15号 | 1989-03-28 |
| JP-29 奈良県 | 奈良県の休日を定める条例, 平成元年条例第32号 | 1989-03-31 |
| JP-32 島根県 | 島根県の休日を定める条例, 平成元年条例第9号 | 1989-03-25 |
| JP-33 岡山県 | 岡山県の休日を定める条例, 平成元年条例第2号 | 1989-03-28 |
| JP-34 広島県 | 広島県の休日を定める条例, 平成元年条例第2号 | 1989-03-27 |
| JP-35 山口県 | 山口県の休日に関する条例, 平成元年条例第16号 | 1989-07-04 |
| JP-36 徳島県 | 徳島県の休日を定める条例, 平成元年条例第3号 | 1989-03-23 |
| JP-39 高知県 | 高知県の休日を定める条例, 平成元年条例第2号 | 1989-03-24 |
| JP-40 福岡県 | 福岡県の休日を定める条例, 平成元年条例第23号 | 1989-07-11 |
| JP-41 佐賀県 | 佐賀県の休日に関する条例, 平成元年条例第29号 | 1989-07-08 |
| JP-42 長崎県 | 長崎県の休日を定める条例, 平成元年条例第43号 | 1989-07-18 |
| JP-43 熊本県 | 熊本県の休日を定める条例, 平成元年条例第10号 | 1989-03-25 |
| JP-44 大分県 | 大分県の休日を定める条例, 平成元年条例第21号 | 1989-07-10 |
| JP-45 宮崎県 | 宮崎県の休日を定める条例, 平成元年条例第22号 | 1989-07-08 |
| JP-01-100 札幌市 | 札幌市の休日を定める条例, 平成2年条例第23号 | 1990-06-15 |
| JP-04-100 仙台市 | 仙台市の休日を定める条例, 平成元年条例第61号 | 1989-09-22 |
| JP-15-100 新潟市 | 新潟市の休日を定める条例, 平成元年条例第35号 | 1989-10-09 |
| JP-27-100 大阪市 | 大阪市の休日を定める条例, 平成3年条例第42号 | 1991-12-24, in force 1992-04-01 |
| JP-40-100 北九州市 | 北九州市の休日を定める条例, 平成3年条例第2号 | 1991-03-25 |
| JP-40-130 福岡市 | 福岡市の休日を定める条例, 平成2年条例第52号 | 1990-12-22 |

The year is the year of the 公布; the 施行 dates were not read, and a
prefecture whose ordinance came into force in the next year is answered
a year early. The years before it, back to 1948, are a gap. 岩手 and
宮城, which have the earthquake memorial days of their own, are read
from 1989 all the same. A prefecture with a day of its own is not made a
gap by this: it is answered in every year from its own rule's, and
absent before it, as the ordinance says.

**How "no ordinance found" was searched.** Where the 例規集 is a static
index (Aomori, Iwate, Akita, Yamagata, Ibaraki, Tochigi, Shiga, Kyoto,
Osaka, Wakayama, Gifu, Tottori, Okayama, Tokushima, Kagawa, Saga,
Kagoshima and others), every title in its 五十音 index was read for の日,
記念日, 県民, 府民, 誕生 and ふるさと, and for Kyoto, Osaka, Shiga, Gifu and
Wakayama the body of every 条例 was searched for an article headed
(…の日). Where the 例規集 is a d1-law database searchable only by form
(Kanagawa, Nagano, Hyogo, Hiroshima, Kochi, Ehime), the 休日条例 was read
by its address and the rest rests on web searches and the prefecture's
own pages: Kanagawa's says its 立庁記念日 is a date of its history, not a
day it keeps. Where it answers only a form (Miyagi, Nara, Shimane,
Yamaguchi, Fukuoka, Nagasaki, Oita), the archive's title and full-text
search was used, limited to the prefecture. Hyogo's and Nagano's are the
weakest negatives: no full-text search of either was possible.

**Akita and Ehime, searched again.** On 2026-09-29 the archive's full
text of each prefecture's 例規集 was searched for 県の記念日, 県政発足,
発足記念 and the dates; nothing sets either day. Akita's rules name
8月29日 only to keep its archives, library and museums open on it, and
Ehime's 1973 公告 of 20 February concerns its emblem and song, not the
day. Both stay gaps [jp-pref-ordinance-days].

### The prefectures' other days

Read on 2026-09-29, in the archive's copy of each prefecture's 例規集 or
the 例規集 itself [jp-pref-ordinance-days, jp-pref-education-days]. The
first year is the first year the instrument was in force on the day. None
closes anything; schools were not checked except where the first survey
read the school rule.

| Code | Prefecture | Day | Instrument | First year |
| --- | --- | --- | --- | --- |
| JP-32 | 島根県 | 竹島の日 (Takeshima Day), 22 February | 竹島の日を定める条例, 平成17年島根県条例第36号, in force 2005-03-25 | 2006 |
| JP-22 | 静岡県 | 富士山の日 (Mount Fuji Day), 23 February | 静岡県富士山の日条例, 平成21年静岡県条例第72号, 2009-12-25 | 2010 |
| JP-19 | 山梨県 | 富士山の日, 23 February | 山梨県富士山の日条例, 平成23年山梨県条例第55号, 2011-12-22 | 2012 |
| JP-13 | 東京都 | 東京都平和の日 (Tokyo Peace Day), 10 March | 東京都平和の日条例, 平成2年東京都条例第90号, 1990-07-20; 記念行事 | 1991 |
| JP-03 | 岩手県 | 東日本大震災津波を語り継ぐ日, 11 March | 令和3年岩手県条例第1号, 2021-02-19 | 2021 |
| JP-04 | 宮城県 | みやぎ鎮魂の日, 11 March | 平成25年宮城県条例第18号, 2013-04-01 | 2014 |
| JP-04 | 宮城県 | みやぎ県民防災の日, 12 June | 震災対策推進条例, 平成20年宮城県条例第62号, Article 26, 2009-04-01 | 2009 |
| JP-03 | 岩手県 | 平泉世界遺産の日, 29 June | 平成26年岩手県条例第17号, 2014-03-28 | 2014 |
| JP-25 | 滋賀県 | びわ湖の日 (Lake Biwa Day), 1 July | 滋賀県環境基本条例, 平成8年滋賀県条例第18号, Article 8, 1996-07-01 | 1996 |
| JP-21 | 岐阜県 | 飛騨・美濃じまんの日, 21 August | みんなでつくろう観光王国飛騨・美濃条例, 平成19年岐阜県条例第39号, Article 15, 2007-10-01 | 2008 |
| JP-47 | 沖縄県 | しまくとぅばの日, 18 September | 平成18年沖縄県条例第35号, 2006-03-31 | 2006 |
| JP-47 | 沖縄県 | 琉球歴史文化の日, 1 November | 令和3年沖縄県条例第13号, 2021-03-31; fees waived | 2021 |
| JP-03 | 岩手県 | いわて教育の日, 1 November | 平成17年岩手県条例第41号, 2005-04-01 | 2005 |
| JP-04 | 宮城県 | みやぎ教育の日, 1 November | 平成17年宮城県条例第90号, 2005-04-01 | 2005 |
| JP-07 | 福島県 | ふくしま教育の日, 1 November | 平成15年福島県条例第50号, 2003-03-24; fees waived in the week | 2003 |
| JP-08 | 茨城県 | いばらき教育の日, 1 November | 平成16年茨城県条例第35号, 2004-06-16 | 2004 |
| JP-11 | 埼玉県 | 彩の国教育の日, 1 November | 彩の国教育の日を定める要綱, 平成15年県・教育委員会告示第1号, 2003-01-24 | 2003 |
| JP-13 | 東京都 | 東京都教育の日, the first Saturday of November | the board's decision of February 2004, by its page | 2004 |
| JP-15 | 新潟県 | 新潟県教育の日, 1 November | 令和4年新潟県条例第49号, 2022-12-27 | 2023 |
| JP-06 | 山形県 | やまがた教育の日, the second Saturday of November | やまがた教育の日を定める要綱, by the board's page, year not given | read from 2026 |
| JP-17 | 石川県 | いしかわ教育の日, 1 November | 平成17年石川県条例第32号, 2005-03-22 | 2005 |
| JP-25 | 滋賀県 | 滋賀教育の日, 1 November | 「滋賀 教育の日」を定める要綱, 2006-06-01, by the board's page | 2006 |
| JP-29 | 奈良県 | 奈良県教育の日, 1 November | 奈良県教育委員会告示第6号, 2003-07-01, by the board's page | 2003 |
| JP-32 | 島根県 | しまね教育の日, 1 November | 平成14年島根県条例第66号, 2002-10-25 | 2002 |
| JP-33 | 岡山県 | おかやま教育の日, 1 November | 平成13年岡山県条例第58号, 2001-06-26 | 2001 |
| JP-34 | 広島県 | ひろしま教育の日, 1 November | 平成13年広島県条例第40号, 2001-10-10 | 2001 |
| JP-36 | 徳島県 | とくしま教育の日, 1 November | 平成16年徳島県条例第35号, 2004-03-31 | 2004 |
| JP-44 | 大分県 | おおいた教育の日, 1 November | 平成17年大分県条例第30号, 2005-03-31 | 2005 |

The first survey's "ten prefectures" with an education day on 1 November
were short by the three it had not met, 宮城, 福島 and 広島. The archive's
title and full-text search over the forty-seven for 教育の日, 教育週間 and
教育月間 found no other instrument.

### The designated cities

Read on 2026-09-29, each in the city's 例規集 where it answers a plain
request, and else in the archive's copy of July 2026
[jp-city-designated]; the codes are from [jis-x0402-cities]. For every
city the 休日条例 and the school rule's 休業日 article were read. "School"
is the city's own schools.

| Code | City | Day | Instrument | Schools | First year | Kind |
| --- | --- | --- | --- | --- | --- | --- |
| JP-01-100 | 札幌市 | no instrument found; four of the index's pages searched | | | | |
| JP-04-100 | 仙台市 | no instrument found | | | | |
| JP-11-100 | さいたま市 | さいたま市民の日, 1 May | さいたま市民の日条例, 令和3年さいたま市条例第1号, 2021-03-11; fees waived | 休業日 from 2021-04-01 | 2021 | school |
| JP-12-100 | 千葉市 | 千葉市の市民の日, 18 October; the prefecture's 県民の日, 15 June, as an observance only | 平成7年千葉市告示第373号, 1995-12-18 | not listed; 千葉市立小学校及び中学校管理規則 Article 19-2 (as amended to 令和7年教委規則第5号, read 2026-10-03) lists no 県民の日 [jp-chiba-city-school-rule], and a page on the city's schools says it moves the day to the autumn break | 1996 | observance |
| JP-14-100 | 横浜市 | 開港記念日, 2 June | 横浜市開港記念日条例, 令和7年横浜市条例第21号, 2025-03-31; before it the city assembly's resolutions, not read | 休業日; the article as amended in January 2021 | read from 2021 | school |
| JP-14-130 | 川崎市 | 市制記念日, 1 July | 昭和12年川崎市告示第163号, 1937-06-25 | 休業日, in the text of 2026, the earliest copy read | 1937 | observance to 2025, school from 2026 |
| JP-14-150 | 相模原市 | 市制施行記念日, named in 相模原市表彰条例 Article 10 without a date | | not listed | | gap |
| JP-15-100 | 新潟市 | no instrument found | | | | |
| JP-22-100 | 静岡市 | お茶の日, 1 November | 静岡市めざせ茶どころ日本一条例 Article 9; 平成22年静岡市告示第106号, 2010-03-20 | not listed | 2010 | observance |
| JP-22-130 | 浜松市 | 市制記念日, 1 July | named by 平成9年浜松市条例第62号, in force 1997-04-01, which waives fees on it | not listed | read from 1997 | observance |
| JP-23-100 | 名古屋市 | なごや平和の日, 14 May | なごや平和の日を定める条例, 令和6年名古屋市条例第36号, 2024-04-01 | not listed | 2024 | observance |
| JP-26-100 | 京都市 | 京都市自治記念日, 15 October | 京都市自治記念日について, 昭和33年9月3日公告 | not listed | 1958 | observance |
| JP-26-100 | 京都市 | 憲章の日, 5 February | 平成23年京都市条例第72号 Article 16, 2011-04-01 | not listed | 2012 | observance |
| JP-26-100 | 京都市 | 伝統産業の日, 春分の日 | 京都市伝統産業活性化推進条例, 平成17年京都市条例第21号, Article 15, 2005-10-15 | not listed | 2006 | observance |
| JP-26-100 | 京都市 | 食の安全安心推進の日, 1 August | 平成22年京都市条例第59号 Article 15, 2010-10-01 | not listed | 2011 | observance |
| JP-27-100 | 大阪市 | no instrument found | | | | |
| JP-27-140 | 堺市 | 開庁記念日, 26 July | named by 堺市有功章条例, 昭和46年堺市条例第7号, Article 4, as its awards' day | not listed | read from 1971 | observance |
| JP-28-100 | 神戸市 | 市民防災の日, 17 January | 神戸市民の安全の推進に関する条例, 平成10年神戸市条例第49号, Article 24, 1998-01-17 | not listed | 1998 | observance |
| JP-33-100 | 岡山市 | 岡山市民の日, 1 June | the mayor's decision of 2012-03-22, by the city's page; no 告示 found | not listed | 2012 | observance |
| JP-34-100 | 広島市 | 平和記念日, 6 August | 広島市役所事務休停日条例, 昭和22年広島市条例第14号, 1947-07-31, repealed 2021; 広島市の休日を定める条例, 平成3年広島市条例第49号, Article 1 item 4, 1991-12-01; 広島市平和推進基本条例, 令和3年広島市条例第50号, Article 6 | not listed; within the summer vacation | 1947 | government |
| JP-40-100 | 北九州市 | no instrument found | | | | |
| JP-40-130 | 福岡市 | no instrument found | | | | |
| JP-43-100 | 熊本市 | 熊本地震の日, 16 April | 熊本市防災基本条例, 令和4年熊本市条例第33号, Article 16, 2022-10-01 | not listed | 2023 | observance |
| JP-43-100 | 熊本市 | 市民健康の日, 1 October | 熊本市市民健康の日を定める条例, 昭和61年熊本市条例第12号, 1986-04-01 | not listed | 1986 | observance |
| JP-42-201 | 長崎市 | ながさき平和の日, 9 August | ながさき平和の日条例, 平成7年長崎市条例第2号, 1995-03-23 | not listed | 1995 | observance |

Every 休日条例 read, of the twenty and of 長崎, lists Sundays, Saturdays,
the national holidays and the days of the year's end and beginning, and
広島's alone adds a day. さいたま's school rule also lists Saitama's
県民の日 among its 休業日, which the prefecture's own rule already carries
as a school day. 札幌's is the weakest negative: four of the thirty-eight
pages of its title index were read.

### Not yet carried

- Every municipality's days beyond the twenty designated cities and 長崎:
  the next survey is the 中核市 and the other cities, and Tokyo's
  special wards. A city not read answers with its prefecture's days and
  a gap.
- The education days no instrument of the prefecture sets: 北海道's and
  愛媛's, declared by councils of education bodies, 高知's by a forum's
  declaration, 長野's by three private bodies, 香川's by an executive
  committee with no date on the page read; and 秋田's あきた教育の日, whose
  setter was not found. They need a decision whether a council's
  declaration is an instrument the table cites.
- Shiga's 環境美化の日 of 30 May and 1 December, whose article's wording
  before its amendments of 1996 and 2001 was not shown; the monthly days (浜松's 市民交通安全の日,
  名古屋's 環境保全の日, 川崎's 市民地震防災デー); 新潟's 防犯の日, the third
  Wednesday of October; and 神戸港's and 神戸開港's days, for which no
  instrument of the city was found.
- Aichi's school holiday day by day, which needs the board's PDF lists;
  横浜's 開港記念日 before 2021, which needs the city assembly's
  resolutions of 1918 and 1928 and the school rule's earlier texts; and
  相模原's 市制施行記念日, which needs an instrument that dates it.

## Accuracy

The amendments are pinned one by one in `crates/hc-holiday/tests/japan.rs`:
the 1948 half year and 1949, the 1966 additions and 建国記念の日's 1967
start, the first 振替休日 on 30 April 1973 and the absence of one on
12 February 1973 and before 1973, the bridge rule absent in 1985 and first
producing 4 May 1988, its Sunday exclusion, the 1989 move of 天皇誕生日,
each imperial one-off in its year alone, 海の日 from 1996, the two
ハッピーマンデー waves, the 2007 changes and the rewording's first effect
on 6 May 2009 and 2015, 山の日 from 2016, 2019 with no 天皇誕生日, the
ten-day Golden Week of 2019 and the enthronement day, the 2020 and 2021
Olympic moves and スポーツの日's return in 2022, the growth of the count
(9, 9, 12, 14, 16 for 1950, 1965, 1967, 1997, 2017), and a whole recent
year, 2024, against the Cabinet Office's published list with its two
振替休日. `the_equinox_holidays_match_the_dates_japan_published` checks a
dozen spring and nine autumn equinox days against the 暦要項's dates,
including the autumn equinox of 2012 on 22 September, the first since
1896.

The equinox computation is checked in `hc-seasons`
(`crates/hc-seasons/tests/japanese_equinox_days.rs`) against the published
days for 1980 to 2030 transcribed into the test, 102 dates, with no
disagreement, and against a floor formula for 1980 to 2099 that
reproduces the published table. The Observatory publishes one year ahead
— the latest 暦要項 is for 2027 — and its FAQ gives predicted dates only
to 2050, with a caveat. The 2031–2099 comparison is therefore against the
formula, which the test file gives without attribution and which
circulates in Japanese references as a simplification of the 1980 epoch
and the tropical year: a comparison with a prediction, not with a
publication, as `hc-holiday`'s and `hc-seasons`'s READMEs and
`hc-seasons`'s module documentation say.

Known points a reader may stumble on:

- The 天皇誕生日 move is in the 皇室典範特例法 (平成29年法律第63号), in
  force 1 May 2019, not in 平成30年法律第99号, which gave only the two 2019
  days; the 2020 Olympic moves are 平成30年法律第55号 and the renaming
  第57号, both of 20 June 2018, not 平成30年法律第76号.
- The 1985 act came into force on promulgation, 27 December 1985. The
  table dates the bridge from 1986, which is the first year a day could
  have been trapped.
- Under the 1973 wording the module says a Sunday holiday whose Monday
  was itself a 祝日 produced nothing. That is what the wording implies,
  and the policy is modelled that way, but no such day occurred between
  1973 and 2006: the only adjacent 祝日 then were 3 and 5 May with 4 May
  between them, and 4 May was not a 祝日 until 2007.

**The prefectures** are pinned in
`crates/hc-holiday/tests/japan_prefectures.rs`: each day in its first
year and in 2026, absent the year before it began, its kind and its
instrument; 都民の日 in `JP-13` and in no other region nor nationwide;
Chiba's and Tochigi's 15 June each in its own prefecture; the region
matched in either case; no prefectural day a day off; Okinawa's gap for
1972–1990 and nothing in 1971; the prefectures and cities read for no day a
gap before their 休日条例 of 1989 and later; 千葉市's 15 June an observance
and the prefecture's a school day; each of the other days in its first year
and 2026 and not the year before, 東京's and 山形's Saturdays, and Aichi's
gap from 2023. The cities are pinned in
`crates/hc-holiday/tests/japan_municipalities.rs`: each day in its first
year and 2026, its kind, and the year before either empty or, for the
days read from an instrument's first year, a gap; さいたま市 with Saitama's
県民の日 and its own; 開港記念日 in Yokohama and not in Kawasaki; 伝統産業の日
on the equinox of 2006 and 2026; 平和記念日 closing Hiroshima's offices and
no business; the cities read without a day complete; 鎌倉市 a gap. The
scope itself is pinned in `crates/hc-holiday/tests/municipal_scope.rs`
on an invented table, and the boundary's region column in
`holiday_lines`'s test `a_city_s_lines_name_the_widest_region_whose_entry_each_is`.
What the tests cannot check, and a reader should know:

- `Kind::School` starts at the first year a text of the school rule read
  shows the day, and the years before are observances. Those earlier
  years are gaps in what is known, not findings that the schools opened:
  Tokyo's 1960 rule, Yamanashi's of 1961 (its day of 1986) and the others
  may have listed the day long before, and the rule's text as it stood in
  each earlier year, or the amendment that added the item, would settle
  it. For Ibaraki, Tokyo and Yamanashi the article's notes name the
  amendments, from 昭和41年 (Ibaraki), 昭和37年 (Tokyo) and 昭和44年
  (Yamanashi), so the texts to read are those; for Gunma, Saitama and
  Chiba the d1-law text shows no article history, and the earliest copy
  read is the evidence.
- Kagawa's closure is confirmed by the archive's copies: the 学則 in its
  versions to 2023 does not list the day, and the version of 30 March
  2026 does.
- Seven prefectures were read in the archive's copy of their 例規集 rather
  than the 例規集 itself, and the rows mark them. The copy is of July 2026.
- The first 休日条例 of Okinawa may have worded its Saturday item
  differently before 平成4年沖縄県条例第43号; the 慰霊の日 item appears to be
  of 1991, which is what the table carries.
- The cities' codes are from Wikipedia's infoboxes, checked against the
  check digit and the archive's own municipality identifiers, not from
  JIS X 0402 itself.
- Most other days and six cities were read in the archive's copy, whose
  host's certificate had expired on the day read; the text was read over
  that connection without the certificate's check. The copies agree with
  every official page that was also read (山梨, 東京, the Aichi 学則).
- An instrument dated after the day in its year is taken to begin the
  next year, and one with no stated commencement to be in force from its
  date: 千葉's 告示, 川崎's 告示 and 京都's 公告 state none.
- The Japanese holiday laws did not apply in Okinawa before the reversion
  of 15 May 1972, and the nationwide rules are not scoped away from
  `JP-47` for those years; that is a gap in the table's nationwide rules,
  not in the prefectural ones.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [jp-holiday-act] | The Act as in force: Articles 1 to 3, the sixteen holidays and their purposes, the wording of the 振替休日 and 国民の休日, the list of amending laws and their commencement | Yes, 2026-09-25, the e-Gov law API's XML; e-Gov's page renders by script |
| [wikisource-ja-holiday-act] | The amendment list with promulgation and commencement dates | Yes, 2026-09-25 |
| [jp-holiday-act-amendment-2005] | 平成17年法律第43号: the old and new wording of Article 3 paragraph 2, 昭和の日, みどりの日, in force 1 January 2007 | Yes, 2026-09-25, the Wikisource copy |
| [jp-imperial-succession-act-2017] | 平成29年法律第63号, whose 附則 moved 天皇誕生日 to 23 February from 1 May 2019 | Through the Cabinet Office's page and the e-Gov amendment list, 2026-09-25; the act's text not read |
| [jp-accession-holidays-act-2018] | 平成30年法律第99号: 1 May and 22 October 2019, treated as 国民の祝日 for Article 3 | Yes, 2026-09-25, the e-Gov law API |
| [jp-olympic-laws-2018] | 平成30年法律第55号 (the 2020 special dates) and 第57号 (スポーツの日), both of 20 June 2018 | Yes, 2026-09-25, the Sports Agency's notice |
| [nao-topics-2021-holidays] | 令和2年法律第68号 and the 2021 dates; the 暦要項 reissued | Yes, 2026-09-25 |
| [jp-akihito-wedding-holiday-act-1959] | 昭和34年法律第16号, 17 March 1959 | The Diet Library's index entry, 2026-09-25; the other three one-off acts as the module cites them, not read |
| [cao-shukujitsu] | The Cabinet Office's statement of the rules and its yearly lists; 2026's 22 September | Yes, 2026-09-25 |
| [cao-shukujitsu-kaku] | The history of each holiday: 建国記念の日's 政令, 海の日's dates, 山の日, スポーツの日, 天皇誕生日 | Yes, 2026-09-25 |
| [nao-rekiyoko] | What the 暦要項 is and when it appears in the 官報 | Yes, 2026-09-25 |
| [nao-rekiyoko-2027] | The 2027 holidays, the equinox days and the 振替休日 of 22 March; the date of the announcement | Yes, 2026-09-25 |
| [nao-faq-equinox] | That the equinox days are decided by the February publication and that later years are predictions | Yes, 2026-09-25 |
| [jp-local-autonomy-act] | 地方自治法 Article 4-2: the 休日 of a local government, paragraph 3's historical day and paragraph 4's deadlines | Yes, 2026-09-28, the e-Gov law API |
| [jp-bank-act-order] | 銀行法施行令 Article 5: a bank's holidays are the national ones | Yes, 2026-09-28, the e-Gov law API |
| [jp-pref-holiday-ordinances] | The forty-seven 休日条例: their numbers, commencement and the days each lists | Yes, 2026-09-28, in each 例規集 or the archive's copy |
| [jp-pref-01] … [jp-pref-47] | Each prefecture's ordinance: the date, the first year, what it does; the school rule that lists the day | Yes, 2026-09-28; the entries' notes say where each was read |
| [doshisha-jorei-archive] | The copies of the 例規集 that answer only a form | Yes, 2026-09-28, through its search interface; the numbers and 公布 dates of the 休日条例 of 26 prefectures and six cities, 2026-10-03, through its search API (certificate expired; read without its check, through a parser, not saved) |
| [okinawa-archives-irei] | 慰霊の日 under the 琉球政府: 22 June from 1961, 23 June from 1965 | Yes, 2026-09-28; the two acts themselves not read |
| [jp-pref-ordinance-days] | The prefectures' other days: 竹島の日, 富士山の日, びわ湖の日, the memorial days and the rest; the second search for Akita's and Ehime's instruments | Yes, 2026-09-29, the archive's copies and two 例規集 |
| [jp-pref-education-days] | The eleven education-day ordinances, Saitama's 告示, and the four boards' pages | Yes, 2026-09-29; 山形's, 滋賀's and 奈良's instruments themselves not read |
| [jp-aichi-school-holiday] | The 学則's item and the board's candidate days | Yes, 2026-09-29; the lists of the days chosen are PDF, not read |
| [jp-chiba-city-school-rule] | 千葉市立小学校及び中学校管理規則 Article 19-2: the 休業日 of the city's schools, which list no 県民の日; the rule's number and its latest amendment, 令和7年教委規則第5号 | Yes, 2026-10-03, on 千葉市's 例規集 (g-reiki.net) |
| [jp-city-designated] | The twenty designated cities' and 長崎's days, 休日条例 and school rules | Yes, 2026-09-29, each 例規集 or the archive's copy |
| [jis-x0402-cities] | The cities' 全国地方公共団体コード | Yes, 2026-09-29, Wikipedia; JIS X 0402 itself not read |

## Code

`crates/hc-holiday/src/countries/japan.rs`: the rules in `RULES`, the
prefectures' days, their other days and the cities' days last among them,
built by `prefectural` and `municipal`, the two
振替休日 policies in `SUBSTITUTION`, the 国民の休日 policy in `BRIDGES`,
and the table `JAPAN`. `Kind::Government` is in `rule.rs`, and
[ADR 0010](../adr/0010-a-government-office-day-off-is-its-own-kind.md)
records why it is a kind of its own; a city's code is
`hc_holiday::rule::region_parent` and `region_within`, which
[ADR 0014](../adr/0014-a-municipality-is-a-region-within-its-subdivision.md)
records; the region column of the lines is
`hyper_calendar::holiday_lines`. The engine's part is
`SubstitutionPolicy::skip_occupied` and `BridgePolicy` in `rule.rs`, and
the substitution pass, `substitute_day`'s stop on an occupied day, and
the bridge pass of `evaluate` in `engine.rs`; the equinox comes from
`hc_seasons::solar_terms` at `Meridian::JAPAN`. The exchange is
`TOKYO_STOCK_EXCHANGE` in `exchanges.rs`.

Anchors: every test in `crates/hc-holiday/tests/japan.rs` and
`crates/hc-holiday/tests/japan_prefectures.rs`, in particular
`the_first_substitute_holiday_in_japanese_history_was_30_april_1973`,
`the_first_citizens_holiday_was_4_may_1988`,
`the_2007_substitution_rule_walks_past_a_day_that_is_already_a_holiday`,
`the_2019_accession_produced_a_ten_day_golden_week`,
`the_equinox_holidays_match_the_dates_japan_published` and
`a_recent_year_matches_the_cabinet_offices_published_list`,
`every_prefectural_day_is_kept_from_its_first_year_to_today`,
`a_prefectural_day_belongs_to_its_prefecture_alone`,
`okinawa_s_memorial_day_is_a_gap_before_its_holiday_ordinance`,
`every_other_prefectural_day_is_kept_from_its_first_year_and_not_before`,
`every_municipal_day_is_kept_from_its_first_year_to_today`,
`a_city_has_its_prefecture_s_days_and_its_own` and
`yokohama_s_port_opening_day_is_the_worked_example`;
`a_holiday_on_a_sunday_moves_to_the_monday` in
`crates/hc-holiday/src/engine.rs`;
`tokyo_closes_on_japans_holidays_and_its_three_market_holidays` in
`crates/hc-holiday/tests/exchanges.rs`; and
`crates/hc-seasons/tests/japanese_equinox_days.rs` for the equinox days.
