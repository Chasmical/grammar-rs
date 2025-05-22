use std::fmt::Display;

use crate::{InflectionBuffer, categories::*, declension::*, letters};

pub struct Noun<'a> {
    stem: &'a str,
    declension: Option<Declension>,
    gender_animacy: GenderExAndAnimacy,
    tantum: Option<Number>,
    exceptions: &'a [(CaseExAndNumber, &'a str)],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounDeclInfo {
    pub(crate) case_number: CaseAndNumber,
    pub(crate) gender_animacy: GenderAndAnimacy,
}

impl<'a> Noun<'a> {
    pub fn inflect(
        &self,
        f: &mut std::fmt::Formatter,
        case_number: CaseExAndNumber,
    ) -> std::fmt::Result {
        if let Some(str) = self.exceptions.iter().find(|x| x.0 == case_number) {
            return str.1.fmt(f);
        }
        if let Some(decl) = self.declension {
            let mut case_number = case_number.normalize();
            let gender_animacy = self.gender_animacy.normalize();

            if let Some(tantum) = self.tantum {
                case_number = case_number.case().with_num(tantum);
            }

            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Noun(decl) => {
                    decl.inflect(&mut buf, NounDeclInfo {
                        case_number,
                        gender_animacy,
                    });
                },
                Declension::Adjective(decl) => {
                    decl.inflect(&mut buf, AdjectiveDeclInfo {
                        case_number,
                        info: gender_animacy.with_num(case_number.number()),
                    });
                },
                Declension::Pronoun(_) => todo!(), // TODO
            };

            return buf.as_str().fmt(f);
        } else {
            return self.stem.fmt(f);
        }
    }
}

impl const HasCase for NounDeclInfo {
    fn case(&self) -> Case {
        self.case_number.case()
    }
}
impl const HasNumber for NounDeclInfo {
    fn number(&self) -> Number {
        self.case_number.number()
    }
}
impl const HasGender for NounDeclInfo {
    fn gender(&self) -> Gender {
        self.gender_animacy.gender()
    }
}
impl const HasAnimacy for NounDeclInfo {
    fn animacy(&self) -> Animacy {
        self.gender_animacy.animacy()
    }
}

impl NounDeclension {
    pub fn inflect(self, buf: &mut InflectionBuffer, info: NounDeclInfo) {
        buf.append_to_ending(self.get_ending(info));

        if self.flags.has_circle() {
            self.apply_unique_alternation(buf, info);
        }

        todo!() // TODO
    }

    pub fn apply_unique_alternation(self, buf: &mut InflectionBuffer, info: NounDeclInfo) {
        use letters::*;

        match buf.stem_mut() {
            // -ин (-[ая]нин)
            [.., и, н] => {
                if info.is_plural() {
                    // Shrink by 4 bytes (2 chars), removing 'ин'
                    buf.shrink_stem_by(4);

                    // Nominative - ending 'е', genitive - ending '', other - no changes
                    if let Some(is_nominative) = info.case().acc_is_nom(info) {
                        buf.replace_ending(match is_nominative {
                            // Don't override if (1) flag already did (господин - господа)
                            true if !self.flags.has_circled_one() => "е",
                            false => "",
                            _ => return,
                        });
                    }
                }
            },
            // -[оё]нок
            #[rustfmt::skip]
            [.., yo @ _, n @ н, о, к] => {
                if info.is_plural() {
                    // Transform '-[оё]нок' into '-[ая]та'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = match *yo { о => а, ё => я, _ => todo!() }; // TODO
                    // Set the stem char after '[ая]' to 'т'
                    *n = т;
                    // Shrink by 4 bytes (2 chars), removing 'ок'
                    buf.shrink_stem_by(4);

                    // Nominative - ending 'а', genitive - ending '', other - no changes
                    if let Some(is_nominative) = info.case().acc_is_nom(info) {
                        buf.replace_ending(if is_nominative { "а" } else { "" });
                    }
                } else {
                    // Remove the last vowel for non-nominative cases ('о', pre-last char)
                    if !info.case().is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -ок
            [.., preceding @ _, o @ о, k @ к] => {
                if info.is_plural() {
                    // Transform '-ок' into '-[ая]т'

                    // If preceded by a sibilant, replace 'о' with 'а'; otherwise, with 'я'
                    *o = if preceding.is_sibilant() { а } else { я };
                    // Set the last stem char to 'т'
                    *k = т;

                    // Nominative - ending 'а', genitive - ending '', other - no changes
                    if let Some(is_nominative) = info.case().acc_is_nom(info) {
                        buf.replace_ending(if is_nominative { "а" } else { "" });
                    }
                } else {
                    // Remove the last vowel for non-nominative cases ('о', pre-last char)
                    if !info.case().is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -[оё]ночек
            #[rustfmt::skip]
            [.., yo @ _, n @ н, o @ о, ч, е, к] => {
                if info.is_plural() {
                    // Transform '-[оё]ночек' into '-[ая]тки'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = match *yo { о => а, ё => я, _ => todo!() }; // TODO
                    // Set the stem chars after '[оё]' to 'тк'
                    (*n, *o) = (т, к);
                    // Shrink by 6 bytes (3 chars), removing 'чек'
                    buf.shrink_stem_by(6);
                } else {
                    // Remove the last vowel for non-nominative cases ('е', pre-last char)
                    if !info.case().is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -мя
            [.., м] if matches!(info.gender(), Gender::Neuter) => {
                todo!() // TODO
            },
            _ => {
                todo!() // TODO
            },
        };
    }
}
