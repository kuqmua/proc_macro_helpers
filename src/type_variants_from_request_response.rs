pub static STATUS_CODES_CHECKER: &str = "StatusCodesChecker";

pub fn generate_enum_status_codes_checker_name_token_stream(
    ident: &syn::Ident,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let enum_status_codes_checker_name_stringified = format!("{ident}{STATUS_CODES_CHECKER}");
    enum_status_codes_checker_name_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {enum_status_codes_checker_name_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

pub fn generate_tvfrr_extraction_logic_token_stream(
    ident_lower_case_stringified: &std::string::String,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let tvfrr_extraction_logic_stringified = format!("tvfrr_extraction_logic_{ident_lower_case_stringified}");
    tvfrr_extraction_logic_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {tvfrr_extraction_logic_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}