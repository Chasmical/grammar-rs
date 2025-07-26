#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![feature(const_from)]
#![feature(const_cmp)]
#![feature(const_try)]
#![feature(const_index)]
#![feature(const_option_ops)]
#![feature(core_intrinsics)]
#![feature(const_eval_select)]
#![cfg_attr(test, feature(test))]
// Fix issues with alphabet::letters::*
#![allow(confusable_idents, non_upper_case_globals, internal_features)]

pub mod categories;
pub mod declension;
pub mod stress;

mod alphabet;
mod inflection_buffer;
mod util;

pub use alphabet::*;
pub use inflection_buffer::*;
