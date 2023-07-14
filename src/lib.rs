#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod error_occurence;
pub mod global_variables;
pub mod panic_location;
pub mod to_lower_snake_case;
