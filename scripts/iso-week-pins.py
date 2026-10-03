#!/usr/bin/env python3
"""Pins for the ISO week date at both ends of the Gregorian range.

The algorithm is the one of Wikipedia, "ISO week date"
(<https://en.wikipedia.org/wiki/ISO_week_date>, `wikipedia-iso-week-date`, read
2026-10-03): the week number is the floor of (10 + day of year - weekday) / 7,
corrected to the year before or after for a week 0 or one past the last, and a
year has 53 weeks when p(y) = 4 or p(y - 1) = 3 for
p(y) = (y + y/4 - y/100 + y/400) mod 7. It is checked here against Python's
`datetime.date.isocalendar` for 1900 to 2099, then applied to the years
+-9 999 999, which `datetime` cannot reach. The pins are those of
`crates/hc-calendars-solar/src/iso_week.rs` and
`crates/hyper-calendar/tests/python.rs`. It uses no code of this repository.

    python3 scripts/iso-week-pins.py
"""
def p(y): return (y + y//4 - y//100 + y//400) % 7      # weekday of 31 Dec of y, 0 = Sunday (4 = Thursday)
def weeks(y): return 53 if p(y)==4 or p(y-1)==3 else 52
def leap(y): return y%4==0 and (y%100!=0 or y%400==0)
MD=[31,28,31,30,31,30,31,31,30,31,30,31]
def doy(y,m,d): return sum(MD[:m-1])+(1 if m>2 and leap(y) else 0)+d
def dow(y,m,d):  # 1=Monday..7=Sunday
    ylen=366 if leap(y) else 365
    dec31=p(y) or 7
    back=ylen-doy(y,m,d)          # days from the date to 31 Dec
    return (dec31-1-back)%7+1
def iso(y,m,d):
    w=(10+doy(y,m,d)-dow(y,m,d))//7
    if w<1: return (y-1,weeks(y-1),dow(y,m,d))
    if w>weeks(y): return (y+1,1,dow(y,m,d))
    return (y,w,dow(y,m,d))
# sanity against python for a modern range
import datetime
for y in range(1900,2100):
    for m in (1,12):
        for d in (1,2,3,4,5,28,29,30,31):
            try: dt=datetime.date(y,m,d)
            except: continue
            assert iso(y,m,d)==tuple(dt.isocalendar()),(y,m,d)
if __name__=="__main__":
    for y,m,d in [(9999999,1,1),(9999999,1,3),(9999999,1,4),(9999999,1,5),(9999999,12,26),(9999999,12,27),(9999999,12,28),(9999999,12,29),(9999999,12,30),(9999999,12,31),(-9999999,1,1),(-9999999,1,2),(-9999999,1,3),(-9999999,1,4),(-9999999,1,5),(-9999999,12,31)]:
        print((y,m,d),"dow",dow(y,m,d),"iso",iso(y,m,d), "weeks(y)",weeks(y),"weeks(MAX+1)" if y>0 else "")
    print("weeks 9999999",weeks(9999999),"10000000",weeks(10000000),"-10000000",weeks(-10000000),"-9999999",weeks(-9999999))
