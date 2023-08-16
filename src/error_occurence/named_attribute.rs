#[allow(clippy::enum_variant_names)]
#[derive(strum_macros::EnumIter, strum_macros::Display)]
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

impl NamedAttribute {
    pub fn to_str(&self) -> &str {
        match self {
            NamedAttribute::EoDisplay => "eo_display",
            NamedAttribute::EoDisplayWithSerializeDeserialize => "eo_display_with_serialize_deserialize",
            NamedAttribute::EoDisplayForeignType => "eo_display_foreign_type",
            NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize => "eo_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoErrorOccurence => "eo_error_occurence",
            NamedAttribute::EoVecDisplay => "eo_vec_display",
            NamedAttribute::EoVecDisplayWithSerializeDeserialize => "eo_vec_display_with_serialize_deserialize",
            NamedAttribute::EoVecDisplayForeignType => "eo_vec_display_foreign_type",
            NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize => "eo_vec_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoVecErrorOccurence => "eo_vec_error_occurence",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => "eo_hashmap_key_display_with_serialize_deserialize_value_display",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => "eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => "eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => "eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => "eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => "eo_hashmap_key_display_foreign_type_value_display",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => "eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => "eo_hashmap_key_display_foreign_type_value_display_foreign_type",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => "eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => "eo_hashmap_key_display_foreign_type_value_error_occurence",
        }
    }
    pub fn attribute_view(&self) -> String {
        self.to_str().to_string()
    }
}
