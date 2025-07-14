use crate::{InflectionBuffer, Letter, categories::*, declension::*, letters, stress::*};
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
            let number = self.info.tantum.unwrap_or(number);
            let (case, number) = case.normalize_with(number);

            let info = DeclInfo {
                case,
                number,
                gender: self.info.gender.normalize(),
                animacy: self.info.animacy,
            };

            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Noun(decl) => decl.inflect(info, &mut buf),
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Pronoun(_) => {
                    unimplemented!("Nouns don't decline by pronoun declension")
                },
            };

            buf.as_str().fmt(f)
        } else {
            self.stem.fmt(f)
        }
    }
}

impl NounDeclension {
    pub fn inflect(self, mut info: DeclInfo, buf: &mut InflectionBuffer) {
        if let Some(gender_animacy) = self.override_gender {
            info.gender = gender_animacy.gender();
            info.animacy = gender_animacy.animacy();
        }

        buf.append_to_ending(self.get_ending(info));

        if self.flags.has_circle() {
            self.apply_unique_alternation(info, buf);
        }

        // Special case for stem type 8: replace 'я' with 'а' after hissing consonant in stem
        if self.stem_type == NounStemType::Type8
            && buf.stem().last().is_some_and(|x| x.is_hissing())
            && let [ya @ letters::я, ..] = buf.ending_mut()
        {
            *ya = letters::а;
        }

        if self.flags.has_star() {
            self.apply_vowel_alternation(info, buf);
        }
        if self.flags.has_alternating_yo() {
            self.apply_ye_yo_alternation(info, buf);
        }
    }

    pub fn apply_unique_alternation(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        use letters as lt;

        match buf.stem_mut() {
            // -ин (боярин, крестьянин, землянин, господин)
            [.., lt::и, lt::н] => {
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
            // -[оё]нок (утёнок, ребёнок, опёнок, мышонок, зайчонок)
            [.., yo @ (lt::о | lt::ё), n @ lt::н, lt::о, lt::к] => {
                if info.is_plural() {
                    // Transform '-[оё]нок' into '-[ая]т'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = if matches!(*yo, lt::о) { lt::а } else { lt::я };
                    // Set the stem char after '[ая]' to 'т'
                    *n = lt::т;
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
            // -ок (щенок, внучок)
            [.., preceding, o @ lt::о, k @ lt::к] => {
                if info.is_plural() {
                    // Transform '-ок' into '-[ая]т'

                    // If preceded by a sibilant, replace 'о' with 'а'; otherwise, with 'я'
                    *o = if preceding.is_sibilant() { lt::а } else { lt::я };
                    // Set the last stem char to 'т'
                    *k = lt::т;

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
            // -[оё]ночек (телёночек, котёночек, мышоночек)
            [.., yo @ (lt::о | lt::ё), n @ lt::н, o @ lt::о, lt::ч, lt::е, lt::к] => {
                if info.is_plural() {
                    // Transform '-[оё]ночек' into '-[ая]тк'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = if matches!(*yo, lt::о) { lt::а } else { lt::я };
                    // Set the stem chars after '[ая]' to 'тк'
                    (*n, *o) = (lt::т, lt::к);
                    // Shrink by 6 bytes (3 chars), removing 'чек'
                    buf.shrink_stem_by(6);
                } else {
                    // Remove the last vowel for non-nominative cases ('е', pre-last char)
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -очек (щеночек, внучочек)
            [.., preceding, o @ lt::о, ch @ lt::ч, ye @ lt::е, lt::к] => {
                if info.is_plural() {
                    // Transform '-очек' into '-[ая]тк'

                    // If preceded by a sibilant, replace 'о' with 'а'; otherwise, with 'я'
                    *o = if preceding.is_sibilant() { lt::а } else { lt::я };
                    // Set the stem chars after '[ая]' to 'тк'
                    (*ch, *ye) = (lt::т, lt::к);
                    // Shrink by 2 bytes (1 char), removing 'к'
                    buf.shrink_stem_by(2);
                } else {
                    // Remove the last vowel for non-nominative cases ('е', pre-last char)
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_from_stem((buf.stem_len - 4)..(buf.stem_len - 2));
                    }
                }
            },
            // -м(я) (время, знамя, пламя, имя)
            [.., lt::м] if matches!(info.gender, Gender::Neuter) => {
                if info.is_plural() && !info.case.is_nom_or_acc_inan(info) {
                    // The е/ё alternation is handled here, instead of in apply_ye_yo_alternation()
                    let use_yo = info.is_plural()
                        && self.flags.has_alternating_yo()
                        && info.case.is_gen_or_acc_an(info);

                    // Append '[её]н' to the stem
                    buf.append_to_stem(if use_yo { "ён" } else { "ен" });
                }
                // Replace nominative ending 'ь' with 'я'
                if info.is_singular()
                    && let [ending @ letters::ь] = buf.ending_mut()
                {
                    *ending = letters::я;
                }
            },
            _ => {
                unimplemented!("Unknown unique stem alternation")
            },
        };
    }

    pub fn apply_vowel_alternation(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        let gender = info.gender();

        if gender == Gender::Masculine
            || gender == Gender::Feminine && self.stem_type == NounStemType::Type8
        {
            let Some(last_vowel_index) = buf.stem().iter().rposition(|x| x.is_vowel()) else {
                unimplemented!("No vowels found in stem for vowel alternation")
            };

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
                _ => {
                    unimplemented!("Unknown vowel alternation in stem")
                },
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

            if self.stem_type == NounStemType::Type6
                && let [.., last @ letters::ь] = buf.stem_mut()
            {
                *last = match self.stress.is_ending_stressed(info) {
                    true => letters::е,
                    false => letters::и,
                };
                return;
            }

            if gender == Gender::Feminine && matches!(buf.ending(), [letters::ь]) {
                buf.replace_ending("");
            }

            let Some(last_cons_index) = buf.stem().iter().rposition(|x| x.is_consonant()) else {
                unimplemented!("No consonants found in stem for vowel alternation")
            };

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

    pub fn apply_ye_yo_alternation(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        // The е/ё alternation is handled in apply_unique_alternation()
        if self.flags.has_circle() {
            return;
        }

        // If there's a 'ё' in the stem, check if it keeps its stress
        if let Some(yo) = buf.stem_mut().iter_mut().find(|x| matches!(**x, letters::ё)) {
            // SAFETY: yo is not modified until right before return
            let yo: &mut Letter = unsafe { std::mem::transmute(yo) };

            // If stress falls on the ending, unstress the 'ё' in stem to just 'е'
            if self.stress.is_ending_stressed(info) && buf.ending().iter().any(|x| x.is_vowel()) {
                *yo = letters::е;
            }
        } else {
            let mut search_stem = buf.stem_mut();

            // If there was vowel alternation, ignore the last two letters that may have been affected by it
            if self.flags.has_star()
                && let [new_search_stem @ .., _, _] = search_stem
            {
                search_stem = new_search_stem;
            }

            // Find the LAST unstressed 'е' in stem
            let Some(ye) = search_stem.iter_mut().rfind(|x| matches!(**x, letters::е)) else {
                unimplemented!("е/ё not found in е/ё alternation")
            };
            // SAFETY: ye is not modified until right before return
            let ye: &mut Letter = unsafe { std::mem::transmute(ye) };

            // If the ending doesn't have any vowels (can't receive stress), stress 'е' in stem into 'ё'
            if !buf.ending().iter().any(|x| x.is_vowel()) {
                *ye = letters::ё;
            } else if self.stress.is_stem_stressed(info) {
                // Special case for f/f′/f″: 'е' in stem can only receive stress in first vowel position
                // E.g. exceptions: железа (1f, ё) - же́лезы, середа (1f′, ё) - се́реды
                if matches!(self.stress, NounStress::F | NounStress::Fp | NounStress::Fpp) {
                    if buf.stem().iter().find(|x| x.is_vowel()).is_some_and(|x| std::ptr::eq(ye, x))
                    {
                        *ye = letters::ё;
                    }
                } else {
                    // Otherwise, 'е' is always stressed into 'ё'
                    *ye = letters::ё;
                }
            }
        }
    }
}
