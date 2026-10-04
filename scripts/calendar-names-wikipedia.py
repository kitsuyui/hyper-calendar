#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/data/wikipedia_calendar_names.rs from the
interlanguage links of Wikipedia's articles on the registry's calendars.

    python3 scripts/calendar-names-wikipedia.py            # rewrite the file
    python3 scripts/calendar-names-wikipedia.py --check    # exit 1 if it is stale
    python3 scripts/calendar-names-wikipedia.py --dump     # every name, a TSV, and the log

CLDR names the calendars of its own `-u-ca-` keys, and `src/data.rs` and
`src/data/cldr48_locales.rs` carry those names; the registry has many more
calendars, which every locale but English was left to name in English. For
each of those, `ARTICLES` below names the Wikipedia article about the
calendar — the English one, or where English has none the Japanese or
Chinese one — and this script reads the article's interlanguage links
(the MediaWiki API, `action=query&prop=langlinks`, JSON over HTTP into
memory; nothing is saved) and carries, for every carried locale whose
Wikipedia edition the links reach, the title of that edition's article on
the calendar, exactly as the edition writes it, with its revision of the
article read. A title is a name the language's own writers gave the
calendar; nothing is translated here, and a locale whose edition has no
article on a calendar gets no name for it, so that a page falls back to
the calendar's English name as before. The Chinese Wikipedia keeps one
article under one title and shows it to Simplified and Traditional readers
through its variant conversion, so `zh-Hans` and `zh-Hant` take the
displayed title of each variant (`action=parse&prop=displaytitle`,
`variant=zh-hans` and `zh-hant`); the Cantonese edition (`zh-yue`) serves
`yue-Hant`.

A name already carried from CLDR or written by hand (`src/calendar_names.rs`)
is left alone: those answer first, and the generated table only fills what
they leave. A title that would give two calendars one name in a locale, or
that is written in a script other than the locale's, is left out and
logged, since a reader choosing from a list has to tell the calendars
apart (`crates/hyper-calendar/tests/distinct_names.rs`). So is a link into
a section of an article, a link onto a redirect, and a link the linked
article does not return — a link kept by hand into a broader article, the
English "Tzolkʼin" into the Chinese article on the Maya calendar as a
whole — which the script checks by reading each linked article's own link
back. A parenthesised disambiguator at the end of a title, "Ab urbe
condita (Chronologie)", tells the article from another of the same name
and is dropped, the rest being the article's name. Every registry
identifier of `docs/supported.md` is either in `ARTICLES` or in `UNNAMED`
with the reason no article names it, so that a new calendar fails
`--check` until it is placed.
"""
import html
import json
import os
import re
import sys
import time
import unicodedata
import urllib.error
import urllib.parse
import urllib.request

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cldr_xml import lit, rustfmt, write_or_check  # noqa: E402

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data/wikipedia_calendar_names.rs')
SUPPORTED = os.path.join(ROOT_DIR, 'docs/supported.md')
CLDR_FILES = [os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data.rs'),
              os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data/cldr48_locales.rs')]
HAND_FILE = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/calendar_names.rs')
READ = '2026-10-05'
USER_AGENT = ('hyper-calendar scripts/calendar-names-wikipedia.py '
              '(https://github.com/kitsuyui/hyper-calendar)')
# Seconds between requests: the API asks clients to go one request at a
# time, and answers a burst with 429.
PACE = 1.0

# Registry identifier -> (Wikipedia edition, article title): the article
# about that calendar, on the English Wikipedia unless it has none, then on
# the edition named. The title must be the article's own, not a redirect:
# "Khmer calendar" redirects to a section of "Month", whose links name the
# month and not the calendar, so a redirect is refused below. Where the
# registry carries one calendar under several conventions (policy §5), the
# article names the calendar itself, and the convention that is the
# calendar's own rule takes it: `hebrew` and not `hebrew-observational`;
# `french-republican-equinox`, the rule of the decree of 1793, and not
# Romme's or Richards's arithmetic; `tibetan`, the Phugpa reckoning the
# official almanacs use.
ARTICLES = {
    'akan': ('en', 'Akan calendar'),
    'armenian': ('en', 'Armenian calendar'),
    'assyrian': ('en', 'Assyrian calendar'),
    'aztec-tonalpohualli': ('en', 'Tōnalpōhualli'),
    'aztec-xiuhpohualli': ('en', 'Xiuhpōhualli'),
    'babylonian': ('en', 'Babylonian calendar'),
    'bahai': ('en', 'Baháʼí calendar'),
    'balinese-pawukon': ('en', 'Pawukon calendar'),
    'berber': ('en', 'Berber calendar'),
    'bikram-sambat': ('en', 'Vikram Samvat'),
    'bostran-era': ('en', 'Bostran era'),
    'buddhist': ('en', 'Buddhist calendar'),
    'burmese': ('en', 'Burmese calendar'),
    'byzantine': ('en', 'Byzantine calendar'),
    'chinese': ('en', 'Chinese calendar'),
    'chinese-daming': ('ja', '大明暦'),
    'chinese-jingchu': ('ja', '景初暦'),
    'chinese-kaihuang': ('zh', '開皇曆'),
    'chinese-qianxiang': ('ja', '乾象暦'),
    'chinese-sanji': ('zh', '三紀曆'),
    'chinese-sifen': ('ja', '四分暦'),
    'chinese-taichu': ('ja', '太初暦'),
    'chinese-tianhe': ('zh', '天和曆'),
    'chinese-xinghe': ('zh', '興和曆'),
    'chinese-yuanjia': ('en', 'Genka calendar'),
    'chinese-zhengguang': ('zh', '正光曆'),
    'coptic': ('en', 'Coptic calendar'),
    'dangi': ('en', 'Korean calendar'),
    'discordian': ('en', 'Discordian calendar'),
    'egyptian': ('en', 'Ancient Egyptian calendar'),
    'era-fascista': ('en', 'Era Fascista'),
    'ethiopic': ('en', 'Ethiopian calendar'),
    'french-republican-equinox': ('en', 'French Republican calendar'),
    'gregory': ('en', 'Gregorian calendar'),
    'gupta': ('en', 'Gupta era'),
    'hanke-henry': ('en', 'Hanke–Henry Permanent Calendar'),
    'hebrew': ('en', 'Hebrew calendar'),
    'hindu-lunar': ('en', 'Hindu calendar'),
    'hindu-solar-bengali': ('en', 'Bengali calendar'),
    'hindu-solar-malayalam': ('en', 'Malayalam calendar'),
    'hindu-solar-tamil': ('en', 'Tamil calendar'),
    'holocene': ('en', 'Holocene calendar'),
    'hongxian': ('ja', '洪憲'),
    'huangdi-era': ('ja', '黄帝紀元'),
    'indian': ('en', 'Indian national calendar'),
    'international-fixed': ('en', 'International Fixed Calendar'),
    'islamic-civil': ('en', 'Tabular Islamic calendar'),
    'iso8601': ('en', 'ISO 8601'),
    'iso8601-ordinal': ('en', 'Ordinal date'),
    'iso8601-week': ('en', 'ISO week date'),
    'jalali': ('en', 'Jalali calendar'),
    'japanese': ('en', 'Japanese calendar'),
    'japanese-horyaku': ('en', 'Hōryaku calendar'),
    'japanese-imperial': ('en', 'Japanese imperial year'),
    'japanese-jokyo': ('en', 'Jōkyō calendar'),
    'japanese-kansei': ('en', 'Kansei calendar'),
    'japanese-senmyo': ('en', 'Xuanming calendar'),
    'japanese-tenpo': ('en', 'Tenpō calendar'),
    'javanese': ('en', 'Javanese calendar'),
    'juche': ('en', 'Juche calendar'),
    'julian': ('en', 'Julian calendar'),
    'julian-day': ('en', 'Julian day'),
    'kalachuri': ('en', 'Kalachuri Era'),
    'kurdish': ('en', 'Kurdish calendar'),
    'lilian': ('en', 'Lilian date'),
    'mandaean': ('en', 'Mandaean calendar'),
    'masonic-anno-lucis': ('en', 'Anno Lucis'),
    'maya-haab': ('en', 'Haabʼ'),
    'maya-longcount': ('en', 'Mesoamerican Long Count calendar'),
    'maya-tzolkin': ('en', 'Tzolkʼin'),
    'mongolian': ('en', 'Mongolian calendar'),
    'nanakshahi': ('en', 'Nanakshahi calendar'),
    'nepal-sambat': ('en', 'Nepal Sambat'),
    'olympiad': ('en', 'Olympiad'),
    'pax': ('en', 'Pax Calendar'),
    'persian': ('en', 'Solar Hijri calendar'),
    'positivist': ('en', 'Positivist calendar'),
    'revised-julian': ('en', 'Revised Julian calendar'),
    'roc': ('en', 'Republic of China calendar'),
    'roman-auc': ('en', 'Ab urbe condita'),
    'rumi': ('en', 'Rumi calendar'),
    'sexagenary': ('en', 'Sexagenary cycle'),
    'soviet-week': ('en', 'Soviet calendar'),
    'spanish-era': ('en', 'Spanish era'),
    'swedish-1700': ('en', 'Swedish calendar'),
    'symmetry454': ('en', 'Symmetry454'),
    'thai-lunar': ('en', 'Thai lunar calendar'),
    'tibetan': ('en', 'Tibetan calendar'),
    'vietnamese': ('en', 'Vietnamese calendar'),
    'vira-nirvana-samvat': ('en', 'Vira Nirvana Samvat'),
    'world-calendar': ('en', 'World Calendar'),
}

# Why no article names the other identifiers, by reason; the generated file
# lists them so that the gap is stated, not discovered.
CONVENTION = ('a convention of a calendar whose article names the calendar, not the '
              'convention, and whose own rule carries the title')
NO_ARTICLE = ('no article on it in the English, Japanese or Chinese Wikipedia (searched '
              f'{READ}); a day count, a proposal or a reckoning no edition has written up')
SECTION = ('the English title redirects to a section of an article on another subject, '
           'whose links name that subject')
SHARED = ("one article would serve several identifiers, none of them the calendar's own "
          'rule')
OTHER_SUBJECT = 'the article of that title is about another subject'
UNNAMED = {
    'ada': NO_ARTICLE, 'ansi-date': NO_ARTICLE, 'antioch-caesarean-era': NO_ARTICLE,
    'antioch-caesarean-era-september': NO_ARTICLE, 'archetypes': NO_ARTICLE,
    'armenian-fixed': CONVENTION, 'arsacid-era': NO_ARTICLE, 'asian': NO_ARTICLE,
    'bahai-arithmetic': CONVENTION, 'bahai-astronomical': CONVENTION,
    'bangladeshi': SHARED, 'buddhist-lk': CONVENTION, 'ccsds-day': NO_ARTICLE,
    'cheondogyo-podeok': NO_ARTICLE, 'chinese-regnal': NO_ARTICLE,
    'chinese-regnal-qing-court': NO_ARTICLE, 'chronological-julian-day': NO_ARTICLE,
    'cnes-julian-day': NO_ARTICLE, 'dangi-kasi': CONVENTION, 'dee': NO_ARTICLE,
    'dee-cecil': NO_ARTICLE, 'dublin-julian-day': NO_ARTICLE, 'egyptian-ptolemy': CONVENTION,
    'excel-1900': NO_ARTICLE, 'excel-1904': NO_ARTICLE, 'fasli-bombay': SHARED,
    'fasli-madras': SHARED, 'french-republican-arithmetic': CONVENTION,
    'french-republican-arithmetic-richards': CONVENTION, 'gaza-era': NO_ARTICLE,
    'hebrew-observational': CONVENTION, 'hermetic-leap-week': NO_ARTICLE,
    'hindu-lunar-purnimanta': CONVENTION, 'hindu-lunar-reingold-dershowitz': CONVENTION,
    'hindu-lunar-surya-siddhanta': CONVENTION, 'hindu-old-lunar': NO_ARTICLE,
    'hindu-old-solar': NO_ARTICLE, 'hindu-solar-reingold-dershowitz': NO_ARTICLE,
    'hindu-solar-surya-siddhanta': NO_ARTICLE, 'hindu-solar-vikrami': NO_ARTICLE,
    'huangdi-era-jiangsu': CONVENTION, 'huangdi-era-liu-shipei': CONVENTION,
    'huangdi-era-tongmenghui': CONVENTION, 'icelandic': SECTION, 'icelandic-almanac': SECTION,
    'icelandic-friday': SECTION, 'icelandic-julian': SECTION,
    'icelandic-julian-friday': SECTION, 'icelandic-medieval': SECTION,
    'islamic-fatimid': SHARED, 'islamic-fcna': SHARED, 'islamic-istanbul-2016': SHARED,
    'islamic-khgt': SHARED, 'islamic-observational-cairo-rd': SHARED, 'islamic-rgsa': SHARED,
    'islamic-saudi-rule-rd': SHARED, 'islamic-tbla': CONVENTION, 'islamic-umalqura': SECTION,
    'jalali-natanz': CONVENTION, 'jalali-tusi': CONVENTION, 'japanese-kaigen-toji': CONVENTION,
    'japanese-northern': CONVENTION, 'japanese-northern-proclaimed': CONVENTION,
    'japanese-proclaimed': CONVENTION, 'japanese-southern': CONVENTION,
    'japanese-southern-proclaimed': CONVENTION, 'javanese-aboge': CONVENTION,
    'javanese-pasaran': OTHER_SUBJECT, 'javanese-yogyakarta': CONVENTION,
    'julian-gregorian-bg': NO_ARTICLE, 'julian-gregorian-catholic': NO_ARTICLE,
    'julian-gregorian-de-catholic': NO_ARTICLE, 'julian-gregorian-de-protestant': NO_ARTICLE,
    'julian-gregorian-fr': NO_ARTICLE, 'julian-gregorian-gb': NO_ARTICLE,
    'julian-gregorian-gr': NO_ARTICLE, 'julian-gregorian-hu': NO_ARTICLE,
    'julian-gregorian-nl-holland': NO_ARTICLE, 'julian-gregorian-nl-states-general': NO_ARTICLE,
    'julian-gregorian-ro': NO_ARTICLE, 'julian-gregorian-rs': NO_ARTICLE,
    'julian-gregorian-ru': NO_ARTICLE, 'julian-gregorian-se': NO_ARTICLE, 'khmer': SECTION,
    'korean-regnal': NO_ARTICLE, 'korean-regnal-backdated': NO_ARTICLE,
    'lakshmana-sena': NO_ARTICLE, 'lao': NO_ARTICLE, 'liberalia-triday-lunar': NO_ARTICLE,
    'liberalia-triday-solar': NO_ARTICLE, 'magi-san': NO_ARTICLE, 'manchukuo': NO_ARTICLE,
    'masonic-anno-depositionis': NO_ARTICLE, 'masonic-anno-inventionis': NO_ARTICLE,
    'masonic-anno-lucis-march': CONVENTION, 'masonic-anno-ordinis': NO_ARTICLE,
    'matlab-datenum': NO_ARTICLE, 'maya-819': NO_ARTICLE, 'maya-819-584286': NO_ARTICLE,
    'maya-819-gmt2': NO_ARTICLE, 'maya-haab-584286': CONVENTION, 'maya-haab-gmt2': CONVENTION,
    'maya-longcount-584286': CONVENTION, 'maya-longcount-gmt2': CONVENTION,
    'maya-round': SECTION, 'maya-round-584286': SECTION, 'maya-round-gmt2': SECTION,
    'maya-tzolkin-584286': CONVENTION, 'maya-tzolkin-gmt2': CONVENTION,
    'meyer-palmen': NO_ARTICLE, 'mixtec-year': NO_ARTICLE, 'modified-julian-day': SECTION,
    'modified-julian-day-2000': NO_ARTICLE, 'nepal-sambat-fortnight': CONVENTION,
    'odia-anka': OTHER_SUBJECT, 'ole-automation-date': NO_ARTICLE, 'persian-afghan': CONVENTION,
    'persian-apparent-noon': CONVENTION, 'persian-arithmetic': CONVENTION,
    'persian-arithmetic-33': CONVENTION, 'persian-imperial': NO_ARTICLE,
    'philip-era': NO_ARTICLE, 'philip-era-ptolemy': NO_ARTICLE, 'qumran': NO_ARTICLE,
    'rajyabhisheka-saka': NO_ARTICLE, 'reduced-julian-day': SECTION,
    'roman-auc-capitoline': CONVENTION, 'samaritan': SECTION, 'saptarshi': NO_ARTICLE,
    'sas-date': NO_ARTICLE, 'seleucid-syrian': SHARED, 'stata-date': NO_ARTICLE,
    'stata-week': NO_ARTICLE, 'sur-san': NO_ARTICLE, 'symmetry010': NO_ARTICLE,
    'tabot': OTHER_SUBJECT, 'taiping-tianli': NO_ARTICLE, 'tibetan-bhutan': CONVENTION,
    'tibetan-bhutan-lochen': CONVENTION, 'tibetan-lochen': CONVENTION,
    'tibetan-tsurphu': CONVENTION, 'tibetan-tsurphu-karana': CONVENTION,
    'tranquility': NO_ARTICLE, 'truncated-julian-day': NO_ARTICLE, 'valabhi': SECTION,
    'vietnamese-regnal-nguyen': NO_ARTICLE, 'vietnamese-south-1968': CONVENTION,
    'vikram-samvat-kartikadi': CONVENTION, 'week-and-month': NO_ARTICLE, 'yazidi': NO_ARTICLE,
    'yerm': NO_ARTICLE, 'zapotec-yza': NO_ARTICLE, 'zoroastrian-fasli': SHARED,
    'zoroastrian-qadimi': SHARED, 'zoroastrian-shahanshahi': SHARED,
}

# hc-i18n tag -> the Wikipedia edition that writes the language, as the
# interlanguage links key it (`tl` for Filipino, `pnb` for Punjabi in
# Shahmukhi, `zh-yue` for Cantonese), or (`zh`, variant) for the Chinese
# edition's displayed title in a variant.
WIKIS = {
    'am': 'am', 'ar': 'ar', 'ban': 'ban', 'bn': 'bn', 'bo': 'bo', 'cs': 'cs', 'de': 'de',
    'es': 'es', 'fa': 'fa', 'fil': 'tl', 'fr': 'fr', 'ha': 'ha', 'he': 'he', 'hi': 'hi',
    'id': 'id', 'it': 'it', 'ja': 'ja', 'jv': 'jv', 'kab': 'kab', 'ko': 'ko', 'ml': 'ml',
    'mn': 'mn', 'mr': 'mr', 'my': 'my', 'nah': 'nah', 'ne': 'ne', 'nl': 'nl',
    'pa-Arab': 'pnb', 'pa-Guru': 'pa', 'pcm': 'pcm', 'pl': 'pl', 'ps': 'ps', 'pt': 'pt',
    'ru': 'ru', 'sa': 'sa', 'shi-Latn': 'shi', 'sw': 'sw', 'ta': 'ta', 'te': 'te', 'th': 'th',
    'tr': 'tr', 'ur': 'ur', 'vi': 'vi', 'yue-Hant': 'zh-yue', 'zgh': 'zgh',
    'zh-Hans': ('zh', 'zh-hans'), 'zh-Hant': ('zh', 'zh-hant'),
}
REGIONAL = "a regional entry, which the fallback takes to its language's names"
NO_EDITION = 'no Wikipedia edition writes the language'
# The carried tags with no edition to read, and why.
NO_WIKI = {
    'en': ("the calendars' own English names (`CalendarMeta::english_name`) serve English; "
           'the English articles are the ones the links are read from'),
    'und': 'root names nothing',
    'ar-EG': REGIONAL, 'en-001': REGIONAL, 'en-GB': REGIONAL, 'es-419': REGIONAL,
    'pt-PT': REGIONAL, 'ur-IN': REGIONAL, 'zh-Hant-HK': REGIONAL,
    'yue-Hans': 'the Cantonese edition (`zh-yue`) writes Traditional characters only',
    'cop': 'no Coptic Wikipedia, only an Incubator test project',
    'syr': ('no edition keyed `syr`; the Classical Syriac edition is keyed `arc`, Aramaic, '
            'and is not read for it'),
    'aeb-Latn': NO_EDITION, 'ayl-Latn': NO_EDITION, 'mid': NO_EDITION, 'mix': NO_EDITION,
    'rif': NO_EDITION, 'yua': NO_EDITION, 'zap': NO_EDITION,
}
# A locale written in one script whose edition writes in more than one, or
# whose edition's title may be in another: a title in another script is not
# the locale's name.
SCRIPTS = {'shi-Latn': 'LATIN', 'zgh': 'TIFINAGH', 'pa-Arab': 'ARABIC', 'pa-Guru': 'GURMUKHI',
           'kab': 'LATIN', 'mn': 'CYRILLIC', 'sa': 'DEVANAGARI'}

log = []


# --- the API ------------------------------------------------------------------

_last = [0.0]


def api(edition, **params):
    """One MediaWiki API call on `<edition>.wikipedia.org`, paced and retried
    on 429, the JSON parsed."""
    query = urllib.parse.urlencode({**params, 'format': 'json', 'formatversion': '2'})
    url = f'https://{edition}.wikipedia.org/w/api.php?{query}'
    request = urllib.request.Request(url, headers={'User-Agent': USER_AGENT})
    for attempt in range(6):
        time.sleep(max(0.0, _last[0] + PACE - time.monotonic()))
        try:
            with urllib.request.urlopen(request, timeout=60) as response:
                _last[0] = time.monotonic()
                return json.load(response)
        except urllib.error.HTTPError as error:
            _last[0] = time.monotonic()
            if error.code == 429 and attempt < 5:
                time.sleep(float(error.headers.get('Retry-After') or 2 ** (attempt + 1)))
                continue
            raise


def articles(edition, titles):
    """{title: (revision, {edition: title})} for the articles named, their
    interlanguage links whole; a missing title or a redirect is an error,
    because the mapping must name the article itself."""
    out = {}
    for start in range(0, len(titles), 50):
        batch = titles[start:start + 50]
        params = {'action': 'query', 'prop': 'langlinks|info', 'lllimit': 'max',
                  'redirects': '1', 'titles': '|'.join(batch)}
        pages = {}
        while True:
            data = api(edition, **params)
            for redirect in data['query'].get('redirects', []):
                target = redirect['to'] + ('#' + redirect['tofragment']
                                           if redirect.get('tofragment') else '')
                raise SystemExit(f'{edition}: "{redirect["from"]}" redirects to "{target}": '
                                 'name the article itself, or list the identifier as unnamed')
            for page in data['query']['pages']:
                if page.get('missing'):
                    raise SystemExit(f'{edition}: no article "{page["title"]}"')
                entry = pages.setdefault(page['title'], [page.get('lastrevid'), {}])
                if page.get('lastrevid'):
                    entry[0] = page['lastrevid']
                for link in page.get('langlinks', []):
                    entry[1][link['lang']] = link['title']
            if 'continue' not in data:
                break
            params = {**params, **data['continue']}
        normalized = {n['to']: n['from'] for n in data['query'].get('normalized', [])}
        for title, (revision, links) in pages.items():
            out[normalized.get(title, title)] = (revision, links)
    return out


def back_links(edition, source, titles):
    """{title: the title of `source`'s article that each of `titles` links
    to in `edition`, or None where the title is missing, a redirect, or
    links to none}."""
    out = {title: None for title in titles}
    for start in range(0, len(titles), 50):
        batch = titles[start:start + 50]
        params = {'action': 'query', 'prop': 'langlinks', 'lllang': source, 'lllimit': 'max',
                  'redirects': '1', 'titles': '|'.join(batch)}
        data = api(edition, **params)
        query = data['query']
        renamed = {n['from']: n['to'] for n in query.get('normalized', [])}
        redirected = {r['from'] for r in query.get('redirects', [])}
        pages = {page['title']: page for page in query['pages']}
        for title in batch:
            name = renamed.get(title, title)
            if name in redirected or name not in pages or pages[name].get('missing'):
                continue
            links = pages[name].get('langlinks', [])
            if len(links) == 1:
                out[title] = links[0]['title'].split('#', 1)[0]
    return out


def displayed(title, variant):
    """The Chinese edition's title of `title` as it shows it in `variant`."""
    data = api('zh', action='parse', page=title, prop='displaytitle', variant=variant,
               redirects='1')
    text = re.sub(r'<[^>]+>', '', data['parse']['displaytitle'])
    return html.unescape(text).strip()


# --- what is already carried ---------------------------------------------------

def existing_names():
    """{tag: {identifier: name}} of the names `src/data.rs`,
    `src/data/cldr48_locales.rs` and `src/calendar_names.rs` carry, and the
    set of every carried tag."""
    names, tags = {}, set()
    for file in CLDR_FILES:
        with open(file, encoding='utf-8') as handle:
            source = handle.read()
        tag_of = {}
        for block in re.finditer(r': LocaleData = LocaleData \{(.*?)\n\};', source, re.S):
            tag = re.search(r'\n    tag: "([^"]+)"', block.group(1))
            table = re.search(r'\n    calendar_names: (\w+_CALENDAR_NAMES)', block.group(1))
            tags.add(tag.group(1))
            if table:
                tag_of[table.group(1)] = tag.group(1)
        # The entries a helper builds, `calendar_only("rif", ...)`, name no calendar.
        tags.update(re.findall(r': LocaleData = calendar_only\(\s*"([^"]+)"', source))
        for table in re.finditer(r'const (\w+_CALENDAR_NAMES): &\[CalendarDisplayName\] =\s*'
                                 r'&\[(.*?)\];', source, re.S):
            tag = tag_of[table.group(1)]
            for identifier, name in re.findall(
                    r'CalendarDisplayName::new\(\s*"([^"]+)",\s*"([^"]+)",?\s*\)', table.group(2)):
                names.setdefault(tag, {})[identifier] = name
    with open(HAND_FILE, encoding='utf-8') as handle:
        source = handle.read()
    for tag, identifier, name in re.findall(
            r'\(\s*"([^"]+)",\s*CalendarDisplayName::new\(\s*"([^"]+)",\s*"([^"]+)",?\s*\)',
            source, re.S):
        names.setdefault(tag, {})[identifier] = name
    return names, tags


def registry_ids():
    """The identifiers `docs/supported.md`, generated from the registry, lists."""
    with open(SUPPORTED, encoding='utf-8') as handle:
        text = handle.read()
    section = text.split('## Calendars', 1)[1].split('\n## ', 1)[0]
    return re.findall(r'^\| `([a-z0-9-]+)` \|', section, re.M)


def in_script(text, script):
    """Whether every letter of `text` is of `script`, a modifier letter, or,
    for a script other than Latin, a Latin letter, as an initialism such as
    ISO 8601 is written in every script."""
    def allowed(letter):
        name = unicodedata.name(letter, '')
        return (script in name or name.startswith('MODIFIER LETTER')
                or (script != 'LATIN' and 'LATIN' in name))
    return all(allowed(c) for c in text if c.isalpha())


# --- generation -------------------------------------------------------------

def generate():
    ids = registry_ids()
    placed = set(ARTICLES) | set(UNNAMED)
    missing = sorted(set(ids) - placed)
    extra = sorted(placed - set(ids))
    if missing or extra:
        raise SystemExit(f'ARTICLES and UNNAMED must cover the registry exactly: '
                         f'unplaced {missing}, not registered {extra}')
    both = sorted(set(ARTICLES) & set(UNNAMED))
    if both:
        raise SystemExit(f'both named and unnamed: {both}')
    existing, tags = existing_names()
    unplaced = sorted(tags - set(WIKIS) - set(NO_WIKI))
    if unplaced or (set(WIKIS) & set(NO_WIKI)) or sorted(set(WIKIS) | set(NO_WIKI)) != sorted(tags):
        raise SystemExit(f'WIKIS and NO_WIKI must cover the carried tags exactly: {unplaced}, '
                         f'{sorted((set(WIKIS) | set(NO_WIKI)) - tags)}')

    # The articles, read edition by edition.
    by_edition = {}
    for identifier, (edition, title) in sorted(ARTICLES.items()):
        by_edition.setdefault(edition, []).append(title)
    read = {}
    for edition, titles in sorted(by_edition.items()):
        read.update({(edition, title): entry for title, entry in articles(edition, titles).items()})
    for identifier, key in ARTICLES.items():
        if key not in read:
            raise SystemExit(f'{key} was not read (a normalised title?)')

    # Each locale's title of each article, as its edition stores it: the
    # link's title, or the article's own where the locale's edition is the
    # article's. A link into a section of an article (`#`) names that
    # article's subject, not the calendar, and is left out.
    def edition_of(wiki):
        return wiki[0] if isinstance(wiki, tuple) else wiki

    stored = {}
    for tag, wiki in sorted(WIKIS.items()):
        for identifier, key in sorted(ARTICLES.items()):
            edition, title = key
            revision, links = read[key]
            target = edition_of(wiki)
            name = title if edition == target else links.get(target)
            if name is None:
                continue
            if '#' in name:
                log.append(f'{tag} {identifier}: the link "{name}" is into a section, left out')
                continue
            stored[(tag, identifier)] = name

    # Each linked article must link back to the article it was reached
    # from: a link kept by hand into a broader article — the English
    # "Tzolkʼin" into the Chinese article on the Maya calendar as a whole —
    # names that article's subject and is left out, as is a link onto a
    # redirect.
    verified = set()
    to_verify = {}
    for (tag, identifier), name in stored.items():
        edition = ARTICLES[identifier][0]
        target = edition_of(WIKIS[tag])
        if target != edition:
            to_verify.setdefault((target, edition), set()).add(name)
    for (target, edition), titles in sorted(to_verify.items()):
        for name, back in back_links(target, edition, sorted(titles)).items():
            if back is not None:
                verified.add((target, edition, name, back))
    for (tag, identifier), name in sorted(stored.items()):
        edition, title = ARTICLES[identifier]
        target = edition_of(WIKIS[tag])
        if target != edition and (target, edition, name, title) not in verified:
            log.append(f'{tag} {identifier}: "{name}" ({target}) does not link back to '
                       f'"{title}" ({edition}), left out')
            del stored[(tag, identifier)]

    variants = {}

    def title_in(tag, identifier):
        name = stored.get((tag, identifier))
        if name is None:
            return None
        wiki = WIKIS[tag]
        if isinstance(wiki, tuple):
            if (name, wiki[1]) not in variants:
                variants[(name, wiki[1])] = displayed(name, wiki[1])
            name = variants[(name, wiki[1])]
        # A parenthesised disambiguator, "Ab urbe condita (Chronologie)", tells
        # the article from another of the same name and is not part of the
        # name; the article's own name is what is carried.
        plain = re.sub(r' \([^()]*\)$', '', name)
        if plain != name:
            log.append(f'{tag} {identifier}: "{name}" carried as "{plain}"')
        return plain

    table = {}
    for tag, wiki in sorted(WIKIS.items()):
        own = existing.get(tag, {})
        names = {}
        for identifier, key in sorted(ARTICLES.items()):
            name = title_in(tag, identifier)
            if name is None:
                continue
            if identifier in own:
                log.append(f'{tag} {identifier}: CLDR or a hand-written entry names it already, '
                           f'"{own[identifier]}" ({name} left out)')
                continue
            if tag in SCRIPTS and not in_script(name, SCRIPTS[tag]):
                log.append(f'{tag} {identifier}: "{name}" is not in the {SCRIPTS[tag].title()} '
                           'script, left out')
                continue
            if name == identifier:
                log.append(f'{tag} {identifier}: the title is the identifier, left out')
                continue
            names[identifier] = name
        shared = {}
        for identifier, name in list(own.items()) + list(names.items()):
            shared.setdefault(name, []).append(identifier)
        for name, identifiers in shared.items():
            if len(identifiers) > 1:
                for identifier in identifiers:
                    if identifier in names:
                        del names[identifier]
                log.append(f'{tag}: "{name}" would name {", ".join(sorted(identifiers))}; '
                           'the generated ones are left out')
        if names:
            table[tag] = names

    # The file.
    lines = [HEADER.format(read=READ).rstrip('\n'), '//!', '//! The articles read, with the '
             'revision each stood at:', '//!']
    for identifier, (edition, title) in sorted(ARTICLES.items()):
        revision, _ = read[(edition, title)]
        lines.append(f'//! - `{identifier}`: "{title}" ({edition}), revision {revision}')
    lines += ['//!', '//! Not named, no article naming the calendar:', '//!']
    by_reason = {}
    for identifier, reason in UNNAMED.items():
        by_reason.setdefault(reason, []).append(identifier)
    for reason, identifiers in sorted(by_reason.items(), key=lambda item: item[1][0]):
        listed = ', '.join(f'`{i}`' for i in sorted(identifiers))
        lines.append(f'//! - {listed}: {reason}.')
    lines += ['//!', '//! Locales with no edition to read:', '//!']
    for tag, reason in sorted(NO_WIKI.items()):
        lines.append(f'//! - `{tag}`: {reason}.')
    lines += ['', 'use crate::names::CalendarDisplayName;', '',
              '/// Each carried locale\'s Wikipedia titles for the calendars CLDR and the',
              '/// hand-written entries do not name in it, sorted by tag so that a lookup can',
              '/// search it; the identifiers sorted within a tag.',
              'pub static WIKIPEDIA_CALENDAR_NAMES: &[(&str, &[CalendarDisplayName])] = &[']
    dump = []
    for tag in sorted(table):
        lines.append(f'    ({lit(tag)}, &[')
        for identifier in sorted(table[tag]):
            lines.append(f'        CalendarDisplayName::new({lit(identifier)}, '
                         f'{lit(table[tag][identifier])}),')
            dump.append(f'{tag}\t{identifier}\t{table[tag][identifier]}')
        lines.append('    ]),')
    lines.append('];')
    for tag in sorted(tags):
        dump.append(f'count\t{tag}\t{len(existing.get(tag, {}))}\t{len(table.get(tag, {}))}')
    return '\n'.join(lines) + '\n', dump


HEADER = '''\
//! What each locale's Wikipedia calls the calendars CLDR has no key for,
//! generated.
//!
//! **Do not edit.** `scripts/calendar-names-wikipedia.py` writes this file
//! from the interlanguage links of Wikipedia's articles on the registry's
//! calendars (the MediaWiki API, `action=query&prop=langlinks`, and for the
//! Chinese edition's two scripts `action=parse&prop=displaytitle` in the
//! `zh-hans` and `zh-hant` variants; read {read},
//! `wikipedia-calendar-names`), as the script's documentation says: for
//! each calendar the script names an article about it, and each carried
//! locale whose edition the links reach takes the title of that edition's
//! article, exactly as the edition writes it; nothing is translated, and a
//! calendar no edition of a language has an article on keeps no name in
//! it, so that a page shows its English name. A calendar CLDR or
//! `crate::calendar_names` names in a locale is not here, since those
//! answer first; a title that would give two calendars one name in a locale
//! is left out, as is one written in a script other than the locale's.
//! `crate::calendar_names::sourced_calendar_name` reads the table.
'''


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump + log))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/calendar-names-wikipedia.py')
    if log and '--check' not in sys.argv:
        print('\n'.join(log), file=sys.stderr)


if __name__ == '__main__':
    main()
