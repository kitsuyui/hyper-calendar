# Vietnam's holidays, the annual Tết notices and the swapped Saturdays

Backs the `VIETNAM` table (`VN`) in `hc-holiday`.

## What it is

Vietnam's holidays are in the Labour Code, and which days they fall on is
in part decided each year. The Bộ luật Lao động, Law 45/2019/QH14, adopted
on 20 November 2019 and in force from 1 January 2021 (article 220(1)), sets
them in article 112(1) [vn-labour-code-2019]:

- "a) Tết Dương lịch: 01 ngày (ngày 01 tháng 01 dương lịch)" — New Year's
  Day;
- "b) Tết Âm lịch: 05 ngày" — five days of the lunar New Year, Tết
  Nguyên Đán;
- "c) Ngày Chiến thắng: 01 ngày (ngày 30 tháng 4 dương lịch)";
- "d) Ngày Quốc tế lao động: 01 ngày (ngày 01 tháng 5 dương lịch)";
- "đ) Quốc khánh: 02 ngày (ngày 02 tháng 9 dương lịch và 01 ngày liền kề
  trước hoặc sau)" — National Day and one adjacent day, before or after;
- "e) Ngày Giỗ Tổ Hùng Vương: 01 ngày (ngày 10 tháng 3 âm lịch)" — the
  Hùng Kings' commemoration on the tenth of the third lunar month.

Article 112(3) leaves two of them open: "Thủ tướng Chính phủ quyết định cụ
thể ngày nghỉ quy định tại điểm b và điểm đ khoản 1" — the Prime Minister
decides which five days are Tết and which day is National Day's second.
Article 111(3) is the make-up rule: "Nếu ngày nghỉ hằng tuần trùng với ngày
nghỉ lễ, tết quy định tại khoản 1 Điều 112 … được nghỉ bù … vào ngày làm
việc kế tiếp" — a weekly rest day that coincides with one of those
holidays is made up on the next working day [vn-labour-code-2019].

From 1 July 2026 there is one more: Nghị quyết 28/2026/QH16 on the
development of Vietnamese culture, article 2, "Ngày 24 tháng 11 hằng năm
là Ngày Văn hóa Việt Nam; người lao động được nghỉ làm việc và hưởng
nguyên lương" — Vietnamese Culture Day, a paid day off [vn-resolution-28-2026].
It is not in article 112(1), so article 111(3)'s make-up does not reach it.

**The notices.** Each year the Prime Minister's decision reaches the civil
service as a notice of the ministry in charge — the Bộ Lao động – Thương
binh và Xã hội to 2025, the Bộ Nội vụ from 2026 — sometimes preceded by a
letter of the Government Office, the Văn phòng Chính phủ. The notice names
Tết's five days and the second National Day day, and it may swap a
weekday for a Saturday: "hoán đổi ngày làm việc", the weekday off and the
Saturday worked, to join a holiday to a weekend. No article of the Code,
nor of the parts of Nghị định 145/2020/NĐ-CP read, provides for the swap;
it is the Prime Minister's direction for the civil service in each notice, and private employers choose their own days and
may follow it or not [vn-holiday-notices].

## How it works

A year is four kinds of day:

1. **The fixed holidays** of article 112(1): 1 January, 30 April, 1 May,
   2 September, and the Hùng Kings' day on the `vietnamese` calendar;
   from 2026, 24 November.
2. **Article 111(3)'s make-up** for any of those on a Saturday or Sunday,
   the civil service's weekly rest days: the next working day.
3. **The notice's days**: Tết's five days and the days it gives in lieu
   of those that fall on a weekend, and the second National Day day.
4. **The notice's swaps**: a weekday off and a Saturday worked.

**Worked example: 2025.** Thông báo 6150/TB-BLĐTBXH of 3 December 2024
gives civil servants Tết from Saturday 25 January to Sunday 2 February
2025, "05 ngày nghỉ tết Âm lịch và 04 ngày nghỉ hằng tuần" — the five Tết
days are Monday 27 to Friday 31 January, and the two weekends around them
are weekends already. It swaps "từ thứ Sáu ngày 02/5/2025 sang thứ Bảy
ngày 26/4/2025" — Friday 2 May off, Saturday 26 April worked — and gives
National Day from Saturday 30 August to Tuesday 2 September, so that the
second day is Monday 1 September [vn-notice-6150-2024]. The fixed days add
1 January, the Hùng Kings' day on Monday 7 April (10/3 of Ất Tỵ), 30 April
and 1 May, and 2 September; none falls on a weekend, so article 111(3)
adds nothing. The year's weekdays off are therefore 1 January, 27 to
31 January, 7 April, 30 April, 1 and 2 May, 1 and 2 September — twelve —
and the one weekend day worked is 26 April.

For article 111(3), 2023: National Day was Saturday 2 September, and the
second day the notice gave was Friday 1 September; the Saturday was made
up on Monday 4 September, as Thông báo 5034/TB-LĐTBXH says.

## What is carried

- **The fixed holidays of article 112(1)** from the Code's entry into
  force, with the make-up of article 111(3); the Hùng Kings' day from
  2007; Vietnamese Culture Day from 2026, not made up.
- **The notices for 2021 to 2026**: Thông báo 4875/TB-LĐTBXH (2021),
  119/TB-LĐTBXH (2022), 5034/TB-LĐTBXH (2023), 5015/TB-LĐTBXH and
  1570/TB-LĐTBXH (2024), 6150/TB-BLĐTBXH (2025), and for 2026 Thông báo
  9441/TB-BNV, Công văn 12729/VPCP-KGVX and Công văn 3383/BNV-CVL, which
  says that no swap was made around 30 April. Their Tết days, second
  National Day days and swaps are `Rule::Tabulated` rules over 2021 to
  2026, and a year outside is a gap.
- **Not carried, and why:**
  - The years before 2021, under the Labour Code of 2012, whose text and
    notices were not read.
  - 2027, whose notices had not been issued, and a swap around 24 November
    2026, which had been proposed and not decided.
  - Private employers' own choice of Tết days.

## Accuracy

`vietnam_keeps_each_years_notices` in `crates/hc-holiday/tests/countries.rs`
compares, for each year from 2021 to 2026, the weekdays that are not
business days and the weekend days that are with a list assembled by
hand from the statutory days, article 111(3) and the notices, each line
commented with the notice it comes from; all six years agree, and each is
complete. `vietnam_holidays` pins the fixed days, the second National Day
day from 2021 and Vietnamese Culture Day from 2026.

Only the notice of 2025 was re-read for this document, in the government
portal's report of it; the others are as the table's comments give them,
read on 23 September 2026 mostly as the government portals and law
databases reproduce them. The portal numbers the 2025 notice
6150/TB-BLĐTBXH; the earlier notices are carried as the table's sources
number them.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [vn-labour-code-2019] | Articles 111(3), 112(1) and (3), 220(1) | Yes, 2026-09-26, a web.archive.org copy of thuvienphapluat.vn (the official PDF on vanban.chinhphu.vn is a scanned image) |
| [vn-resolution-28-2026] | Article 2: Vietnamese Culture Day, in force 1 July 2026 | Yes, 2026-09-26, the text on xaydungchinhsach.chinhphu.vn |
| [vn-notice-6150-2024] | The worked example: Tết 2025, the swap of 2 May, National Day | Yes, 2026-09-26, the government portal's report |
| [vn-holiday-notices] | The notices of 2021 to 2026 as a series | As the table cites them, read 2026-09-23; not re-read |

## Code

`crates/hc-holiday/src/countries/asia.rs`: `VnFestival`, `VN_DAYS_OFF` and
`VN_WORKDAYS` with `vn_days_off`, `vn_workdays` and the `vn_noticed!`
lookups, `VN_NOTICES_FIRST` and `VN_NOTICES_LAST`, `VN_RULES`,
`VN_SUBSTITUTION` and the table `VIETNAM`. The engine's part is
`Rule::Tabulated`, `Kind::Workday` and `HolidayRule::workday`.

Anchors, in `crates/hc-holiday/tests/countries.rs`: `vietnam_holidays` and
`vietnam_keeps_each_years_notices`.
