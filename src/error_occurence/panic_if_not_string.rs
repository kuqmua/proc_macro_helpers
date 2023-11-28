pub fn panic_if_not_string(
    segments_stringified: &String,
    std_string_string_stringified: &String,
    proc_macro_name_ident_stringified: &String,
    supports_only_stringified: &str,
    as_std_collections_hashmap_key_type_stringified: &String,
    attribute: &crate::error_occurence::named_attribute::NamedAttribute,
) {
    if let false = segments_stringified == std_string_string_stringified {
        panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {std_string_string_stringified} {as_std_collections_hashmap_key_type_stringified} (hashmap key must be string for deserialization)", attribute.attribute_view_stringified());
    }
}
