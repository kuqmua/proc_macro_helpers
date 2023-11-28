#[allow(clippy::enum_variant_names)]
// #[derive(strum_macros::EnumIter, strum_macros::Display)]
pub enum NamedAttribute {
    EoDisplay,
    EoDisplayWithSerializeDeserialize,
    EoDisplayForeignType,
    EoDisplayForeignTypeWithSerializeDeserialize,
    EoErrorOccurence,
    EoVecDisplay,
    EoVecDisplayWithSerializeDeserialize,
    EoVecDisplayForeignType,
    EoVecDisplayForeignTypeWithSerializeDeserialize,
    EoVecErrorOccurence,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize,
    EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence,
    EoHashMapKeyDisplayForeignTypeValueDisplay,
    EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignType,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize,
    EoHashMapKeyDisplayForeignTypeValueErrorOccurence,
}

impl std::string::ToString for NamedAttribute {
    fn to_string(&self) -> std::string::String {
        match self {
            NamedAttribute::EoDisplay => std::string::String::from("eo_display"),
            NamedAttribute::EoDisplayWithSerializeDeserialize => std::string::String::from("eo_display_with_serialize_deserialize"),
            NamedAttribute::EoDisplayForeignType => std::string::String::from("eo_display_foreign_type"),
            NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_display_foreign_type_with_serialize_deserialize"),
            NamedAttribute::EoErrorOccurence => std::string::String::from("eo_error_occurence"),
            NamedAttribute::EoVecDisplay => std::string::String::from("eo_vec_display"),
            NamedAttribute::EoVecDisplayWithSerializeDeserialize => std::string::String::from("eo_vec_display_with_serialize_deserialize"),
            NamedAttribute::EoVecDisplayForeignType => std::string::String::from("eo_vec_display_foreign_type"),
            NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_vec_display_foreign_type_with_serialize_deserialize"),
            NamedAttribute::EoVecErrorOccurence => std::string::String::from("eo_vec_error_occurence"),
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display"),
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize"),
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type"),
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize"),
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => std::string::String::from("eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence"),
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display"),
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize"),
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display_foreign_type"),
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => std::string::String::from("eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize"),
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => std::string::String::from("eo_hashmap_key_display_foreign_type_value_error_occurence"),
        }
    }
}

impl NamedAttribute {
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = format!("#[{}]", self.to_string());
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
    pub fn attribute_view_stringified(&self) -> String {
        self.to_string()
    }
}

pub fn attribute_view(attribute: &String) -> String {
    format!("attribute #[{attribute}]")
}
