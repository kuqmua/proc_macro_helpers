#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod error_occurence;
pub mod attribute;
pub mod generate_quotes;
pub mod type_variants_from_request_response;
pub mod write_token_stream_into_file;
pub mod get_macro_attribute;
pub mod naming_conventions;

pub fn generate_function_name_upper_camel_case_token_stream(
    proc_macro_name_stringified: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = crate::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&proc_macro_name_stringified);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn generate_function_name_snake_case_token_stream(
    proc_macro_name_upper_camel_case_stringified: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = crate::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&proc_macro_name_upper_camel_case_stringified);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn trait_path_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {proc_macro_helpers::naming_conventions}
}
pub fn std_string_string_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{std::string::String}
}


/////


// impl OperationHttpMethod {
//     fn to_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value_snake_case_stringified = proc_macro_helpers::naming_conventions::ToSnakeCase::to_snake_case(self);
//         value_snake_case_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

// trait ParametersUpperCamelCaseTokenStream {
//     fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }

// impl<Generic> ParametersUpperCamelCaseTokenStream for Generic 
//     where Generic: proc_macro_helpers::naming_conventions::ToUpperCamelCase
// {
//     fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = format!(
//             "{}Parameters",
//             self.to_upper_camel_case()
//         );
//         value.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }