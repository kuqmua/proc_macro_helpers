pub fn generate_with_serialize_deserialize_version(
    supported_enum_variant: &crate::error_occurence::supported_enum_variant::SuportedEnumVariant,
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    with_serialize_deserialize_lower_case: &std::string::String,
    error_occurence_lower_case: &std::string::String,
    vec_lower_case: &std::string::String,
    hashmap_lower_case: &std::string::String,
    key_lower_case: &std::string::String,
    value_lower_case: &std::string::String,
    proc_macro_name_ident_stringified: &std::string::String,
    syn_type_path_stringified: &std::string::String,
    generics_len: usize,
    supports_only_supported_container_stringified: &std::string::String,
    with_serialize_deserialize_camel_case: &std::string::String,
    unnamed_camel_case: &std::string::String,
    ident_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
    optional_additional_named_variant: Option<proc_macro2::TokenStream>,
    implements_this_error: bool,
    //todo pub or not
) -> proc_macro2::TokenStream {
    let this_error_token_stream = match implements_this_error {
        true => quote::quote! { thiserror::Error, },
        false => proc_macro2::TokenStream::new(),
    };
    use convert_case::Casing;
    let token_stream = match supported_enum_variant {
        crate::error_occurence::supported_enum_variant::SuportedEnumVariant::Named => {
            let code_occurence_camel_case = format!(
                "Code{}",
                crate::error_occurence::hardcode::OCCURENCE_CAMEL_CASE
            );
            let code_occurence_lower_case = code_occurence_camel_case
                .to_case(convert_case::Case::Snake)
                .to_lowercase();
            let foreign_type_camel_case = "ForeignType";
            let display_camel_case = "Display";
            let display_foreign_type_camel_case =
                format!("{display_camel_case}{foreign_type_camel_case}");
            let display_foreign_type_lower_case = display_foreign_type_camel_case
                .to_case(convert_case::Case::Snake)
                .to_lowercase();
            let display_lower_case = display_camel_case
                .to_case(convert_case::Case::Snake)
                .to_lowercase();
            let attribute_prefix_stringified = "eo_";
            let attribute_display_stringified =
                format!("{attribute_prefix_stringified}{display_lower_case}");
            let attribute_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_display_foreign_type_stringified =
                format!("{attribute_prefix_stringified}{display_foreign_type_lower_case}");
            let attribute_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_error_occurence_stringified =
                format!("{attribute_prefix_stringified}{error_occurence_lower_case}");
            let attribute_vec_display_stringified =
                format!("{attribute_prefix_stringified}{vec_lower_case}_{display_lower_case}");
            let attribute_vec_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_vec_display_foreign_type_stringified = format!(
                "{attribute_prefix_stringified}{vec_lower_case}_{display_foreign_type_lower_case}"
            );
            let attribute_vec_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_vec_error_occurence_stringified = format!(
                "{attribute_prefix_stringified}{vec_lower_case}_{error_occurence_lower_case}"
            );
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{error_occurence_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{error_occurence_lower_case}");
            let variants_vec = variants.into_iter().map(|variant| {
                let variant_fields_vec = if let syn::Fields::Named(fields_named) = variant.fields {
                    fields_named.named.into_iter().map(|field|{
                        let field_ident = field.ident.unwrap_or_else(|| panic!(
                            "{proc_macro_name_ident_stringified} field.ident {}",
                            crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                        ));
                        let error_or_code_occurence = match field_ident == *code_occurence_lower_case {
                            true => {
                                let (code_occurence_type_stringified, code_occurence_lifetime) = {
                                    if let syn::Type::Path(type_path) = &field.ty {
                                        (
                                            {
                                                let mut code_occurence_type_repeat_checker = false;
                                                let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                                .fold(String::from(""), |mut acc, path_segment| {
                                                    let path_segment_ident = &path_segment.ident;
                                                    match *path_segment_ident == code_occurence_camel_case {
                                                        true => {
                                                            if code_occurence_type_repeat_checker {
                                                                panic!("{proc_macro_name_ident_stringified} code_occurence_ident detected more than one {code_occurence_camel_case} inside type path");
                                                            }
                                                            acc.push_str(&path_segment_ident.to_string());
                                                            code_occurence_type_repeat_checker = true;
                                                        },
                                                        false => acc.push_str(&format!("{path_segment_ident}::")),
                                                    }
                                                    acc
                                                });
                                                if !code_occurence_type_repeat_checker {
                                                    panic!("{proc_macro_name_ident_stringified} no {code_occurence_camel_case} named field");
                                                }
                                                code_occurence_segments_stringified_handle
                                            },
                                            crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                &type_path.path.segments,
                                                proc_macro_name_ident_stringified
                                            ),
                                        )
                                      }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {code_occurence_lower_case} {} {syn_type_path_stringified}",
                                            crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                        );
                                    }
                                };
                                crate::error_occurence::error_or_code_occurence::ErrorOrCodeOccurence::CodeOccurence {
                                    field_type: code_occurence_type_stringified,
                                    vec_lifetime: code_occurence_lifetime
                                }
                            },
                            false => {
                                let attribute = {
                                    let mut option_attribute = None;
                                    field.attrs.iter().for_each(|attr|{
                                        if let true = attr.path.segments.len() == 1 {
                                            let error_message = format!("{proc_macro_name_ident_stringified} two or more supported attributes!");
                                            if let true = attr.path.segments[0].ident == attribute_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence);
                                                }
                                            }//other attributes are not for this proc_macro
                                        }//other attributes are not for this proc_macro
                                    });
                                    option_attribute.unwrap_or_else(|| panic!(
                                        "{proc_macro_name_ident_stringified} option attribute {}",
                                        crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                    ))
                                };
                                let syn_type_reference = format!(
                                    "syn::Type::{}",
                                    crate::error_occurence::hardcode::REFERENCE_CAMEL_CASE
                                );
                                let error_message = format!(
                                    "{} {syn_type_path_stringified} and {syn_type_reference}",
                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                );
                                let supported_container = match field.ty {
                                    syn::Type::Path(type_path) => {
                                        let path = crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments);
                                        let vec_lifetime = crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                            &type_path.path.segments,
                                            &proc_macro_name_ident_stringified
                                        );
                                        let path_segment = type_path.path.segments.into_iter().last()
                                        .unwrap_or_else(|| panic!(
                                            "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().last() {}",
                                            crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                        ));
                                        if path_segment.ident == crate::error_occurence::hardcode::VEC_CAMEL_CASE {
                                            let vec_element_type = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = path_segment.arguments {
                                                if let true = angle_brackets_generic_arguments.args.len() == 1 {
                                                    if let syn::GenericArgument::Type(type_handle) =
                                                        angle_brackets_generic_arguments.args
                                                        .into_iter().next()
                                                        .unwrap_or_else(|| panic!(
                                                            "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.into_iter().nth(0) {}",
                                                            crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                        ))
                                                    {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => crate::error_occurence::vec_element_type::VecElementType::Path{
                                                                element_path: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                                                vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                                    &type_path.path.segments,
                                                                    &proc_macro_name_ident_stringified
                                                                )
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments
                                                                        .into_iter().next()
                                                                        .unwrap_or_else(|| panic!(
                                                                            "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                                            crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                                        ))
                                                                        .ident
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!(
                                                                        "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                                                        crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                                    );
                                                                };
                                                                crate::error_occurence::vec_element_type::VecElementType::Reference {
                                                                    reference_ident,
                                                                    lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                                        "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                                        crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                                    )).ident
                                                                }
                                                            },
                                                            _ => panic!(
                                                                "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference}",
                                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                            ),
                                                        }
                                                    }
                                                    else {
                                                        panic!(
                                                            "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}",
                                                            crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                            crate::error_occurence::hardcode::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                                                        );
                                                    }
                                                }
                                                else {
                                                    panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 1");
                                                }
                                            }
                                            else {
                                                panic!(
                                                    "{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed",
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                );
                                            };
                                            crate::error_occurence::supported_container::SupportedContainer::Vec{
                                                path,
                                                vec_element_type
                                            }
                                        }
                                        else if path_segment.ident == crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE {
                                            let (
                                                hashmap_key_type,
                                                hashmap_value_type
                                            ) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = path_segment.arguments {
                                                if let true = angle_brackets_generic_arguments.args.len() == 2 {
                                                    let (
                                                        key_generic_argument,
                                                        value_generic_argument
                                                    ) = {
                                                        let mut key_generic_argument_option = None;
                                                        let mut value_generic_argument_option = None;
                                                        angle_brackets_generic_arguments.args
                                                        .into_iter()
                                                        .enumerate()
                                                        .for_each(|(index, generic_argument)|{
                                                            match index {
                                                                0 => {
                                                                    key_generic_argument_option = Some(generic_argument);
                                                                }
                                                                1 => {
                                                                    value_generic_argument_option = Some(generic_argument);
                                                                }
                                                                _ => panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() != 2")
                                                            }
                                                        });
                                                        (
                                                            key_generic_argument_option.unwrap_or_else(|| panic!(
                                                                "{proc_macro_name_ident_stringified} key_generic_argument_option {}",
                                                                crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                            )),
                                                            value_generic_argument_option.unwrap_or_else(|| panic!(
                                                                "{proc_macro_name_ident_stringified} value_generic_argument_option {}",
                                                                crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                            ))
                                                        )
                                                    };
                                                    let hashmap_key_type
                                                    = if let syn::GenericArgument::Type(type_handle) =
                                                        key_generic_argument
                                                    {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => {
                                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path{
                                                                    key_segments_stringified: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                                                    key_vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                                        &type_path.path.segments,
                                                                        &proc_macro_name_ident_stringified
                                                                    )
                                                                }
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let key_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments
                                                                        .into_iter().next()
                                                                        .unwrap_or_else(|| panic!(
                                                                            "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                                            crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                                        ))
                                                                        .ident
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!(
                                                                        "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                                                        crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                                    );
                                                                };
                                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                                    key_reference_ident,
                                                                    key_lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                                        "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                                        crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                                    )).ident
                                                                }
                                                            },
                                                            _ => panic!(
                                                                "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference}",
                                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                            ),
                                                        }
                                                    }
                                                    else {
                                                        panic!(
                                                            "{proc_macro_name_ident_stringified} key_generic_argument {} {}",
                                                            crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                            crate::error_occurence::hardcode::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                                                        );
                                                    };
                                                    let hashmap_value_type = if let syn::GenericArgument::Type(type_handle) = value_generic_argument {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => {
                                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path{
                                                                    value_segments_stringified: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                                                    value_vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                                        &type_path.path.segments,
                                                                        &proc_macro_name_ident_stringified
                                                                    )
                                                                }
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let value_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments
                                                                        .into_iter().next()
                                                                        .unwrap_or_else(|| panic!(
                                                                            "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                                            crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                                        ))
                                                                        .ident
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!(
                                                                        "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                                                        crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                                    );
                                                                };
                                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                                    value_reference_ident,
                                                                    value_lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                                        "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                                        crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                                    )).ident
                                                                }
                                                            },
                                                            _ => panic!(
                                                                "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and syn::Type::Reference",
                                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                            ),
                                                        }
                                                    }
                                                    else {
                                                        panic!(
                                                            "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}",
                                                            crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                            crate::error_occurence::hardcode::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                                                        );
                                                    };
                                                    (
                                                        hashmap_key_type,
                                                        hashmap_value_type,
                                                    )
                                                }
                                                else {
                                                    panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 2");
                                                }
                                            }
                                            else {
                                                panic!(
                                                    "{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed",
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                                );
                                            };
                                            crate::error_occurence::supported_container::SupportedContainer::HashMap{
                                                path,
                                                hashmap_key_type,
                                                hashmap_value_type
                                            }
                                        }
                                        else {
                                            crate::error_occurence::supported_container::SupportedContainer::Path{
                                                path,
                                                vec_lifetime,
                                            }
                                        }
                                    },
                                    syn::Type::Reference(type_reference) => {
                                        let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                            if let true = type_path.path.segments.len() == 1 {
                                                type_path.path.segments
                                                .into_iter().next()
                                                .unwrap_or_else(|| panic!(
                                                    "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                    crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                                ))
                                                .ident
                                            }
                                            else {
                                                panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                            }
                                        }
                                        else {
                                            panic!(
                                                "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                            );
                                        };
                                        crate::error_occurence::supported_container::SupportedContainer::Reference{
                                            reference_ident,
                                            lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                crate::error_occurence::hardcode::IS_NONE_STRINGIFIED
                                            )).ident,
                                        }
                                    },
                                    _ => panic!("{proc_macro_name_ident_stringified} {code_occurence_lower_case} {error_message}"),
                                };
                                crate::error_occurence::error_or_code_occurence::ErrorOrCodeOccurence::Error {
                                    attribute,
                                    supported_container,
                                }
                            },
                        };
                        (
                            field_ident,
                            error_or_code_occurence,
                        )
                    })
                    .collect::<Vec<(
                        proc_macro2::Ident,
                        crate::error_occurence::error_or_code_occurence::ErrorOrCodeOccurence
                    )>>()
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} expected fields would be named");
                };
                (
                    variant.ident,
                    variant_fields_vec,
                )
            })
            .collect::<Vec<(
                proc_macro2::Ident,
                 Vec<(
                    proc_macro2::Ident,
                    crate::error_occurence::error_or_code_occurence::ErrorOrCodeOccurence
                )>
            )>>();
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(variants_vec.len());
            let mut should_generate_impl_compile_time_check_error_occurence_members = false;
            variants_vec.into_iter().for_each(|(
                variant_ident,
                fields_vec
            )|{
                let mut enum_fields_logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                fields_vec.into_iter().for_each(|(field_ident, error_or_code_occurence)|{
                    match error_or_code_occurence {
                        crate::error_occurence::error_or_code_occurence::ErrorOrCodeOccurence::Error {
                            attribute,
                            supported_container,
                        } => {
                            let string_lower_case = crate::error_occurence::hardcode::STRING_CAMEL_CASE.to_lowercase();
                            let std_stringified = "std";
                            let std_string_string_stringified = format!(
                                "{std_stringified}::{string_lower_case}::{}",
                                crate::error_occurence::hardcode::STRING_CAMEL_CASE
                            );
                            let std_string_string_token_stream = std_string_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {std_string_string_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let vec_element_type_path_stringified = format!(
                                "crate::error_occurence::vec_element_type::VecElementType::{}",
                                crate::error_occurence::hardcode::PATH_CAMEL_CASE
                            );
                            let as_std_collections_hashmap_key_type_stringified = format!(
                                "as {std_stringified}::collections::{} key type",
                                crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                            );
                            let str_stringified = "str";
                            let string_string_stringified: String = format!(
                                "{string_lower_case}::{}",
                                crate::error_occurence::hardcode::STRING_CAMEL_CASE
                            );
                            let std_string_string_stringified = format!("{std_stringified}::{string_string_stringified}");
                            let must_be_used_with_stringified = "must be used with";
                            let does_not_support_stringified = "does not support";
                            let type_camel_case = "Type";
                            let hashmap_key_type_stringified = format!(
                                "{}{}{type_camel_case}",
                                crate::error_occurence::hardcode::KEY_CAMEL_CASE,
                                crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                            );
                            let hashmap_value_type_stringified = format!(
                                "{}{}{type_camel_case}",
                                crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE,
                                crate::error_occurence::hardcode::VALUE_CAMEL_CASE
                            );
                            let hashmap_key_type_path_stringified = format!(
                                "{hashmap_key_type_stringified}::{}",
                                crate::error_occurence::hardcode::PATH_CAMEL_CASE
                            );
                            let hashmap_key_type_reference_stringified = format!(
                                "{hashmap_key_type_stringified}::{}",
                                crate::error_occurence::hardcode::REFERENCE_CAMEL_CASE
                            );
                            let hashmap_value_type_path_stringified = format!(
                                "{hashmap_value_type_stringified}::{}",
                                crate::error_occurence::hardcode::PATH_CAMEL_CASE
                            );
                            let hashmap_value_type_reference_stringified = format!(
                                "{hashmap_value_type_stringified}::{}",
                                crate::error_occurence::hardcode::REFERENCE_CAMEL_CASE
                            );
                            let inform_use_str_string_in_different_attribute = |
                                path: String,
                                wrong_attribute: &String,
                                attribute_to_use: &String
                            | {
                                let wrong_attribute_view = crate::error_occurence::attribute_view::attribute_view(wrong_attribute);
                                let attribute_to_use_view = crate::error_occurence::attribute_view::attribute_view(attribute_to_use);
                                //maybe additional cases exists
                                if path == str_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {str_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == std_string_string_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {std_string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == string_string_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == crate::error_occurence::hardcode::STRING_CAMEL_CASE {
                                    panic!(
                                        "{proc_macro_name_ident_stringified} {wrong_attribute_view} {} {must_be_used_with_stringified} {attribute_to_use_view}",
                                        crate::error_occurence::hardcode::STRING_CAMEL_CASE
                                    );
                                }
                            };
                            let
                                field_type_with_serialize_deserialize_token_stream
                            = match attribute {
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplay => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime: _vec_lifetime } = supported_container {
                                        inform_use_str_string_in_different_attribute(
                                            path,
                                            &attribute.to_str().to_string(),
                                            &attribute_display_with_serialize_deserialize_stringified
                                        );
                                        quote::quote! {
                                            #std_string_string_token_stream
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {} {}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                            crate::error_occurence::hardcode::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                                            crate::error_occurence::hardcode::PATH_CAMEL_CASE
                                        )
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize => {
                                    match supported_container {
                                        crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } => {
                                            {
                                                let type_stringified = format!("{path}{}",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            }
                                        },
                                        crate::error_occurence::supported_container::SupportedContainer::Reference{ reference_ident, lifetime_ident } => {
                                            crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                &reference_ident,
                                                str_stringified,
                                                &proc_macro_name_ident_stringified,
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                &attribute
                                            );
                                            crate::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                lifetime_ident.to_string(),
                                                &mut lifetimes_for_serialize_deserialize
                                            );
                                            quote::quote!{#std_string_string_token_stream}
                                        },
                                        _ => panic!(
                                            "{proc_macro_name_ident_stringified} {} only supports {}{} and {}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                                            crate::error_occurence::hardcode::PATH_CAMEL_CASE,
                                            crate::error_occurence::hardcode::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                                            crate::error_occurence::hardcode::REFERENCE_CAMEL_CASE
                                        ),
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignType => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Path { path: _path, vec_lifetime: _vec_lifetime } = supported_container {}
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::PATH_CAMEL_CASE
                                        );
                                    }
                                    quote::quote! {
                                        #std_string_string_token_stream
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } = supported_container {
                                        {
                                            let type_stringified = format!("{path}{}",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::PATH_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    if let crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime: _vec_lifetime } = supported_container {
                                        {
                                            let type_stringified = format!("{path}{with_serialize_deserialize_camel_case}");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::PATH_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplay => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime: _vec_lifetime } = vec_element_type {
                                            inform_use_str_string_in_different_attribute(
                                                element_path,
                                                &attribute.to_str().to_string(),
                                                &attribute_vec_display_with_serialize_deserialize_stringified
                                            );
                                            let type_stringified = format!("{path}<{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                        }
                                        else {
                                            panic!(
                                                "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                                                attribute.attribute_view(),
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                            );
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::VEC_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        match vec_element_type {
                                            crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } => {
                                                let type_stringified = format!("{path}<{element_path}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            crate::error_occurence::vec_element_type::VecElementType::Reference { reference_ident, lifetime_ident: _lifetime_ident } => {
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!("{path}<{std_string_string_stringified}>");
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::VEC_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignType => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                                        path: _path,
                                        vec_element_type
                                    } = supported_container {
                                        if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path: _element_path, vec_lifetime: _vec_lifetime } = vec_element_type {}
                                        else {
                                            panic!(
                                                "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                                                attribute.attribute_view(),
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                            );
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::VEC_CAMEL_CASE
                                        );
                                    }
                                    quote::quote! {
                                        std::vec::Vec<#std_string_string_token_stream>
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type {
                                            {
                                                let type_stringified = format!("{path}<{element_path}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            }
                                        }
                                        else {
                                            panic!(
                                                "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                                                attribute.attribute_view(),
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                            );
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::VEC_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type  {
                                            {
                                                let type_stringified = format!("{path}<{element_path}{with_serialize_deserialize_camel_case}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            }
                                        }
                                        else {
                                            panic!(
                                                "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                                                attribute.attribute_view(),
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                                            );
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::VEC_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => {
                                    let type_token_stream = if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type,
                                    } = supported_container {
                                        let hashmap_key_type_path_case = |
                                            key_segments_stringified: String,
                                            key_vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
                                        | -> proc_macro2::TokenStream {
                                            crate::error_occurence::panic_if_not_string::panic_if_not_string(
                                                &key_segments_stringified,
                                                &std_string_string_stringified,
                                                &proc_macro_name_ident_stringified,
                                                crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                &as_std_collections_hashmap_key_type_stringified,
                                                &attribute
                                            );
                                            {
                                                let type_stringified = format!(
                                                    "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
                                                     crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                );
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            }
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_str().to_string(),
                                                    &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_path_case(
                                                    key_segments_stringified,
                                                    key_vec_lifetime,
                                                )
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_str().to_string(),
                                                    &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified
                                                );
                                                {
                                                    let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    };
                                    type_token_stream
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {value_segments_stringified}{}>",
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified}, {value_segments_stringified}{}>",
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        let hashmap_key_type_path_case = |
                                            key_segments_stringified: String,
                                            key_vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
                                        | -> proc_macro2::TokenStream {
                                            {
                                                let type_stringified = format!(
                                                    "{path}<{key_segments_stringified}{},{std_string_string_stringified}>",
                                                     crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime)
                                                );
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            }
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => hashmap_key_type_path_case(
                                                key_segments_stringified,
                                                key_vec_lifetime,
                                            ),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                let type_stringified = format!(
                                                    "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
                                                );
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{},{value_segments_stringified}{}>",
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                             crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                         crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                   );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => {
                                    let type_token_stream = if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        let hashmap_key_type_path_case = || -> proc_macro2::TokenStream {
                                            let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_str().to_string(),
                                                    &attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_path_case()
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                 },
                                                crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    };
                                    type_token_stream
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                let type_stringified = format!(
                                                    "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                     crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                );
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => {
                                                crate::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                }
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime:  _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                let type_stringified = format!(
                                                    "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                     crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
                                                );
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                let type_stringified = format!(
                                                    "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                     crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                );
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_key_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view(),
                                            crate::error_occurence::hardcode::HASHMAP_CAMEL_CASE
                                        );
                                    }
                                },
                            };
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #field_type_with_serialize_deserialize_token_stream
                            });
                        },
                        crate::error_occurence::error_or_code_occurence::ErrorOrCodeOccurence::CodeOccurence {
                            field_type,
                            vec_lifetime: _vec_lifetime,
                         } => {
                            let code_occurence_type_with_serialize_deserialize_token_stream = {
                                let code_occurence_type_with_serialize_deserialize_stringified =
                                format!("{field_type}{with_serialize_deserialize_camel_case}");
                                code_occurence_type_with_serialize_deserialize_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {code_occurence_type_with_serialize_deserialize_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #code_occurence_type_with_serialize_deserialize_token_stream
                            });
                        },
                    }
                });
                let enum_fields_logic_for_enum_with_serialize_deserialize_iter = enum_fields_logic_for_enum_with_serialize_deserialize.iter();
                logic_for_enum_with_serialize_deserialize.push(quote::quote! {
                    #variant_ident {
                        #(#enum_fields_logic_for_enum_with_serialize_deserialize_iter),*
                    }
                });
            });
            let logic_for_enum_with_serialize_deserialize_iter =
                logic_for_enum_with_serialize_deserialize.iter();
            let additional_variant_token_stream = match optional_additional_named_variant {
                Some(additional_variant_token_stream) => {
                    quote::quote! { #additional_variant_token_stream,}
                }
                None => quote::quote! {},
            };
            quote::quote! {
                #[derive(Debug, #this_error_token_stream serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_serialize_deserialize_token_stream {
                    #additional_variant_token_stream
                    #(#logic_for_enum_with_serialize_deserialize_iter),*
                }
            }
        }
        crate::error_occurence::supported_enum_variant::SuportedEnumVariant::Unnamed => {
            let variants_len = variants.len();
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(variants_len);
            variants.iter().for_each(|variant|{
                let variant_ident = &variant.ident;
                let field_type = if let syn::Fields::Unnamed(fields_unnamed) = &variant.fields {
                    let unnamed = &fields_unnamed.unnamed;
                    if let false = unnamed.len() == 1 {
                        panic!(
                            "{proc_macro_name_ident_stringified} {}::{unnamed_camel_case} variant fields unnamed len != 1",
                            crate::error_occurence::hardcode::SUPPORTED_ENUM_VARIANT_STRINGIFIED
                        );
                    }
                    &unnamed[0].ty
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {}::{unnamed_camel_case}",
                        crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED,
                        crate::error_occurence::hardcode::SYN_FIELDS
                    );
                };
                let type_token_stream = if let syn::Type::Path(type_path) = field_type {
                    let type_stringified = format!(
                        "{}{with_serialize_deserialize_camel_case}",
                        crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                    );
                    type_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {syn_type_path_stringified}",
                        crate::error_occurence::hardcode::SUPPORTS_ONLY_STRINGIFIED
                    )
                };
                logic_for_enum_with_serialize_deserialize.push({
                    quote::quote!{
                        #variant_ident(#type_token_stream)
                    }
                });
            });
            let logic_for_enum_with_serialize_deserialize_generated =
                logic_for_enum_with_serialize_deserialize.iter();
            quote::quote! {
                #[derive(Debug, #this_error_token_stream serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_serialize_deserialize_token_stream {
                    #(#logic_for_enum_with_serialize_deserialize_generated),*
                }
            }
        }
    };
    // println!("{token_stream}");
    token_stream
}
