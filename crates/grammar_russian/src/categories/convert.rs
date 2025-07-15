use super::{Case, CaseEx, Gender, GenderAnimacy, GenderEx, GenderExAnimacy};
use crate::util::*;
use thiserror::Error;

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error(
    "case must be one of the main 6: nominative, genitive, dative, accusative, instrumental or prepositional"
)]
pub struct CaseError;

enum_conversion!(Case => CaseEx [<= CaseError] {
    Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional,
});

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("gender must be one of the main 3: masculine, neuter or feminine")]
pub struct GenderError;

enum_conversion!(Gender => GenderEx [<= GenderError] {
    Masculine, Neuter, Feminine,
});

enum_conversion!(GenderAnimacy => GenderExAnimacy [<= GenderError] {
    MasculineInanimate, MasculineAnimate,
    NeuterInanimate, NeuterAnimate,
    FeminineInanimate, FeminineAnimate,
});
