#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod generate_supported_enum_variant;
pub mod generate_with_serialize_deserialize_version;
pub mod global_variables;
pub mod panic_location;
