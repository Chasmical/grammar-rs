use std::fmt::Display;

use crate::{InflectionBuffer, declension::*};

pub struct Pronoun<'a> {
    pub stem: &'a str,
    pub info: PronounInfo,
    // exceptions: &'a [(CaseAndNumber, &'a str)],
}
pub struct PronounInfo {
    pub declension: Option<Declension>,
}

impl<'a> Pronoun<'a> {
    pub fn inflect(&self, info: DeclInfo, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: check exceptions

        if let Some(decl) = self.info.declension {
            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Pronoun(decl) => decl.inflect(info, &mut buf),
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Noun(_) => todo!(), // TODO
            };

            buf.as_str().fmt(f)
        } else {
            self.stem.fmt(f)
        }
    }
}

impl PronounDeclension {
    pub fn inflect(self, info: DeclInfo, buf: &mut InflectionBuffer) {
        buf.append_to_ending(self.get_ending(info));

        todo!() // TODO
    }
}
