pub enum SuportedEnumVariant {
    Named,
    Unnamed,
}

pub fn create_supported_enum_variant(
    data_enum: &syn::DataEnum,
    proc_macro_name_ident_stringified: std::string::String,
    supports_only_stringified: &str,
    syn_fields: &str,
    named_camel_case: &str,
    unnamed_camel_case: std::string::String,
) -> SuportedEnumVariant {
    let mut all_equal: Option<SuportedEnumVariant> = None;
    if let true = &data_enum.variants.is_empty() {
        panic!("{proc_macro_name_ident_stringified} enum variants are empty");
    }
    let error_message = format!("{proc_macro_name_ident_stringified} {supports_only_stringified} enums where all variants are {syn_fields}::{named_camel_case} or all variants are {syn_fields}::{unnamed_camel_case}");
    data_enum
        .variants
        .iter()
        .for_each(|variant| match &variant.fields {
            syn::Fields::Named(_) => match &all_equal {
                Some(supported_variant) => {
                    if let SuportedEnumVariant::Unnamed = supported_variant {
                        panic!("{error_message}");
                    }
                }
                None => {
                    all_equal = Some(SuportedEnumVariant::Named);
                }
            },
            syn::Fields::Unnamed(_) => match &all_equal {
                Some(supported_variant) => {
                    if let SuportedEnumVariant::Named = supported_variant {
                        panic!("{error_message}");
                    }
                }
                None => {
                    all_equal = Some(SuportedEnumVariant::Unnamed);
                }
            },
            syn::Fields::Unit => panic!("{error_message}"),
        });
    if let Some(supported_enum_variant) = all_equal {
        supported_enum_variant
    } else {
        panic!("{proc_macro_name_ident_stringified} {supports_only_stringified} with enums where all variants are named or unnamed");
    }
}
