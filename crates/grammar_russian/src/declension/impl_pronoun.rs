use std::fmt::Display;

use crate::{InflectionBuffer, declension::*};

pub struct Pronoun<'a> {
    stem: &'a str,
    declension: Option<Declension>,
    // exceptions: &'a [(CaseAndNumber, &'a str)],
}

impl<'a> Pronoun<'a> {
    pub fn inflect(&self, f: &mut std::fmt::Formatter, info: DeclInfo) -> std::fmt::Result {
        // TODO: check exceptions

        if let Some(decl) = self.declension {
            let mut buf = InflectionBuffer::from_stem_unchecked(self.stem);

            match decl {
                Declension::Pronoun(decl) => decl.inflect(&mut buf, info),
                Declension::Adjective(decl) => decl.inflect(&mut buf, info),
                Declension::Noun(_) => todo!(), // TODO
            };

            return buf.as_str().fmt(f);
        } else {
            return self.stem.fmt(f);
        }
    }
}

impl PronounDeclension {
    pub fn inflect(self, buf: &mut InflectionBuffer, info: DeclInfo) {
        buf.append_to_ending(self.get_ending(info));

        todo!() // TODO
    }
}
