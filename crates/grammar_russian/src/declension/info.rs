use crate::categories::{Animacy, Case, Gender, HasAnimacy, HasCase, HasGender, HasNumber, Number};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DeclInfo {
    pub case: Case,
    pub number: Number,
    pub gender: Gender,
    pub animacy: Animacy,
}

impl const HasCase for DeclInfo {
    fn case(&self) -> Case {
        self.case
    }
}
impl const HasNumber for DeclInfo {
    fn number(&self) -> Number {
        self.number
    }
}
impl const HasGender for DeclInfo {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const HasAnimacy for DeclInfo {
    fn animacy(&self) -> Animacy {
        self.animacy
    }
}
