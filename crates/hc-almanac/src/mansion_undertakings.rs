//! The undertakings each 二十八宿 favours and forbids, list by list.
//!
//! A commercial almanac prints, for each mansion's day, what to do and
//! what to avoid: 角宿 is good for cutting cloth and raising a ridgepole
//! and bad for funerals. No national body publishes such a list — the
//! National Astronomical Observatory of Japan's 暦Wiki and the National
//! Diet Library give none — so there is no *the* list, only publishers'.
//! Each list here is attributed to its publisher and carried as it
//! prints it, as `hc-attributes` carries its birthstones: there is no
//! function that answers for "the" tradition. [`crate::Mansion::fortune`]
//! is the separate 吉/凶 flag, and [`crate::Mansion::undisputed_note`]
//! the two statements every source makes.
//! `docs/systems/japanese-almanac-notes.md` compares the lists.
//!
//! # What is carried
//!
//! Two lists, each read on 2026-09-29, in Japanese as printed:
//!
//! * 歳事暦 「暦の吉凶 二十八宿」 (<https://saijigoyomi.com/28shuku/>,
//!   posted 4 April 2023, updated 9 December 2023), `saijigoyomi-28shuku`;
//! * うまずたゆまず 「二十八宿（にじゅうはっしゅく）」
//!   (<https://www.linderabell.com/entry/koyomi/28>, 1 April 2025, updated
//!   8 April 2025), `linderabell-28shuku`.
//!
//! Each prints a 吉 and a 凶 line per mansion, some a 大吉 or 大凶 line,
//! and a remark about the day as a whole. The lines are split at 「、」
//! into [`Undertakings`]' four lists, in the publisher's order; a remark,
//! and a parenthesis that says what befalls the one who does the thing,
//! go in [`Undertakings::note`]. A remark a publisher puts inside a line
//! — 歳事暦's 「婚礼だが一般にはあまり用いない日」 for 胃宿 — is a note
//! too. The spellings are the publisher's (歳事暦's 家族団らん and
//! しょうゆ, うまずたゆまず's 家族団欒 and 醤油).
//!
//! The two are one witness, not two: every list agrees with the other
//! but for order, spelling, a particle or a comma, and 觜宿, where
//! 歳事暦 alone avoids 衣類の着初め
//! (`the_two_lists_differ_only_where_the_documentation_says`). A third
//! page read, 神仏.ネット's 「28宿(二十八宿)の意味とは」, writes its
//! entries as sentences, not lists, and differs in substance (it calls
//! 昴宿 a 吉祥の宿 good for everything but cutting cloth). It is not yet
//! carried, because its entries are prose sentences under that site's
//! copyright, and a list of undertakings drawn from them would be this
//! crate's reading of them rather than the publisher's list.

use crate::mansions::Mansion;

/// What one list says of one mansion's day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Undertakings {
    /// 大吉: what the day is best for.
    pub best: &'static [&'static str],
    /// 吉: what the day is good for.
    pub favoured: &'static [&'static str],
    /// 凶: what to avoid.
    pub avoided: &'static [&'static str],
    /// 大凶: what to avoid most.
    pub worst: &'static [&'static str],
    /// The remark the list makes about the day as a whole, empty where it
    /// makes none.
    pub note: &'static str,
}

/// One publisher's list for the twenty-eight, 角宿 first.
#[derive(Debug, Clone, Copy)]
pub struct UndertakingList {
    /// The identifier, e.g. `"saijigoyomi"`.
    pub id: &'static str,
    /// Who publishes it and where.
    pub source: &'static str,
    /// The entries, in cycle order.
    pub entries: &'static [Undertakings; 28],
}

impl UndertakingList {
    /// What this list says of a mansion's day.
    #[must_use]
    pub const fn of(&self, mansion: Mansion) -> &'static Undertakings {
        &self.entries[mansion.index() as usize]
    }
}

/// An entry with no 大吉 and no 大凶 line.
const fn plain(
    favoured: &'static [&'static str],
    avoided: &'static [&'static str],
    note: &'static str,
) -> Undertakings {
    Undertakings {
        best: &[],
        favoured,
        avoided,
        worst: &[],
        note,
    }
}

/// An entry with every line.
const fn graded(
    best: &'static [&'static str],
    favoured: &'static [&'static str],
    avoided: &'static [&'static str],
    worst: &'static [&'static str],
    note: &'static str,
) -> Undertakings {
    Undertakings {
        best,
        favoured,
        avoided,
        worst,
        note,
    }
}

/// 歳事暦's list.
static SAIJIGOYOMI: [Undertakings; 28] = [
    // 角
    plain(
        &[
            "衣類裁断",
            "着初め",
            "棟上げ",
            "建築",
            "普請造作",
            "柱立て",
            "井戸掘り",
            "酒造り",
            "婚礼",
            "開店",
            "神仏祭祀",
            "新しい事を始めるなど",
        ],
        &["葬式", "納骨"],
        "",
    ),
    // 亢
    plain(
        &[
            "結納",
            "婚礼",
            "種まき",
            "取り入れ",
            "衣類仕立て",
            "習い事始め",
            "贈り物",
            "友人に会う",
            "物品購入など",
        ],
        &["普請", "建築", "造作", "不動産売買", "移転", "旅行"],
        "",
    ),
    // 氐
    plain(
        &[
            "婚礼",
            "見合い",
            "農耕全般",
            "新改築",
            "酒造り",
            "移転",
            "開店",
            "新しい事を始めるなど",
        ],
        &["衣類の着初め", "大きな事", "水に近づくこと"],
        "",
    ),
    // 房
    plain(
        &[
            "婚礼",
            "神事など祝い事全般",
            "旅行",
            "造作",
            "棟上げ",
            "柱立て",
            "衣類裁断",
            "移転",
            "新しい事を始めるなど",
        ],
        &["訴訟", "不倫など"],
        "驕りや不遜は禁物の日",
    ),
    // 心
    plain(
        &["神事", "仏事", "移転", "旅行", "衣類の着初めなど"],
        &[
            "婚礼",
            "葬送",
            "普請",
            "造作",
            "投資や仕入れなど出費にまつわること",
        ],
        "",
    ),
    // 尾
    plain(
        &[
            "婚礼",
            "開店",
            "移転",
            "旅行",
            "薬合わせ",
            "造作",
            "建築",
            "新しい事を始める",
            "修行・勉強始めなど",
        ],
        &["衣類裁断", "衣類の着初め", "葬送"],
        "",
    ),
    // 箕
    plain(
        &[
            "酒・しょうゆ造り",
            "商品の仕入れ",
            "契約",
            "池や水路を構築",
            "種まき",
            "動土",
            "集金など",
        ],
        &["婚礼", "葬式", "納骨"],
        "驕りは禁物の日",
    ),
    // 斗
    plain(
        &[
            "婚礼",
            "不動産売買",
            "造作",
            "動土",
            "井戸掘り",
            "後まで残る物事",
            "事業開拓",
            "倉庫の建築",
            "造園",
            "車両の新調",
            "新しい事を始めるなど",
        ],
        &["その他のこと"],
        "",
    ),
    // 牛
    plain(
        &[],
        &[],
        "吉祥宿なので何事にも用いて吉。とくに午の刻（11時から13時）が大吉祥",
    ),
    // 女
    plain(
        &[
            "公務・職務・芸能に関わること",
            "武器を造る",
            "学芸の稽古始め",
            "美容・理容院に行く",
            "神仏を拝むなど",
        ],
        &[
            "訴訟",
            "婚礼",
            "葬式",
            "争い事",
            "掛け合い事",
            "衣類新調",
            "着初め",
            "新築",
            "造作",
            "引っ越し",
            "新しい事・大きな事を始める",
        ],
        "",
    ),
    // 虚
    graded(
        &[],
        &["入学", "習い事始め", "家族団らん", "衣類の着初めなど"],
        &["建築", "婚礼", "縁談", "葬式", "納骨", "祝い事", "祭事"],
        &["相談事"],
        "急ぎの事であっても慎重に。万事骨折り損の凶日",
    ),
    // 危
    graded(
        &[],
        &[
            "壁塗り",
            "かまど造り",
            "出張",
            "精神的鍛錬",
            "レジャー",
            "船の普請",
            "動土",
            "酒造",
            "公務など",
        ],
        &["婚礼", "衣類裁断", "釘打ち", "引っ越し", "大きな事"],
        &["高所での仕事", "登山"],
        "何事も慎重になるべき日",
    ),
    // 室
    plain(
        &[
            "神仏祭祀",
            "祝い事",
            "祈願",
            "婚礼",
            "船乗り",
            "造作",
            "戦",
            "狩猟",
            "柱立て",
            "井戸掘り",
            "薬の飲み始め",
            "理髪など",
        ],
        &["葬式", "納骨", "遠出"],
        "",
    ),
    // 壁
    plain(
        &[
            "新築改修",
            "新事業造作",
            "婚礼",
            "葬式",
            "契約",
            "衣類の着初めなど",
        ],
        &["南に進出", "名付け"],
        "衣類裁断に用いると子孫繁栄",
    ),
    // 奎
    plain(
        &[
            "婚礼",
            "棟上げ",
            "柱立て",
            "井戸掘り",
            "伐木",
            "旅立ち",
            "神仏祈願",
            "祭事",
            "宮造り",
            "会合",
            "衣類裁断など",
        ],
        &["開店など新規の事", "訴訟", "交渉など"],
        "",
    ),
    // 婁
    graded(
        &["衣類裁断は増益あり寿命が伸びる"],
        &[
            "婚礼の相談",
            "婚礼",
            "契約",
            "取引始め",
            "旅行",
            "美容",
            "動土",
            "建築",
            "造作",
            "造園",
            "衣類の着初め",
            "休息に関する事",
        ],
        &["訴訟", "判断すること", "改革"],
        &[],
        "急ぎの用事など諸事に用いて吉の日",
    ),
    // 胃
    graded(
        &[],
        &["公事に関する事", "就職", "婚礼"],
        &["造作", "衣類裁断", "私事"],
        &["葬儀"],
        "一般にはあまり用いない日。王者が善を修するに良い日",
    ),
    // 昴
    plain(
        &["神仏祈願", "手斧始め", "祝い事", "新規開店", "家畜購入など"],
        &["衣類裁断", "家の増改築", "争い事"],
        "",
    ),
    // 畢
    plain(
        &[
            "神事",
            "婚礼",
            "棟上げ",
            "新築",
            "増改築",
            "屋根葺き",
            "造作",
            "蔵造り",
            "不動産取得",
            "農耕",
            "契約事など",
        ],
        &["投資・仕入れ・返済などの出費に関する事", "口論"],
        "",
    ),
    // 觜
    plain(
        &[
            "入学",
            "稽古始め",
            "神仏祭祀",
            "建築土木",
            "山仕事始め",
            "転居など",
        ],
        &["衣類の着初め", "造作", "投資", "開店", "事業の新規拡張"],
        "投資などに用いると家財を失う。婚礼に用いると金銀を散じ病に悩む悪日",
    ),
    // 参
    plain(
        &[
            "物品の仕入れ",
            "商品の買い付け",
            "倉庫納入",
            "販売などの商取引",
            "開業",
            "造作",
            "建築全般",
            "土木全般",
            "新規取引開始",
            "婚礼",
            "就職",
            "旅立ち",
            "祝い事",
            "養子縁組など",
        ],
        &["葬式", "転居", "賭け事"],
        "",
    ),
    // 井
    plain(
        &[
            "神事",
            "種まき",
            "婚礼",
            "建築",
            "動土",
            "普請造作",
            "井戸掘り",
            "落成式",
            "商談",
            "不動産売買など",
        ],
        &["衣類裁断", "葬式", "治療始め", "争い事"],
        "人に施した福徳が自分に戻る働きを含む日。衣類裁断すれば離婚する",
    ),
    // 鬼
    graded(
        &["よろずよろし"],
        &[],
        &["婚礼"],
        &[],
        "二十八宿でもっとも格が高く、公の事、とくに式典などに適する。一般の祝い事も全て吉",
    ),
    // 柳
    graded(
        &[],
        &["剛猛の事", "物事を断わる"],
        &["婚礼", "新規事業", "普請造作", "衣類裁断"],
        &["葬送"],
        "葬送すると不幸が重なる。一般には用いない日",
    ),
    // 星
    plain(
        &[
            "運転始め",
            "療養始め",
            "乗馬始め",
            "種まき",
            "改築",
            "祖先の祭祀など",
        ],
        &["婚礼", "祝い事", "葬式", "納骨", "不倫"],
        "",
    ),
    // 張
    graded(
        &["種まき", "養蚕"],
        &[
            "婚礼",
            "就職",
            "神仏祈願",
            "新築",
            "開業",
            "事業の拡張",
            "見合い",
            "祝宴",
            "和合事など",
        ],
        &["衣類裁断", "樹木を切るなど"],
        &[],
        "",
    ),
    // 翼
    plain(
        &[
            "種まき",
            "耕作始め",
            "樹木の植え替え",
            "農耕全般",
            "草刈り",
            "建築",
            "土木",
            "出張",
            "旅行など",
        ],
        &["高所での仕事", "入学試験", "掛け合い事"],
        "万事に用心が必要な日。公の行事、祝い事、祭り事には用いない。婚礼は離婚となる",
    ),
    // 軫
    plain(
        &[
            "婚礼",
            "棟上げ",
            "不動産売買",
            "神仏祭祀",
            "地鎮祭",
            "落成式",
            "建築",
            "祝い事",
            "万事新規の事",
        ],
        &["衣類裁断", "衣類の着初め", "旅行"],
        "衣類を裁断すると火難に遭う",
    ),
];

/// うまずたゆまず's list.
static LINDERABELL: [Undertakings; 28] = [
    // 角
    plain(
        &[
            "衣類裁断",
            "着初め",
            "棟上げ",
            "建築",
            "普請造作",
            "柱立て",
            "井戸掘り",
            "酒造り",
            "婚礼",
            "開店",
            "神仏祭祀",
            "新しい事を始めるなど",
        ],
        &["葬式", "納骨"],
        "",
    ),
    // 亢
    plain(
        &[
            "結納",
            "婚礼",
            "種まき",
            "取り入れ",
            "衣類仕立て",
            "習い事始め",
            "贈り物",
            "友人に会う",
            "物品購入など",
        ],
        &["普請", "建築", "造作", "不動産売買", "移転", "旅行"],
        "",
    ),
    // 氐
    plain(
        &[
            "婚礼",
            "見合い",
            "農耕全般",
            "新改築",
            "酒造り",
            "移転",
            "開店",
            "新しい事を始めるなど",
        ],
        &["衣類の着初め", "大きな事", "水に近づくこと"],
        "",
    ),
    // 房
    plain(
        &[
            "婚礼",
            "神事など祝い事全般",
            "旅行",
            "造作",
            "棟上げ",
            "柱立て",
            "移転",
            "衣類裁断",
            "新しい事を始めるなど",
        ],
        &["訴訟", "不倫など"],
        "驕りや不遜は禁物の日",
    ),
    // 心
    plain(
        &["神事", "仏事", "移転", "旅行", "衣類の着初めなど"],
        &[
            "婚礼",
            "葬送",
            "普請",
            "造作",
            "投資や仕入れなど出費にまつわること",
        ],
        "",
    ),
    // 尾
    plain(
        &[
            "婚礼",
            "開店",
            "移転",
            "旅行",
            "薬合わせ",
            "造作",
            "建築",
            "新しい事を始める",
            "修行・勉強始めなど",
        ],
        &["衣類裁断", "衣類の着初め", "葬送"],
        "",
    ),
    // 箕
    plain(
        &[
            "酒・醤油造り",
            "商品の仕入れ",
            "契約",
            "池や水路を構築",
            "種まき",
            "動土",
            "集金など",
        ],
        &["婚礼", "葬式", "納骨"],
        "驕りは禁物の日",
    ),
    // 斗
    plain(
        &[
            "婚礼",
            "不動産売買",
            "造作",
            "動土",
            "井戸掘り",
            "後まで残る物事",
            "事業開拓",
            "倉庫の建築",
            "造園",
            "車両の新調",
            "新しい事を始めるなど",
        ],
        &["その他のこと"],
        "",
    ),
    // 牛
    plain(
        &[],
        &[],
        "吉祥宿なので何事にも用いて吉。特に午の刻（11時から13時）が大吉祥",
    ),
    // 女
    plain(
        &[
            "公務・職務・芸能に関わること",
            "武器を造る",
            "学芸の稽古始め",
            "美容・理容院に行く",
            "神仏を拝むなど",
        ],
        &[
            "訴訟",
            "婚礼",
            "葬式",
            "争い事",
            "掛け合い事",
            "衣類新調",
            "着初め",
            "新築",
            "造作",
            "引っ越し",
            "新しい事・大きな事を始める",
        ],
        "",
    ),
    // 虚
    graded(
        &[],
        &["入学", "習い事始め", "家族団欒", "衣類の着初めなど"],
        &["建築", "婚礼", "縁談", "祝い事", "祭事", "葬式", "納骨"],
        &["相談事"],
        "万事骨折り損の凶日。急ぎの事であっても慎重に",
    ),
    // 危
    graded(
        &[],
        &[
            "壁塗り",
            "かまど造り",
            "出張",
            "精神的鍛錬",
            "レジャー",
            "船の普請",
            "動土",
            "酒造",
            "公務など",
        ],
        &["婚礼", "衣類裁断", "釘打ち", "引っ越し", "大きな事"],
        &["高所での仕事", "登山"],
        "何事も慎重になるべき日",
    ),
    // 室
    plain(
        &[
            "神仏祭祀",
            "祝い事",
            "祈願",
            "婚礼",
            "船乗り",
            "造作",
            "戦",
            "狩猟",
            "柱立て",
            "井戸掘り",
            "薬の飲み始め",
            "理髪など",
        ],
        &["葬式", "納骨", "遠出"],
        "",
    ),
    // 壁
    plain(
        &[
            "新築改修",
            "新事業造作",
            "婚礼",
            "葬式",
            "契約",
            "衣類の着初めなど",
        ],
        &["南に進出", "名付け"],
        "衣類裁断に用いると子孫繁栄",
    ),
    // 奎
    plain(
        &[
            "婚礼",
            "棟上げ",
            "柱立て",
            "井戸掘り",
            "伐木",
            "旅立ち",
            "神仏祈願",
            "祭事",
            "宮造り",
            "会合",
            "衣類裁断など",
        ],
        &["開店など新規の事", "訴訟", "交渉など"],
        "",
    ),
    // 婁
    graded(
        &["衣類裁断は増益あり、寿命が伸びる"],
        &[
            "婚礼の相談",
            "婚礼",
            "契約",
            "取引始め",
            "旅行",
            "美容",
            "動土",
            "建築",
            "造作",
            "造園",
            "衣類の着初め",
            "休息に関する事",
        ],
        &["訴訟", "判断すること", "改革"],
        &[],
        "急ぎの用事など諸事に用いて吉の日",
    ),
    // 胃
    graded(
        &[],
        &["公事に関する事", "就職", "婚礼"],
        &["造作", "衣類裁断", "私事"],
        &["葬儀"],
        "一般にはあまり用いない日、王者が善を修するに良い日",
    ),
    // 昴
    plain(
        &["神仏祈願", "手斧始め", "祝い事", "新規開店", "家畜購入など"],
        &["衣類裁断", "家の増改築", "争い事"],
        "",
    ),
    // 畢
    plain(
        &[
            "神事",
            "婚礼",
            "棟上げ",
            "新築",
            "増改築",
            "屋根葺き",
            "造作",
            "蔵造り",
            "不動産取得",
            "農耕",
            "契約事など",
        ],
        &["投資・仕入れ・返済など、出費に関する事", "口論"],
        "",
    ),
    // 觜
    plain(
        &[
            "入学",
            "稽古始め",
            "神仏祭祀",
            "建築土木",
            "山仕事始め",
            "転居など",
        ],
        &["造作", "投資", "開店", "事業の新規拡張"],
        "投資などに用いると家財を失う。婚礼に用いると金銀を散じ、病に悩む悪日",
    ),
    // 参
    plain(
        &[
            "物品の仕入れ",
            "商品の買い付け",
            "倉庫納入",
            "販売などの商取引",
            "開業",
            "造作",
            "建築全般",
            "土木全般",
            "新規取引開始",
            "婚礼",
            "就職",
            "旅立ち",
            "祝い事",
            "養子縁組など",
        ],
        &["葬式", "転居", "賭け事"],
        "",
    ),
    // 井
    plain(
        &[
            "神事",
            "種まき",
            "婚礼",
            "建築",
            "動土",
            "普請造作",
            "井戸掘り",
            "落成式",
            "商談",
            "不動産売買など",
        ],
        &["衣類裁断", "葬式", "治療始め", "争い事"],
        "人に施した福徳が自分に戻る働きを含む日。衣類裁断すれば離婚する",
    ),
    // 鬼
    graded(
        &["よろずよろし"],
        &[],
        &["婚礼"],
        &[],
        "大吉日。「二十八宿」で最も格が高く、公の事、特に式典などに適する。一般の祝い事も全て吉",
    ),
    // 柳
    graded(
        &[],
        &["剛猛の事", "物事を断わる"],
        &["婚礼", "新規事業", "普請造作", "衣類裁断"],
        &["葬送"],
        "一般には用いない日。葬送すると不幸が重なる",
    ),
    // 星
    plain(
        &[
            "運転始め",
            "療養始め",
            "乗馬始め",
            "種まき",
            "改築",
            "祖先の祭祀など",
        ],
        &["婚礼", "祝い事", "葬式", "納骨", "不倫"],
        "",
    ),
    // 張
    graded(
        &["種まき", "養蚕"],
        &[
            "婚礼",
            "見合い",
            "祝宴",
            "和合事",
            "就職",
            "神仏祈願",
            "新築",
            "開業",
            "事業の拡張など",
        ],
        &["衣類裁断", "樹木を切るなど"],
        &[],
        "大吉日",
    ),
    // 翼
    plain(
        &[
            "種まき",
            "耕作始め",
            "樹木の植え替え",
            "農耕全般",
            "草刈り",
            "建築",
            "土木",
            "出張",
            "旅行など",
        ],
        &["高所での仕事", "入学試験", "掛け合い事"],
        "万事に用心が必要な日。公の行事、祝い事、祭り事には用いない。婚礼は離婚となる",
    ),
    // 軫
    plain(
        &[
            "婚礼",
            "棟上げ",
            "不動産売買",
            "神仏祭祀",
            "地鎮祭",
            "落成式",
            "建築",
            "祝い事",
            "万事新規の事",
        ],
        &["衣類裁断", "衣類の着初め", "旅行"],
        "衣類を裁断すると火難に遭う",
    ),
];

hc_core::catalogue! {
    type: UndertakingList,
    id: |list| list.id,
    provenance: |list| list.source,
    tests: undertaking_list_tests,
    associated;

    /// Every list, in the order they were read.
    pub const ALL;
    /// The list with this identifier.
    pub fn by_id;

    entries: {
        /// 歳事暦, 「暦の吉凶 二十八宿」.
        pub const SAIJIGOYOMI = Self {
            id: "saijigoyomi",
            source: "歳事暦, 「暦の吉凶 二十八宿」, https://saijigoyomi.com/28shuku/, updated 2023-12-09, read 2026-09-29",
            entries: &SAIJIGOYOMI,
        };
        /// うまずたゆまず, 「二十八宿（にじゅうはっしゅく）」.
        pub const LINDERABELL = Self {
            id: "linderabell",
            source: "うまずたゆまず, 「二十八宿（にじゅうはっしゅく）」, https://www.linderabell.com/entry/koyomi/28, updated 2025-04-08, read 2026-09-29",
            entries: &LINDERABELL,
        };
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::vec::Vec;

    use super::*;

    /// Both lists say what the undisputed notes say: 鬼宿 is best for
    /// everything but marriage, and 牛宿 lists nothing to avoid.
    #[test]
    fn the_lists_agree_with_the_undisputed_notes() {
        for list in UndertakingList::ALL {
            let ghost = list.of(Mansion::GHOST);
            assert_eq!(ghost.best, &["よろずよろし"], "{}", list.id);
            assert_eq!(ghost.avoided, &["婚礼"], "{}", list.id);
            let ox = list.of(Mansion::OX);
            assert!(ox.avoided.is_empty() && ox.worst.is_empty(), "{}", list.id);
            assert!(ox.note.contains("何事にも用いて吉"), "{}", list.id);
            assert_eq!(list.of(Mansion::HORN).avoided, &["葬式", "納骨"]);
        }
    }

    /// Every entry has something in it, and no item is empty or repeated
    /// within its line.
    #[test]
    fn every_entry_is_filled_and_no_item_repeats() {
        for list in UndertakingList::ALL {
            for (index, entry) in list.entries.iter().enumerate() {
                let lines = [entry.best, entry.favoured, entry.avoided, entry.worst];
                assert!(
                    lines.iter().any(|line| !line.is_empty()) || !entry.note.is_empty(),
                    "{} {index}",
                    list.id
                );
                for line in lines {
                    let distinct: BTreeSet<_> = line.iter().collect();
                    assert_eq!(distinct.len(), line.len(), "{} {index}", list.id);
                    assert!(line.iter().all(|item| !item.is_empty()));
                }
            }
        }
    }

    /// The two lists as sets, spelling, commas, a closing など and 畢宿's
    /// 「などの」 against 「など、」 made one: they differ in 觜宿 alone, where 歳事暦 avoids 衣類の着初め and うまずたゆまず does not.
    #[test]
    fn the_two_lists_differ_only_where_the_documentation_says() {
        let normalise = |item: &str| {
            let item = item
                .replace("家族団欒", "家族団らん")
                .replace("醤油", "しょうゆ")
                .replace("、", "")
                .replace("などの", "など");
            String::from(item.strip_suffix("など").unwrap_or(&item))
        };
        let set =
            |line: &[&str]| -> BTreeSet<_> { line.iter().map(|item| normalise(item)).collect() };
        let mut differences = Vec::new();
        for mansion in Mansion::all() {
            let (one, other) = (
                UndertakingList::SAIJIGOYOMI.of(mansion),
                UndertakingList::LINDERABELL.of(mansion),
            );
            for (line, (a, b)) in [
                ("best", (one.best, other.best)),
                ("favoured", (one.favoured, other.favoured)),
                ("avoided", (one.avoided, other.avoided)),
                ("worst", (one.worst, other.worst)),
            ] {
                for item in set(a).symmetric_difference(&set(b)) {
                    differences.push((mansion.japanese_name(), line, item.clone()));
                }
            }
        }
        assert_eq!(
            differences,
            [("觜", "avoided", String::from("衣類の着初め"))]
        );
    }
}
