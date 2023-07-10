pub static NAMED_CAMEL_CASE: &str = "Named";
pub static SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub static SYN_FIELDS: &str = "syn::Fields";
pub static ERROR_OCCURENCE_CASE: &str = "Error";
pub static OCCURENCE_CAMEL_CASE: &str = "Occurence";
pub static VEC_CAMEL_CASE: &str = "Vec";
pub static SERIALIZE_DESERIALIZE_CAMEL_CASE: &str = "SerializeDeserialize";
pub static WITH_CAMEL_CASE: &str = "With";
pub static HASHMAP_CAMEL_CASE: &str = "HashMap";
pub static KEY_CAMEL_CASE: &str = "Key";
pub static VALUE_CAMEL_CASE: &str = "Value";
pub static IS_NONE_STRINGIFIED: &str = "is None";
pub static SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub static PATH_CAMEL_CASE: &str = "Path";
pub static REFERENCE_CAMEL_CASE: &str = "Reference";
pub static STRING_CAMEL_CASE: &str = "String";
pub static SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT: &str =
    "proc_macro_helpers::error_occurence::supported_container::SupportedContainer::";
pub static SUPPORTED_ENUM_VARIANT_STRINGIFIED: &str =
    "proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant";

pub fn named_lower_case() -> std::string::String {
    convert_case::Casing::to_case(&NAMED_CAMEL_CASE, convert_case::Case::Snake).to_lowercase()
}
pub fn unnamed_camel_case() -> std::string::String {
    format!("Un{}", named_lower_case())
}
pub fn with_serialize_deserialize_camel_case() -> std::string::String {
    format!("{}{}", WITH_CAMEL_CASE, SERIALIZE_DESERIALIZE_CAMEL_CASE)
}
pub fn with_serialize_deserialize_lower_case() -> std::string::String {
    convert_case::Casing::to_case(
        &with_serialize_deserialize_camel_case(),
        convert_case::Case::Snake,
    )
    .to_lowercase()
}
pub fn error_occurence_camel_case() -> std::string::String {
    format!("{}{}", ERROR_OCCURENCE_CASE, OCCURENCE_CAMEL_CASE)
}
pub fn error_occurence_lower_case() -> std::string::String {
    convert_case::Casing::to_case(&error_occurence_camel_case(), convert_case::Case::Snake)
        .to_lowercase()
}
pub fn vec_lower_case() -> std::string::String {
    VEC_CAMEL_CASE.to_lowercase()
}
pub fn hashmap_lower_case() -> std::string::String {
    convert_case::Casing::to_case(&HASHMAP_CAMEL_CASE, convert_case::Case::Flat)
}
pub fn key_lower_case() -> std::string::String {
    KEY_CAMEL_CASE.to_lowercase()
}
pub fn value_lower_case() -> std::string::String {
    VALUE_CAMEL_CASE.to_lowercase()
}
pub fn syn_type_path_stringified() -> std::string::String {
    format!("syn::Type::{}", PATH_CAMEL_CASE)
}
pub fn supports_only_supported_container_stringified() -> std::string::String {
    format!(
        "{} {}",
        SUPPORTS_ONLY_STRINGIFIED, SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT
    )
}
