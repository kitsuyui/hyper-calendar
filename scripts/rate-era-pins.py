#!/usr/bin/env python3
"""Pins for the 1961-1971 rate era of TAI - UTC, in exact rational arithmetic.

The rows are those of the USNO `tai-utc.dat`
(<https://maia.usno.navy.mil/ser7/tai-utc.dat>, `usno-tai-utc`), which the IERS
`UTC-TAI.history` (<https://hpiers.obspm.fr/iers/bul/bulc/UTC-TAI.history>,
`iers-utc-tai-history`) repeats; both were read 2026-10-03. The script
prints TAI - UTC in attoseconds at the first second of each segment and at the
last second before it (the `OFFSETS` pins of `crates/hc-core/src/unix.rs`) and
the UTC reading of TAI 63 072 008 (pinned in
`utc_from_tai_keeps_the_fraction_of_the_offset`). It uses no code of this
repository.

    python3 scripts/rate-era-pins.py
"""
from fractions import Fraction as F

usno=[("1961-01-01",37300,"1.4228180",37300,"0.001296"),("1961-08-01",37512,"1.3728180",37300,"0.001296"),("1962-01-01",37665,"1.8458580",37665,"0.0011232"),("1963-11-01",38334,"1.9458580",37665,"0.0011232"),("1964-01-01",38395,"3.2401300",38761,"0.001296"),("1964-04-01",38486,"3.3401300",38761,"0.001296"),("1964-09-01",38639,"3.4401300",38761,"0.001296"),("1965-01-01",38761,"3.5401300",38761,"0.001296"),("1965-03-01",38820,"3.6401300",38761,"0.001296"),("1965-07-01",38942,"3.7401300",38761,"0.001296"),("1965-09-01",39004,"3.8401300",38761,"0.001296"),("1966-01-01",39126,"4.3131700",39126,"0.002592"),("1968-02-01",39887,"4.2131700",39126,"0.002592")]
END=41317
def seg(mjd):
    for i in range(len(usno)-1,-1,-1):
        if usno[i][1]<=mjd: return usno[i]
def off(u):  # u: Fraction seconds unix
    mjd=F(40587)+u/86400
    s=seg(int(mjd//1))
    return F(s[2])+(mjd-s[3])*F(s[4])
def attos(x): return x*10**18
if __name__=="__main__":
    for lab,m,*_ in usno+[("1972-01-01",END,0,0,0)]:
        u=(m-40587)*86400
        if lab!="1961-01-01":
            e=off(F(u-1)) ; print(lab,"last second before:",u-1,float(e), attos(e))
        if lab!="1972-01-01":
            e=off(F(u)); print(lab,"start:",u,float(e),attos(e))
    # 63072008 TAI
    tai=F(63072008)
    # solve in last seg
    s=seg(39887)
    # u = tai - c - (mjd-o)*d ; mjd=40587+u/86400
    c=F(s[2]);o=F(s[3]);d=F(s[4])
    # u + c + (40587+u/86400 - o)*d = tai
    u=(tai-c-(40587-o)*d)/(1+d/86400)
    print("utc of tai 63072008:",float(u), u*10**18)
