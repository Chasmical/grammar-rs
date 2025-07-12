use crate::{InflectionBuffer, declension::*};
use std::fmt::Display;

pub struct Adjective<'a> {
    pub stem: &'a str,
    pub info: AdjectiveInfo,
    // exceptions: &'a [(CaseAndNumber, &'a str)],
}
pub struct AdjectiveInfo {
    pub declension: Option<Declension>,
    pub is_reflexive: bool,
}

impl<'a> Adjective<'a> {
    pub fn inflect(&self, info: DeclInfo, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: check exceptions

        if let Some(decl) = self.info.declension {
            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Pronoun(decl) => decl.inflect(info, &mut buf),
                Declension::Noun(_) => todo!(), // TODO
            };

            if self.info.is_reflexive {
                buf.append_to_ending("ся");
            }

            buf.as_str().fmt(f)
        } else {
            self.stem.fmt(f)
        }
    }
}

impl AdjectiveDeclension {
    pub fn inflect(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        buf.append_to_ending(self.get_ending(info));

        todo!() // TODO
    }
}
