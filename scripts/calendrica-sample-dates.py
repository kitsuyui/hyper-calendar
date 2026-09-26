#!/usr/bin/env python3
"""Regenerate crates/hyper-calendar/tests/data/calendrica_sample_dates.txt.

Reingold and Dershowitz's calendar-code2 publishes the Common Lisp code of
*Calendrical Calculations* (4th ed.) in calendar.l, and in dates.l the 33
sample dates of the book's Appendix C with the program that writes the
appendix's tables. dates.l does not contain the tables themselves. This
script is a small interpreter for the subset of Common Lisp those two files
use, so that the tables can be produced without a Lisp system: it loads
calendar.l, evaluates dates.l's list of dates and its helper functions, and
writes every date through every calendar of dates.l's seven lists, checking
each conversion's inverse as dates.l's own `convert` does.

    python3 scripts/calendrica-sample-dates.py <calendar-code2 checkout> \
        crates/hyper-calendar/tests/data/calendrica_sample_dates.txt

The output file's comment header is kept and the data after its
"# ---- data" line replaced. The interpreter is this repository's own code;
nothing of calendar-code2 is copied into it. What it supports: a Lisp-2
with case-folded symbols, exact integers and ratios, double floats for
every float literal (a long float is a double in the common
implementations), and calendar.l's seven macros as special forms with the
expansions that file gives them.
"""
import math
import re
import sys
from fractions import Fraction

sys.setrecursionlimit(100000)


class Sym(str):
    pass


_syms = {}


def sym(name):
    name = name.lower()
    s = _syms.get(name)
    if s is None:
        s = Sym(name)
        _syms[name] = s
    return s


NIL = ()  # the empty list, and false
T = True


def truthy(v):
    return not (v is NIL or v is False or v is None or (isinstance(v, (list, tuple)) and len(v) == 0))


def lbool(v):
    return T if v else NIL


# ---------------------------------------------------------------- reader
TOKEN = re.compile(r"""\s+|;[^\n]*|"(?:[^"\\]|\\.)*"|\#'|\#\||[()'`]|,@|,|[^\s()'`,";]+""")
INT = re.compile(r"^[+-]?\d+\.?$")
RATIO = re.compile(r"^([+-]?\d+)/(\d+)$")
FLOAT = re.compile(r"^([+-]?(?:\d+\.\d*|\.\d+|\d+))(?:[eEdDlLfFsS]([+-]?\d+))?$")


def tokenize(src):
    out = []
    pos = 0
    while pos < len(src):
        if src.startswith("#|", pos):
            end = src.index("|#", pos)
            pos = end + 2
            continue
        m = TOKEN.match(src, pos)
        if not m:
            raise SyntaxError(src[pos:pos + 40])
        tok = m.group(0)
        pos = m.end()
        if tok.isspace() or tok.startswith(";"):
            continue
        out.append(tok)
    return out


def atom(tok):
    if tok.startswith('"'):
        return tok[1:-1].replace('\\"', '"').replace("\\\\", "\\")
    if INT.match(tok):
        return int(tok.rstrip("."))
    m = RATIO.match(tok)
    if m:
        return norm(Fraction(int(m.group(1)), int(m.group(2))))
    m = FLOAT.match(tok)
    if m and ("." in tok or m.group(2) is not None):
        mant, exp = m.group(1), m.group(2)
        return float(mant + ("e" + exp if exp else ""))
    return sym(tok)


def read_all(src):
    toks = tokenize(src)
    pos = 0

    def read():
        nonlocal pos
        tok = toks[pos]
        pos += 1
        if tok == "(":
            lst = []
            while toks[pos] != ")":
                if toks[pos] == ".":
                    raise SyntaxError("dotted pair")
                lst.append(read())
            pos += 1
            return lst
        if tok == ")":
            raise SyntaxError("unexpected )")
        if tok == "'":
            return [sym("quote"), read()]
        if tok == "#'":
            return [sym("function"), read()]
        if tok == "`":
            return [sym("quasiquote"), read()]
        if tok == ",":
            return [sym("unquote"), read()]
        if tok == ",@":
            return [sym("unquote-splicing"), read()]
        return atom(tok)

    forms = []
    while pos < len(toks):
        forms.append(read())
    return forms


# ---------------------------------------------------------------- numbers
def norm(x):
    if isinstance(x, Fraction) and x.denominator == 1:
        return int(x.numerator)
    return x


def num(x):
    if isinstance(x, bool):
        raise TypeError("boolean in arithmetic")
    return x


def add(*a):
    s = 0
    for x in a:
        s = s + num(x)
    return norm(s)


def sub(x, *a):
    if not a:
        return norm(-x)
    for y in a:
        x = x - num(y)
    return norm(x)


def mul(*a):
    p = 1
    for x in a:
        p = p * num(x)
    return norm(p)


def div(x, *a):
    if not a:
        a, x = (x,), 1
    for y in a:
        if isinstance(x, float) or isinstance(y, float):
            x = x / y
        else:
            x = Fraction(x) / Fraction(y)
    return norm(x)


def cl_floor(x, y=1):
    if isinstance(x, float) or isinstance(y, float):
        return math.floor(x / y)
    return math.floor(Fraction(x) / Fraction(y))


def cl_ceiling(x, y=1):
    if isinstance(x, float) or isinstance(y, float):
        return math.ceil(x / y)
    return math.ceil(Fraction(x) / Fraction(y))


def cl_round(x, y=1):
    if isinstance(x, float) or isinstance(y, float):
        q = x / y
        return int(round(q))  # Python rounds half to even, as CL does
    return int(round(Fraction(x) / Fraction(y)))


def cl_truncate(x, y=1):
    q = x / y if (isinstance(x, float) or isinstance(y, float)) else Fraction(x) / Fraction(y)
    return int(q)


def cl_mod(x, y):
    if isinstance(x, float) or isinstance(y, float):
        return x - y * math.floor(x / y)
    return norm(Fraction(x) - Fraction(y) * math.floor(Fraction(x) / Fraction(y)))


def cl_expt(b, e):
    if isinstance(e, int) and not isinstance(b, float):
        if e >= 0:
            return b ** e
        return norm(Fraction(1, 1) / Fraction(b) ** (-e))
    return float(b) ** float(e)


def cmp_chain(op):
    def f(*a):
        return lbool(all(op(a[i], a[i + 1]) for i in range(len(a) - 1)))
    return f


def cl_atan(y, x=None):
    if x is None:
        return math.atan(y)
    return math.atan2(y, x)


def cl_equal(a, b):
    if isinstance(a, (list, tuple)) and isinstance(b, (list, tuple)):
        return len(a) == len(b) and all(cl_equal(x, y) for x, y in zip(a, b))
    if (a is True) != (b is True):
        return False
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        return (not truthy(a)) and (not truthy(b))
    if type(a) is str or type(b) is str:
        return type(a) is type(b) and a == b
    if isinstance(a, float) != isinstance(b, float):
        return False  # CL equal uses eql for numbers: 1 and 1.0 differ
    return a == b


def lst(x):
    if x is NIL or x is False or x is None:
        return []
    return list(x)


def nth(n, l):
    l = lst(l)
    return l[n] if n < len(l) else NIL


BUILTINS = {
    "+": add, "-": sub, "*": mul, "/": div,
    "1+": lambda x: add(x, 1), "1-": lambda x: sub(x, 1),
    "=": cmp_chain(lambda a, b: a == b), "/=": lambda a, b: lbool(a != b),
    "<": cmp_chain(lambda a, b: a < b), "<=": cmp_chain(lambda a, b: a <= b),
    ">": cmp_chain(lambda a, b: a > b), ">=": cmp_chain(lambda a, b: a >= b),
    "floor": cl_floor, "ceiling": cl_ceiling, "round": cl_round, "truncate": cl_truncate,
    "mod": cl_mod, "rem": lambda x, y: norm(x - y * cl_truncate(x, y)),
    "abs": abs, "max": lambda *a: max(a), "min": lambda *a: min(a),
    "expt": cl_expt, "sqrt": lambda x: math.sqrt(x),
    "sin": math.sin, "cos": math.cos, "tan": math.tan,
    "asin": math.asin, "acos": math.acos, "atan": cl_atan, "exp": math.exp,
    "signum": lambda x: (x > 0) - (x < 0),
    "evenp": lambda x: lbool(x % 2 == 0), "oddp": lambda x: lbool(x % 2 == 1),
    "zerop": lambda x: lbool(x == 0),
    "integerp": lambda x: lbool(isinstance(x, int) and not isinstance(x, bool)),
    "numberp": lambda x: lbool(isinstance(x, (int, float, Fraction)) and not isinstance(x, bool)),
    "not": lambda x: lbool(not truthy(x)), "null": lambda x: lbool(not truthy(x)),
    "list": lambda *a: list(a),
    "first": lambda l: nth(0, l), "second": lambda l: nth(1, l), "third": lambda l: nth(2, l),
    "fourth": lambda l: nth(3, l), "fifth": lambda l: nth(4, l), "sixth": lambda l: nth(5, l),
    "seventh": lambda l: nth(6, l), "eighth": lambda l: nth(7, l), "ninth": lambda l: nth(8, l),
    "tenth": lambda l: nth(9, l), "car": lambda l: nth(0, l), "cadr": lambda l: nth(1, l),
    "nth": nth, "rest": lambda l: lst(l)[1:], "cdr": lambda l: lst(l)[1:],
    "butlast": lambda l: lst(l)[:-1], "last": lambda l: lst(l)[-1:],
    "length": lambda l: len(lst(l)) if not isinstance(l, str) else len(l),
    "append": lambda *ls: [x for l in ls for x in lst(l)],
    "reverse": lambda l: lst(l)[::-1],
    "member": lambda x, l: (lambda L: next((L[i:] for i in range(len(L)) if cl_equal_eql(x, L[i])), NIL))(lst(l)),
    "equal": lambda a, b: lbool(cl_equal(a, b)),
    "eql": lambda a, b: lbool(cl_equal_eql(a, b)),
    "cons": lambda a, l: [a] + lst(l),
}


def cl_equal_eql(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        return a is b or (not truthy(a) and not truthy(b))
    return cl_equal(a, b)


# ---------------------------------------------------------------- evaluator
class Closure:
    def __init__(self, params, body, env, name="lambda"):
        self.required = []
        self.optional = []
        mode = "req"
        for p in params:
            if p == "&optional":
                mode = "opt"
                continue
            (self.required if mode == "req" else self.optional).append(p)
        self.body = body
        self.env = env
        self.name = name

    def __call__(self, *args):
        if len(args) < len(self.required) or len(args) > len(self.required) + len(self.optional):
            raise TypeError(f"{self.name}: wrong number of args {len(args)}")
        frame = dict(zip(self.required, args))
        for i, p in enumerate(self.optional):
            j = len(self.required) + i
            frame[p] = args[j] if j < len(args) else NIL
        env = Env(frame, self.env)
        r = NIL
        for form in self.body:
            r = evaluate(form, env)
        return r


class Env:
    __slots__ = ("vars", "parent")

    def __init__(self, vars, parent):
        self.vars = vars
        self.parent = parent

    def lookup(self, s):
        e = self
        while e is not None:
            if s in e.vars:
                return e.vars[s]
            e = e.parent
        return global_var(s)


FUNCS = {sym(k): v for k, v in BUILTINS.items()}
CONSTS = {}  # symbol -> form (lazy) or value
_CONST_VALUES = {}


def global_var(s):
    if s in _CONST_VALUES:
        return _CONST_VALUES[s]
    if s == "t":
        return T
    if s == "nil":
        return NIL
    if s == "pi":
        return math.pi
    if s in CONSTS:
        v = evaluate(CONSTS[s], None)
        _CONST_VALUES[s] = v
        return v
    raise NameError(f"unbound variable {s}")


def lookup_var(s, env):
    if env is None:
        return global_var(s)
    return env.lookup(s)


def function_of(f, env):
    if isinstance(f, Sym):
        if f in FUNCS:
            return FUNCS[f]
        raise NameError(f"undefined function {f}")
    if isinstance(f, list) and f and f[0] == "lambda":
        return Closure(f[1], f[2:], env)
    if callable(f):
        return f
    raise TypeError(f"not a function {f!r}")


S = sym


def evaluate(x, env):
    if isinstance(x, Sym):
        return lookup_var(x, env)
    if not isinstance(x, list):
        return x
    if not x:
        return NIL
    head = x[0]
    if isinstance(head, Sym):
        h = head
        if h == "quote":
            return x[1]
        if h == "if":
            if truthy(evaluate(x[1], env)):
                return evaluate(x[2], env)
            return evaluate(x[3], env) if len(x) > 3 else NIL
        if h == "cond":
            for clause in x[1:]:
                v = evaluate(clause[0], env)
                if truthy(v):
                    r = v
                    for f in clause[1:]:
                        r = evaluate(f, env)
                    return r
            return NIL
        if h == "and":
            r = T
            for f in x[1:]:
                r = evaluate(f, env)
                if not truthy(r):
                    return NIL
            return r
        if h == "or":
            for f in x[1:]:
                r = evaluate(f, env)
                if truthy(r):
                    return r
            return NIL
        if h == "let*" or h == "let":
            frame = {}
            new = Env(frame, env)
            for b in x[1]:
                if isinstance(b, Sym):
                    frame[b] = NIL
                else:
                    val = evaluate(b[1], new if h == "let*" else env)
                    if h == "let*":
                        # each binding is a new scope in let*; a dict suffices
                        # because names are not rebound within one let*
                        pass
                    frame[b[0]] = val
            r = NIL
            for f in x[2:]:
                r = evaluate(f, new)
            return r
        if h == "progn":
            r = NIL
            for f in x[1:]:
                r = evaluate(f, env)
            return r
        if h == "function":
            return function_of(x[1], env)
        if h == "lambda":
            return Closure(x[1], x[2:], env)
        if h == "defun":
            FUNCS[x[1]] = Closure(x[2], x[3:], None, name=x[1])
            return x[1]
        if h == "defconstant":
            CONSTS[x[1]] = x[2]
            _CONST_VALUES.pop(x[1], None)
            return x[1]
        if h in ("defmacro", "in-package", "export", "use-package"):
            return NIL
        # --- the macros of calendar.l
        if h == "next":  # (next index initial condition)
            var, init, cond = x[1], x[2], x[3]
            i = evaluate(init, env)
            frame = {}
            e = Env(frame, env)
            while True:
                frame[var] = i
                if truthy(evaluate(cond, e)):
                    return i
                i += 1
        if h == "final":
            var, init, cond = x[1], x[2], x[3]
            i = evaluate(init, env)
            frame = {}
            e = Env(frame, env)
            while True:
                frame[var] = i
                if not truthy(evaluate(cond, e)):
                    return i - 1
                i += 1
        if h == "sum" or h == "prod":
            expr, var, init, cond = x[1], x[2], x[3], x[4]
            i = evaluate(init, env)
            frame = {}
            e = Env(frame, env)
            acc = 0 if h == "sum" else 1
            while True:
                frame[var] = i
                if not truthy(evaluate(cond, e)):
                    return acc
                v = evaluate(expr, e)
                acc = add(acc, v) if h == "sum" else mul(acc, v)
                i += 1
        if h == "binary-search":  # (binary-search l lo h hi x test end)
            lv, lo, hv, hi, xv, test, end = x[1:8]
            frame = {}
            e = Env(frame, env)
            frame[xv] = NIL
            frame["#left"] = NIL
            frame[lv] = evaluate(lo, e)
            frame[hv] = evaluate(hi, e)
            while True:
                if truthy(evaluate(end, e)):
                    return div(add(frame[hv], frame[lv]), 2)
                frame[xv] = div(add(frame[hv], frame[lv]), 2)
                left = truthy(evaluate(test, e))
                if not left:
                    frame[lv] = frame[xv]
                else:
                    frame[hv] = frame[xv]
        if h == "invert-angular":  # (invert-angular f y r)
            f, y, r = x[1], x[2], x[3]
            form = [S("binary-search"), S("#l"), [S("begin"), r], S("#u"), [S("end"), r], S("#x"),
                    [S("<"), [S("mod"), [S("-"), [f, S("#x")], y], 360], [S("deg"), 180]],
                    [S("<"), [S("-"), S("#u"), S("#l")], Fraction(1, 100000)]]
            return evaluate(form, env)
        if h == "sigma":  # (sigma ((i1 l1) ...) body)
            pairs, body = x[1], x[2]
            names = [p[0] for p in pairs]
            lists = [lst(evaluate(p[1], env)) for p in pairs]
            total = 0
            n = min(len(l) for l in lists)
            for k in range(n):
                e = Env({nm: l[k] for nm, l in zip(names, lists)}, env)
                total = add(total, evaluate(body, e))
            return total
        if h == "apply":
            f = evaluate(x[1], env)
            args = [evaluate(a, env) for a in x[2:]]
            f = function_of(f, env) if not callable(f) else f
            spread = args[:-1] + lst(args[-1])
            return f(*spread)
        if h == "mapcar":
            f = evaluate(x[1], env)
            f = function_of(f, env) if not callable(f) else f
            lists = [lst(evaluate(a, env)) for a in x[2:]]
            n = min(len(l) for l in lists)
            return [f(*[l[k] for l in lists]) for k in range(n)]
        if h == "funcall":
            f = evaluate(x[1], env)
            f = function_of(f, env) if not callable(f) else f
            return f(*[evaluate(a, env) for a in x[2:]])
        f = FUNCS.get(h)
        if f is None:
            raise NameError(f"undefined function {h}")
        return f(*[evaluate(a, env) for a in x[1:]])
    if isinstance(head, list) and head and head[0] == "lambda":
        f = Closure(head[1], head[2:], env)
        return f(*[evaluate(a, env) for a in x[1:]])
    raise TypeError(f"bad form {x!r}")


def load(path):
    with open(path, encoding="utf-8") as fh:
        for form in read_all(fh.read()):
            evaluate(form, None)


def call(name, *args):
    return FUNCS[sym(name)](*args)


# ---------------------------------------------------------------- the tables
# Field orders are those of calendar.l's constructors; "t" and "f" are its
# booleans, and moments are fixed dates with a fraction of a day.
FIELDS = {
    "day": "day of week", "jd": "Julian date of the day's midnight",
    "mjd": "modified Julian day", "unix": "Unix seconds at the day's midnight",
    "gregorian": "year month day", "julian": "year month day, no year 0 (-1 is 1 BCE)",
    "roman": "year month event(1 kalends, 2 nones, 3 ides) count leap",
    "olympiad": "cycle year", "egyptian": "year month day", "armenian": "year month day",
    "akan-name": "prefix stem", "coptic": "year month day", "ethiopic": "year month day",
    "iso": "year week day", "icelandic": "year season(90 summer, 270 winter) week weekday(0 Sunday)",
    "islamic": "year month day", "observational-islamic": "year month day, Cairo",
    "saudi-islamic": "year month day, Mecca", "hebrew": "year month(1 Nisan, 7 Tishri, 12 Adar or Adar I, 13 Adar II) day",
    "observational-hebrew": "year month day, as hebrew, Haifa", "persian": "year month day",
    "arithmetic-persian": "year month day", "bahai": "major cycle year month(19 Ala, 0 Ayyam-i-Ha) day",
    "astro-bahai": "major cycle year month day, as bahai", "french": "year month day",
    "arithmetic-french": "year month day", "orthodox-easter": "Gregorian year month day",
    "easter": "Gregorian year month day", "astronomical-easter": "Gregorian year month day",
    "mayan-long-count": "baktun katun tun uinal kin", "mayan-haab": "month day(0-19)",
    "mayan-tzolkin": "number name", "aztec-xihuitl": "month day", "aztec-tonalpohualli": "number name",
    "bali-pawukon": "luang dwiwara triwara caturwara pancawara sadwara saptawara asatawara sangawara dasawara(0-9)",
    "babylonian": "year(Seleucid) month leap day", "samaritan": "year month(1 the month of Passover) day",
    "chinese": "cycle year month leap day", "chinese-day-name": "stem branch",
    "major-solar-term-on-or-after": "moment (UT)", "old-hindu-solar": "year month day",
    "hindu-solar": "year(Saka) month day", "astro-hindu-solar": "year(Saka) month day",
    "old-hindu-lunar": "year month leap day", "hindu-lunar": "year(Vikrama) month leap-month day leap-day",
    "astro-hindu-lunar": "year(Vikrama) month leap-month day leap-day",
    "tibetan": "year month leap-month day leap-day", "ephem-corr": "days (TT - UT)",
    "eqn-of-time": "days", "solar-long": "degrees at noon UT", "solstice": "moment (UT)",
    "dawn": "day-fraction hh mm ss, standard time at Paris, 18 degrees",
    "mid-day": "day-fraction hh mm ss, standard time at Tehran",
    "set": "day-fraction hh mm ss, standard time of sunset at Jerusalem",
    "lunar-long": "degrees at midnight UT", "lunar-lat": "degrees at midnight UT",
    "lunar-alt": "degrees at midnight UT at Mecca", "new-moon-after": "moment (UT)",
    "moonrise": "day-fraction hh mm ss, standard time at Mecca",
    "moonset": "day-fraction hh mm ss, standard time at Mecca",
}


def token(v):
    if v is True:
        return "t"
    if isinstance(v, (list, tuple)) and len(v) == 0:
        return "f"
    if isinstance(v, Fraction):
        return repr(float(v))
    if isinstance(v, float):
        return repr(v)
    if isinstance(v, (list, tuple)):
        return " ".join(token(x) for x in v)
    return str(v)


def main():
    code, out = sys.argv[1], sys.argv[2]
    load(code + "/calendar.l")
    skip = {"cat", "format-time", "convert", "texify", "compute-dates", "jd-from-fixed"}
    with open(code + "/dates.l", encoding="utf-8") as fh:
        forms = read_all(fh.read())
    lists, no_inverse = {}, set()
    for f in forms:
        if not isinstance(f, list) or not f:
            continue
        head = f[0]
        if head == "defun" and f[1] in skip:
            continue
        if head in ("or", "compute-dates", "in-package", "use-package"):
            continue
        if head == "defconstant" and str(f[1]).startswith("calendars-"):
            lists[str(f[1])] = [str(s) for s in f[2][1]]
            continue
        if head == "defconstant" and f[1] == "do-not-invert-list":
            no_inverse = {str(s) for s in f[2][1]}
            continue
        evaluate(f, None)

    def format_time(p):
        # dates.l's format-time, without the TeX: add half a second, then
        # truncate the seconds.
        if p == "bogus":
            return "bogus"
        clock = call("clock-from-moment", add(Fraction(1, 2 * 24 * 60 * 60), p))
        return [float(cl_mod(p, 1)), int(clock[0]), int(clock[1]), int(math.floor(clock[2]))]

    FUNCS[sym("format-time")] = format_time
    # dates.l coerces the Julian date to a single float for printing; the
    # values are exact halves either way.
    FUNCS[sym("jd-from-fixed")] = lambda d: call("jd-from-moment", d)
    dates = global_var(sym("dates"))
    lines = []
    for key in sorted(lists):
        for cal in lists[key]:
            lines.append(f"# {cal} ({key}): {FIELDS[cal]}")
            for d in dates:
                v = call(cal + "-from-fixed", d)
                if cal not in no_inverse and call("fixed-from-" + cal, v) != d:
                    raise SystemExit(f"{cal} does not round-trip at {d}")
                lines.append(f"{cal}\t{d}\t{token(v)}")
    with open(out, "r+", encoding="utf-8") as fh:
        text = fh.read()
        marker = "# ---- data\n"
        head = text[: text.index(marker) + len(marker)]
        fh.seek(0)
        fh.write(head + "\n".join(lines) + "\n")
        fh.truncate()


if __name__ == "__main__":
    main()
