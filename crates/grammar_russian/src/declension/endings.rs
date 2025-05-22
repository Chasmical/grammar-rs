use crate::categories::{Case, HasAnimacy, HasCase, HasGender, HasNumber};

use crate::declension::{endings_data::*, *};

impl NounDeclension {
    pub const fn get_ending(self, info: NounDeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.lookup(info, info.case());

        if un_str == 0 {
            let case = info.animacy().acc_case();
            (un_str, str) = self.lookup(info, case);
        }

        let stressed = un_str == str || self.stress.is_ending_stressed(info, info);
        get_ending(if stressed { str } else { un_str })
    }
    const fn lookup(self, info: NounDeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 2 + info.number() as usize;
        x = x * 3 + info.gender() as usize;
        x = x * 8 + (self.stem_type as usize - 1);
        NOUN_LOOKUP[x]
    }
}

impl AdjectiveDeclension {
    pub const fn get_ending(self, info: AdjectiveDeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.lookup(info, info.case());

        if un_str == 0 {
            let case = info.animacy().acc_case();
            (un_str, str) = self.lookup(info, case);
        }

        let stressed = un_str == str || self.stress.full.is_ending_stressed();
        get_ending(if stressed { str } else { un_str })
    }
    const fn lookup(self, info: AdjectiveDeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 4 + info.info as usize;
        x = x * 7 + (self.stem_type as usize - 1);
        ADJ_LOOKUP[x]
    }
}

impl PronounDeclension {
    pub const fn get_ending(self, info: PronounDeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.lookup(info, info.case());

        if un_str == 0 {
            let case = info.animacy().acc_case();
            (un_str, str) = self.lookup(info, case);
        }

        let stressed = un_str == str || self.stress.is_ending_stressed(info, info);
        get_ending(if stressed { str } else { un_str })
    }
    const fn lookup(self, info: PronounDeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 4 + info.info.gender_or_plural() as usize;
        x = x * 7 + (self.stem_type as usize - 1);
        PRO_LOOKUP[x]
    }
}
