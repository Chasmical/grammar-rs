use crate::{InflectionBuffer, categories::*, declension::*, letters, stress::*};
use std::fmt::Display;

pub struct Noun<'a> {
    pub stem: &'a str,
    pub info: NounInfo,
    // exceptions: &'a [(CaseExAndNumber, &'a str)],
}
pub struct NounInfo {
    pub declension: Option<Declension>,
    pub gender: GenderEx,
    pub animacy: Animacy,
    pub tantum: Option<Number>,
}

impl<'a> Noun<'a> {
    pub fn inflect(
        &self,
        case: CaseEx,
        number: Number,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        // TODO: check exceptions

        if let Some(decl) = self.info.declension {
            let (case, number) = case.normalize_with(number);
            let number = self.info.tantum.unwrap_or(number);

            let mut info = DeclInfo {
                case,
                number,
                gender: self.info.gender.normalize(),
                animacy: self.info.animacy,
            };

            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Noun(decl) => {
                    if let Some(gender_animacy) = decl.override_gender {
                        info.gender = gender_animacy.gender();
                        info.animacy = gender_animacy.animacy();
                    }
                    decl.inflect(info, &mut buf)
                },
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Pronoun(_) => todo!(), // TODO
            };

            buf.as_str().fmt(f)
        } else {
            self.stem.fmt(f)
        }
    }
}

impl NounDeclension {
    pub fn inflect(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        buf.append_to_ending(self.get_ending(info));

        if self.flags.has_circle() {
            self.apply_unique_alternation(info, buf);
        }

        if self.stem_type == NounStemType::Type8
            && matches!(buf.ending(), [letters::я, ..])
            && buf.stem().last().is_some_and(|x| x.is_hissing())
        {
            buf.ending_mut()[0] = letters::а;
        }

        if self.flags.has_star() {
            self.apply_vowel_alternation(info, buf);
        }

        if self.flags.has_alternating_yo() {
            todo!(); // TODO: yo alternation
        }
    }

    pub fn apply_unique_alternation(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        use letters::*;

        match buf.stem_mut() {
            // -ин (-[ая]нин)
            [.., и, н] => {
                if info.is_plural() {
                    // Shrink by 4 bytes (2 chars), removing 'ин'
                    buf.shrink_stem_by(4);

                    // Nominative - ending 'е', genitive - ending '', other - no changes
                    if let Some(is_nominative) = info.case.acc_is_nom(info) {
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
            [.., yo, n @ н, о, к] => {
                if info.is_plural() {
                    // Transform '-[оё]нок' into '-[ая]та'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = match *yo { о => а, ё => я, _ => todo!() }; // TODO
                    // Set the stem char after '[ая]' to 'т'
                    *n = т;
                    // Shrink by 4 bytes (2 chars), removing 'ок'
                    buf.shrink_stem_by(4);

                    // Nominative - ending 'а', genitive - ending '', other - no changes
                    if let Some(is_nominative) = info.case.acc_is_nom(info) {
                        buf.replace_ending(if is_nominative { "а" } else { "" });
                    }
                } else {
                    // Remove the last vowel for non-nominative cases ('о', pre-last char)
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -ок
            [.., preceding, o @ о, k @ к] => {
                if info.is_plural() {
                    // Transform '-ок' into '-[ая]т'

                    // If preceded by a sibilant, replace 'о' with 'а'; otherwise, with 'я'
                    *o = if preceding.is_sibilant() { а } else { я };
                    // Set the last stem char to 'т'
                    *k = т;

                    // Nominative - ending 'а', genitive - ending '', other - no changes
                    if let Some(is_nominative) = info.case.acc_is_nom(info) {
                        buf.replace_ending(if is_nominative { "а" } else { "" });
                    }
                } else {
                    // Remove the last vowel for non-nominative cases ('о', pre-last char)
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -[оё]ночек
            #[rustfmt::skip]
            [.., yo, n @ н, o @ о, ч, е, к] => {
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
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -мя
            [.., м] if matches!(info.gender, Gender::Neuter) => {
                todo!() // TODO
            },
            _ => {
                todo!() // TODO
            },
        };
    }

    pub fn apply_vowel_alternation(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        let gender = info.gender();

        if gender == Gender::Masculine
            || gender == Gender::Feminine && self.stem_type == NounStemType::Type8
        {
            let last_vowel_index = buf.stem().iter().rposition(|x| x.is_vowel()).expect("todo"); // TODO

            if info.is_singular() && info.case.is_nom_or_acc_inan(info)
                || gender == Gender::Feminine && info.case == Case::Instrumental
            {
                return;
            }

            let last_vowel = buf.stem()[last_vowel_index];
            match last_vowel {
                letters::о => {
                    buf.remove_from_stem((last_vowel_index * 2)..((last_vowel_index + 1) * 2));
                },
                letters::е | letters::ё => {
                    let preceding = buf.stem().get(last_vowel_index - 1);

                    if let Some(preceding) = preceding {
                        if preceding.is_vowel() {
                            buf.stem_mut()[last_vowel_index] = letters::й;
                        } else if self.stem_type == NounStemType::Type6
                            || self.stem_type == NounStemType::Type3
                                && preceding.is_non_sibilant_consonant()
                            || *preceding == letters::л
                        {
                            buf.stem_mut()[last_vowel_index] = letters::ь;
                        }
                    } else {
                        buf.remove_from_stem((last_vowel_index * 2)..((last_vowel_index + 1) * 2));
                    }
                },
                _ => todo!(), // TODO
            }
        } else if matches!(gender, Gender::Neuter | Gender::Feminine)
            && info.is_plural()
            && info.case.acc_is_nom(info) == Some(false)
        {
            if self.stem_type == NounStemType::Type2
                && matches!(self.stress, NounStress::B | NounStress::F)
                || self.flags.has_circled_two()
            {
                return;
            }

            if self.stem_type == NounStemType::Type6 && matches!(buf.stem(), [.., letters::ь]) {
                let len = buf.stem().len();
                buf.stem_mut()[len - 1] = match self.stress.is_ending_stressed(info) {
                    true => letters::е,
                    false => letters::и,
                };
                return;
            }

            if gender == Gender::Feminine && matches!(buf.ending(), [letters::ь]) {
                buf.replace_ending("");
            }

            let last_cons_index = buf.stem().iter().rposition(|x| x.is_consonant()).expect("todo"); // TODO

            let last = buf.stem()[last_cons_index];
            let pre_last = buf.stem_mut().get_mut(last_cons_index - 1);

            if let Some(pre_last @ &mut (letters::ь | letters::й)) = pre_last {
                *pre_last = if last != letters::ц && self.stress.is_ending_stressed(info) {
                    letters::ё
                } else {
                    letters::е
                };
                return;
            };

            let pre_last = pre_last.copied();

            if matches!(pre_last, Some(letters::к | letters::г | letters::х))
                || matches!(last, letters::к | letters::г | letters::х)
                    && pre_last.is_some_and(|x| x.is_sibilant())
            {
                buf.insert_between_last_two_stem_letters(letters::о);
                return;
            }

            buf.insert_between_last_two_stem_letters(
                if last != letters::ц && self.stress.is_ending_stressed(info) {
                    if pre_last.is_some_and(|x| x.is_hissing()) { letters::о } else { letters::ё }
                } else {
                    letters::е
                },
            );
        }
    }
}
