pub const NAMED_UPPER_CAMEL_CASE: &str = "Named";
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const ERROR_OCCURENCE_CASE: &str = "Error";
pub const OCCURENCE_UPPER_CAMEL_CASE: &str = "Occurence";
pub const VEC_UPPER_CAMEL_CASE: &str = "Vec";
pub const SERIALIZE_DESERIALIZE_UPPER_CAMEL_CASE: &str = "SerializeDeserialize";
pub const WITH_UPPER_CAMEL_CASE: &str = "With";
pub const HASHMAP_UPPER_CAMEL_CASE: &str = "HashMap";
pub const KEY_UPPER_CAMEL_CASE: &str = "Key";
pub const VALUE_UPPER_CAMEL_CASE: &str = "Value";
pub const IS_NONE_STRINGIFIED: &str = "is None";
pub const SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub const PATH_UPPER_CAMEL_CASE: &str = "Path";
pub const REFERENCE_UPPER_CAMEL_CASE: &str = "Reference";
pub const STRING_UPPER_CAMEL_CASE: &str = "String";
pub const STRING_SNAKE_CASE: &str = "string";
pub const SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT: &str =
    "proc_macro_helpers::error_occurence::supported_container::SupportedContainer::";
pub const SUPPORTED_ENUM_VARIANT_STRINGIFIED: &str =
    "proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant";
pub const STD_STRINGIFIED: &str = "std";

pub fn named_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(&NAMED_UPPER_CAMEL_CASE)
}
pub fn unnamed_upper_camel_case() -> std::string::String {
    format!("Un{}", named_snake_case())
}
pub fn with_serialize_deserialize_upper_camel_case() -> std::string::String {
    format!("{}{}", WITH_UPPER_CAMEL_CASE, SERIALIZE_DESERIALIZE_UPPER_CAMEL_CASE)
}
pub fn with_serialize_deserialize_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(
        &with_serialize_deserialize_upper_camel_case(),
    )
}
pub fn error_occurence_upper_camel_case() -> std::string::String {
    format!("{}{}", ERROR_OCCURENCE_CASE, OCCURENCE_UPPER_CAMEL_CASE)
}
pub fn error_occurence_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(&error_occurence_upper_camel_case())
}
pub fn vec_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(&VEC_UPPER_CAMEL_CASE)
}
pub fn hashmap_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(&HASHMAP_UPPER_CAMEL_CASE)
}
pub fn key_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(&KEY_UPPER_CAMEL_CASE)
}
pub fn value_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCase::to_snake_case(&VALUE_UPPER_CAMEL_CASE)
}
pub fn syn_type_path_stringified() -> std::string::String {
    format!("syn::Type::{}", PATH_UPPER_CAMEL_CASE)
}
pub fn supports_only_supported_container_stringified() -> std::string::String {
    format!(
        "{} {}",
        SUPPORTS_ONLY_STRINGIFIED, SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT
    )
}
