use std::fmt::Display;

use crate::{InflectionBuffer, categories::*, declension::*};

pub struct Adjective<'a> {
    stem: &'a str,
    declension: Option<Declension>,
    exceptions: &'a [(CaseAndNumber, &'a str)],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjectiveDeclInfo {
    pub(crate) case_number: CaseAndNumber,
    pub(crate) info: GenderOrPluralAndAnimacy,
}

impl<'a> Adjective<'a> {
    pub fn inflect(
        &self,
        f: &mut std::fmt::Formatter,
        case_number: CaseAndNumber,
        gender_animacy: GenderAndAnimacy,
    ) -> std::fmt::Result {
        if let Some(str) = self.exceptions.iter().find(|x| x.0 == case_number) {
            return str.1.fmt(f);
        }
        if let Some(decl) = self.declension {
            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Adjective(decl) => {
                    decl.inflect(&mut buf, AdjectiveDeclInfo {
                        case_number,
                        info: gender_animacy.with_num(case_number.number()),
                    });
                },
                Declension::Pronoun(decl) => {
                    decl.inflect(&mut buf, PronounDeclInfo {
                        case_number,
                        info: gender_animacy.with_num(case_number.number()),
                    });
                },
                Declension::Noun(_) => todo!(), // TODO
            };

            return buf.as_str().fmt(f);
        } else {
            return self.stem.fmt(f);
        }
    }
}

impl const HasCase for AdjectiveDeclInfo {
    fn case(&self) -> Case {
        self.case_number.case()
    }
}
impl const HasNumber for AdjectiveDeclInfo {
    fn number(&self) -> Number {
        self.case_number.number()
    }
}
impl const HasAnimacy for AdjectiveDeclInfo {
    fn animacy(&self) -> Animacy {
        self.info.animacy()
    }
}

impl AdjectiveDeclension {
    pub fn inflect(self, buf: &mut InflectionBuffer, info: AdjectiveDeclInfo) {
        buf.append_to_ending(self.get_ending(info));

        todo!() // TODO
    }
}
