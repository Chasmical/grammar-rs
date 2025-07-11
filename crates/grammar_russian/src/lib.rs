#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(const_eval_select)]
#![feature(formatting_options)]
#![cfg_attr(test, feature(test))]
// Fix issues with alphabet::letters::*
#![allow(confusable_idents, non_upper_case_globals, internal_features)]

mod util;

pub mod categories;
pub mod declension;
pub mod stress;

mod alphabet;
mod inflection_buffer;

pub use alphabet::*;
pub use inflection_buffer::*;
