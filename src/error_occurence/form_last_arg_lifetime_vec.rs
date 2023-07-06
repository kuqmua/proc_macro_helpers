pub fn form_last_arg_lifetime_vec(
    segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>,
    proc_macro_name_ident_stringified: &String,
    supports_only_stringified: &str,
    is_none_stringified: &str,
    syn_generic_argument_type_stringified: &str,
) -> Vec<crate::error_occurence::lifetime::Lifetime> {
    if let Some(path_segment) = segments.last() {
        match &path_segment.arguments {
            syn::PathArguments::None => Vec::new(),
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => {
                angle_bracketed_generic_argument.args.iter().map(|generic_argument|{
                    match generic_argument {
                        syn::GenericArgument::Lifetime(lfmt) => crate::error_occurence::lifetime::Lifetime::Specified(lfmt.ident.to_string()),
                        syn::GenericArgument::Type(_) => crate::error_occurence::lifetime::Lifetime::NotSpecified,
                        _ => panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] {supports_only_stringified} syn::GenericArgument::Lifetime and {syn_generic_argument_type_stringified}")
                    }
                })
                .collect()
            },
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        }
    } else {
        panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() {is_none_stringified}");
    }
}
