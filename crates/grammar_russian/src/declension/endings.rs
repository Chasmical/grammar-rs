use crate::{categories::*, declension::*, util::slice_find};

// All endings of nouns, adjectives and pronouns in one 55-char span
const ENDINGS: &[u8] = "оегоговыеейёмойёйамийаямиемуююахяяхыйыхымихомуимиевёвью".as_bytes();

#[rustfmt::skip]
const NOUN_LOOKUP: [(u8, u8); 288] = [
    //    stem types: 1, 2,   3,    4,    5,    6,   7,   8
    /* nom sg masc */ е, ь,   null, null, null, й,   й,   ь,
    /* nom sg n    */ о, е_ё, о,    е_о,  е_о,  е_ё, е_ё, о,
    /* nom sg fem  */ а, я,   а,    а,    а,    я,   я,   ь,
    //    stem types: 1, 2, 3, 4, 5, 6, 7, 8
    /* nom pl masc */ ы, и, и, и, ы, и, и, и,
    /* nom pl n    */ а, я, а, а, а, я, я, а,
    /* nom pl fem  */ ы, и, и, и, ы, и, и, и,

    //    stem types: 1, 2, 3, 4, 5, 6, 7, 8
    /* gen sg masc */ а, я, а, а, а, я, я, и,
    /* gen sg n    */ а, я, а, а, а, я, я, а,
    /* gen sg fem  */ ы, и, и, и, ы, и, и, и,
    //    stem types: 1,    2,    3,    4,       5,     6,     7,     8
    /* gen pl masc */ ов,   ей,   ов,   ей,      ев_ов, ев_ёв, ев_ёв, ей,
    /* gen pl n    */ null, ь_ей, null, null_ей, null,  й,     й,     null,
    /* gen pl fem  */ null, ь_ей, null, null_ей, null,  й,     й,     ей,

    //    stem types: 1, 2, 3, 4, 5, 6, 7,   8
    /* dat sg masc */ у, ю, у, у, у, ю, ю,   и,
    /* dat sg n    */ у, ю, у, у, у, ю, ю,   у,
    /* dat sg fem  */ е, е, е, е, е, е, и_е, и,
    //    stem types: 1,  2,  3,  4,  5,  6,  7,  8
    /* dat pl masc */ ам, ям, ам, ам, ам, ям, ям, ям,
    /* dat pl n    */ ам, ям, ам, ам, ам, ям, ям, ам,
    /* dat pl fem  */ ам, ям, ам, ам, ам, ям, ям, ям,

    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* acc sg masc */ acc, acc, acc, acc, acc, acc, acc, acc,
    /* acc sg n    */ acc, acc, acc, acc, acc, acc, acc, acc,
    /* acc sg fem  */ у,   ю,   у,   у,   у,   ю,   ю,   ь,
    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* acc pl masc */ acc, acc, acc, acc, acc, acc, acc, acc,
    /* acc pl n    */ acc, acc, acc, acc, acc, acc, acc, acc,
    /* acc pl fem  */ acc, acc, acc, acc, acc, acc, acc, acc,

    //    stem types: 1,  2,     3,  4,     5,     6,     7,     8
    /* ins sg masc */ ом, ем_ём, ом, ем_ом, ем_ом, ем_ём, ем_ём, ем_ём,
    /* ins sg n    */ ом, ем_ём, ом, ем_ом, ем_ом, ем_ём, ем_ём, ом,
    /* ins sg fem  */ ой, ей_ёй, ой, ей_ой, ей_ой, ей_ёй, ей_ёй, ью,
    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* ins pl masc */ ами, ями, ами, ами, ами, ями, ями, ями,
    /* ins pl n    */ ами, ями, ами, ами, ами, ями, ями, ами,
    /* ins pl fem  */ ами, ями, ами, ами, ами, ями, ями, ями,

    //    stem types: 1, 2, 3, 4, 5, 6, 7,   8
    /* prp sg masc */ е, е, е, е, е, е, и_е, и,
    /* prp sg n    */ е, е, е, е, е, е, и_е, и,
    /* prp sg fem  */ е, е, е, е, е, е, и_е, и,
    //    stem types: 1,  2,  3,  4,  5,  6,  7,  8
    /* prp pl masc */ ах, ях, ах, ах, ах, ях, ях, ях,
    /* prp pl n    */ ах, ях, ах, ах, ах, ях, ях, ах,
    /* prp pl fem  */ ах, ях, ах, ах, ах, ях, ях, ях,
];

#[rustfmt::skip]
const ADJ_LOOKUP: [(u8, u8); 196] = [
    // stem types: 1,     2,  3,     4,     5,     6,  7
    /* nom masc */ ый_ой, ий, ий_ой, ий_ой, ый_ой, ий, ий,
    /* nom n    */ ое,    ее, ое,    ее_ое, ее_ое, ее, ее,
    /* nom fem  */ ая,    яя, ая,    ая,    ая,    яя, яя,
    /* nom pl   */ ые,    ие, ие,    ие,    ые,    ие, ие,

    // stem types: 1,   2,   3,   4,       5,       6,   7
    /* gen masc */ ого, его, ого, его_ого, его_ого, его, его,
    /* gen n    */ ого, его, ого, его_ого, его_ого, его, его,
    /* gen fem  */ ой,  ей,  ой,  ей_ой,   ей_ой,   ей,  ей,
    /* gen pl   */ ых,  их,  их,  их,      ых,      их,  их,

    // stem types: 1,   2,   3,   4,       5,       6,   7
    /* dat masc */ ому, ему, ому, ему_ому, ему_ому, ему, ему,
    /* dat n    */ ому, ему, ому, ему_ому, ему_ому, ему, ему,
    /* dat fem  */ ой,  ей,  ой,  ей_ой,   ей_ой,   ей,  ей,
    /* dat pl   */ ым,  им,  им,  им,      ым,      им,  им,

    // stem types: 1,   2,   3,   4,   5,   6,   7
    /* acc masc */ acc, acc, acc, acc, acc, acc, acc,
    /* acc n    */ acc, acc, acc, acc, acc, acc, acc,
    /* acc fem  */ ую,  юю,  ую,  ую,  ую,  юю,  юю,
    /* acc pl   */ acc, acc, acc, acc, acc, acc, acc,

    // stem types: 1,   2,   3,   4,     5,     6,   7
    /* ins masc */ ым,  им,  им,  им,    ым,    им,  им,
    /* ins n    */ ым,  им,  им,  им,    ым,    им,  им,
    /* ins fem  */ ой,  ей,  ой,  ей_ой, ей_ой, ей,  ей,
    /* ins pl   */ ыми, ими, ими, ими,   ыми,   ими, ими,

    // stem types: 1,  2,  3,  4,     5,     6,  7
    /* prp masc */ ом, ем, ом, ем_ом, ем_ом, ем, ем,
    /* prp n    */ ом, ем, ом, ем_ом, ем_ом, ем, ем,
    /* prp fem  */ ой, ей, ой, ей_ой, ей_ой, ей, ей,
    /* prp pl   */ ых, их, их, их,    ых,    их, их,

    // stem types: 1,    2,   3,    4,    5,    6,   7
    /* srt masc */ null, ь,   null, null, null, й,   й,
    /* srt n    */ о,    е_ё, о,    е_о,  е_о,  е_ё, е_ё,
    /* srt fem  */ а,    я,   а,    а,    а,    я,   я,
    /* srt pl   */ ы,    и,   и,    и,    ы,    и,   и,
];

#[rustfmt::skip]
const PRO_LOOKUP: [(u8, u8); 168] = [
    // stem types: 1,    2,   3,    4,    5,    6,   7
    /* nom masc */ null, ь,   null, null, null, й,   й,
    /* nom n    */ о,    е_ё, о,    е_о,  е_о,  е_ё, е_ё,
    /* nom fem  */ а,    я,   а,    а,    а,    я,   я,
    /* nom pl   */ ы,    и,   и,    и,    ы,    и,   и,

    // stem types: 1,  2,  3,   4,       5,       6,   7
    /* gen masc */ а,  я,  ого, его_ого, его_ого, его, его,
    /* gen n    */ а,  я,  ого, его_ого, его_ого, его, его,
    /* gen fem  */ ой, ей, ой,  ей_ой,   ей_ой,   ей,  ей,
    /* gen pl   */ ых, их, их,  их,      ых,      их,  их,

    // stem types: 1,  2,  3,   4,       5,       6,   7
    /* dat masc */ у,  ю,  ому, ему_ому, ему_ому, ему, ему,
    /* dat n    */ у,  ю,  ому, ему_ому, ему_ому, ему, ему,
    /* dat fem  */ ой, ей, ой,  ей_ой,   ей_ой,   ей,  ей,
    /* dat pl   */ ым, им, им,  им,      ым,      им,  им,

    // stem types: 1,   2,   3,   4,   5,   6,   7
    /* acc masc */ acc, acc, acc, acc, acc, acc, acc,
    /* acc n    */ acc, acc, acc, acc, acc, acc, acc,
    /* acc fem  */ у,   ю,   у,   у,   у,   ю,   ю,
    /* acc pl   */ acc, acc, acc, acc, acc, acc, acc,

    // stem types: 1,   2,   3,   4,     5,     6,   7
    /* ins masc */ ым,  им,  им,  им,    ым,    им,  им,
    /* ins n    */ ым,  им,  им,  им,    ым,    им,  им,
    /* ins fem  */ ой,  ей,  ой,  ей_ой, ей_ой, ей,  ей,
    /* ins pl   */ ыми, ими, ими, ими,   ыми,   ими, ими,

    // stem types: 1,  2,     3,  4,     5,     6,     7
    /* prp masc */ ом, ем_ём, ом, ем_ом, ем_ом, ем_ём, ем_ём,
    /* prp n    */ ом, ем_ём, ом, ем_ом, ем_ом, ем_ём, ем_ём,
    /* prp fem  */ ой, ей,    ой, ей_ой, ей_ой, ей,    ей,
    /* prp pl   */ ых, их,    их, их,    ых,    их,    их,
];

macro_rules! define_endings {
    ($($ident:ident)*) => ($(
        const $ident: (u8, u8) = find_ending_indices(stringify!($ident));
    )*);
    ($($x:ident($a:ident, $b:ident)),* $(,)?) => ($(
        const $x: (u8, u8) = ($a.0, $b.0);
    )*);
}

define_endings! {
    о е ов ы ей й ё ём ой ёй а ам ами и я ям ями ем у ю ах ях ом ев ёв ь ью // nouns
    ое его ого ые ее ий ая ие ему ую юю яя ый ых ым ыми их ому им ими // adjectives, pronouns
}
define_endings! {
    // nouns
    е_ё(е, ё), е_о(е, о), и_е(и, е),
    ев_ёв(ев, ёв), ев_ов(ев, ов),
    ем_ём(ем, ём), ем_ом(ем, ом),
    ей_ёй(ей, ёй), ей_ой(ей, ой),
    ь_ей(ь, ей), null_ей(null, ей),
    // adjectives, pronouns
    ее_ое(ее, ое), ый_ой(ый, ой), ий_ой(ий, ой),
    его_ого(его, ого), ему_ому(ему, ому),
}

const acc: (u8, u8) = (0x00, 0x00);
const null: (u8, u8) = (0x01, 0x01);

const fn find_ending_indices(s: &str) -> (u8, u8) {
    let start = slice_find(ENDINGS, s.as_bytes()).unwrap();
    let encoded = (((s.len() >> 1) << 6) | (start >> 1)) as u8;
    (encoded, encoded)
}
const fn get_ending_by_index(index: u8) -> &'static str {
    unsafe {
        let index = index as usize;
        let ptr = ENDINGS.as_ptr().add((index & 0x3F) << 1);
        let slice = std::slice::from_raw_parts(ptr, (index >> 6) << 1);
        str::from_utf8_unchecked(slice)
    }
}

impl NounDeclension {
    pub const fn get_ending(self, info: DeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.lookup(info, info.case());

        if un_str == 0 {
            let case = info.animacy().acc_case();
            (un_str, str) = self.lookup(info, case);
        }

        let stressed = un_str == str || self.stress.is_ending_stressed(info);
        get_ending_by_index(if stressed { str } else { un_str })
    }
    const fn lookup(self, info: DeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 2 + info.number() as usize;
        x = x * 3 + info.gender() as usize;
        x = x * 8 + (self.stem_type as usize - 1);
        NOUN_LOOKUP[x]
    }
}

impl AdjectiveDeclension {
    pub const fn get_ending(self, info: DeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.lookup(info, info.case());

        if un_str == 0 {
            let case = info.animacy().acc_case();
            (un_str, str) = self.lookup(info, case);
        }

        let stressed = un_str == str || self.stress.full.is_ending_stressed();
        get_ending_by_index(if stressed { str } else { un_str })
    }
    const fn lookup(self, info: DeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        x = x * 7 + (self.stem_type as usize - 1);
        ADJ_LOOKUP[x]
    }
}

impl PronounDeclension {
    pub const fn get_ending(self, info: DeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.lookup(info, info.case);

        if un_str == 0 {
            let case = info.animacy.acc_case();
            (un_str, str) = self.lookup(info, case);
        }

        let stressed = un_str == str || self.stress.is_ending_stressed(info);
        get_ending_by_index(if stressed { str } else { un_str })
    }
    const fn lookup(self, info: DeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        x = x * 7 + (self.stem_type as usize - 1);
        PRO_LOOKUP[x]
    }
}
