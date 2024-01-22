// todo maybe use struct like struct Parameters<'a>(&'a str) and impl ToUpperCamelCaseString for it ?
// todo maybe reqwrite it this way
// trait InnerValue<'a> {
//     fn inner_value() -> &'a str;
// }

// pub struct Named;
// impl<'a> InnerValue<'a> for Named {
//     fn inner_value() -> &'a str {
//         "named"
//     }
// }
// impl ToUpperCamelCaseString for Named {
//     fn to_upper_camel_case_string(&self) -> std::string::String {
//         convert_case::Casing::to_case(&Self::inner_value(), convert_case::Case::UpperCamel)
//     }
// }
pub const SUPPORTS_ONLY_STRINGIFIED: &str = "supports only";
pub const SYN_FIELDS: &str = "syn::Fields";
pub const SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT: &str =
    "proc_macro_helpers::error_occurence::supported_container::SupportedContainer::";
pub const SUPPORTED_ENUM_VARIANT_STRINGIFIED: &str =
    "proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant";
pub const SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED: &str = "syn::GenericArgument::Type";
pub const IS_NONE_STRINGIFIED: &str = "is None";
pub const STD_STRINGIFIED: &str = "std";

const NAMED: &str = "named";
pub fn named_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&NAMED)
}
pub fn named_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&NAMED)
}
const ERROR: &str = "error";
pub fn error_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ERROR)
}
pub fn error_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ERROR)
}
const OCCURENCE: &str = "occurence";
pub fn occurence_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&OCCURENCE)
}
pub fn occurence_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&OCCURENCE)
}
const STRING: &str = "string";
pub fn string_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&STRING)
}
pub fn string_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&STRING)
}
const PARAMETERS: &str = "parameters";
pub fn parameters_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&PARAMETERS)
}
// pub fn parameters_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&PARAMETERS)
// }
const PAYLOAD: &str = "payload";
pub fn payload_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&PAYLOAD)
}
pub fn payload_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&PAYLOAD)
}
const ELEMENT: &str = "element";
pub fn element_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&ELEMENT)
}
pub fn element_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ELEMENT)
}
const TRY: &str = "try";
pub fn try_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&TRY)
}
pub fn try_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&TRY)
}
const FROM: &str = "from";
pub fn from_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&FROM)
}
pub fn from_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&FROM)
}
const RESPONSE_VARIANTS: &str = "response_variants";
pub fn response_variants_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&RESPONSE_VARIANTS)
}
// pub fn response_variants_snake_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&RESPONSE_VARIANTS)
// }
const PATH: &str = "path";
pub fn path_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&PATH)
}
pub fn path_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&PATH)
}
const KEY: &str = "key";
pub fn key_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&KEY)
}
pub fn key_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&KEY)
}
const VALUE: &str = "value";
pub fn value_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&VALUE)
}
pub fn value_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&VALUE)
}
const VEC: &str = "vec";
pub fn vec_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&VEC)
}
pub fn vec_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&VEC)
}
const HASHMAP: &str = "HashMap";
pub fn hashmap_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&HASHMAP)
}
pub fn hashmap_snake_case_stringified() -> std::string::String {
    //naming exception - 
    std::string::String::from("hashmap")
}
const REFERENCE: &str = "reference";
pub fn reference_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&REFERENCE)
}
pub fn reference_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&REFERENCE)
}
const WITH: &str = "with";
pub fn with_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&WITH)
}
pub fn with_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&WITH)
}
const SERIALIZE: &str = "serialize";
pub fn serialize_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&SERIALIZE)
}
pub fn serialize_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&SERIALIZE)
}
const DESERIALIZE: &str = "deserialize";
pub fn deserialize_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&DESERIALIZE)
}
pub fn deserialize_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&DESERIALIZE)
}
const REQUEST: &str = "request";
pub fn request_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&REQUEST)
}
pub fn request_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&REQUEST)
}
const RESPONSE: &str = "response";
pub fn response_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&RESPONSE)
}
pub fn response_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&RESPONSE)
}
const VARIANTS: &str = "variants";
pub fn variants_upper_camel_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&VARIANTS)
}
pub fn variants_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&VARIANTS)
}
pub const TVFRR_EXTRACTION_LOGIC: &str = "tvfrr_extraction_logic";
// pub fn tvfrr_extraction_logic_upper_camel_case_stringified() -> std::string::String {
//     proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&TVFRR_EXTRACTION_LOGIC)
// }
pub fn tvfrr_extraction_logic_snake_case_stringified() -> std::string::String {
    proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&TVFRR_EXTRACTION_LOGIC)
}



pub fn serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        serialize_upper_camel_case_stringified(),
        deserialize_upper_camel_case_stringified()
    )
}
pub fn serialize_deserialize_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        serialize_snake_case_stringified(),
        deserialize_snake_case_stringified()
    )
}
pub fn with_serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        with_upper_camel_case_stringified(),
        serialize_upper_camel_case_stringified(),
        deserialize_upper_camel_case_stringified()
    )
}
pub fn with_serialize_deserialize_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        with_snake_case_stringified(),
        serialize_snake_case_stringified(),
        deserialize_snake_case_stringified()
    )
}
pub fn unnamed_upper_camel_case_stringified() -> std::string::String {
    format!("Un{}", named_snake_case_stringified())
}
pub fn error_occurence_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}", 
        error_upper_camel_case_stringified(), 
        occurence_upper_camel_case_stringified()
    )
}
pub fn error_occurence_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}", 
        error_snake_case_stringified(), 
        occurence_snake_case_stringified()
    )
}
pub fn syn_type_path_stringified() -> std::string::String {
    format!("syn::Type::{}", path_upper_camel_case_stringified())
}
pub fn supports_only_supported_container_stringified() -> std::string::String {
    format!(
        "{} {}",
        SUPPORTS_ONLY_STRINGIFIED, SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT
    )
}

pub trait SelfParametersUpperCamelCaseTokenStream {
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfParametersUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_stringified(),
            parameters_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadUpperCamelCaseTokenStream {
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeStringified {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeTokenStream {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeTokenStream for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_try_from_self_payload_with_serialize_deserialize_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified()
        )
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified =
            self.self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfSnakeCaseStringified {
    fn try_self_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}",
            try_snake_case_stringified(),
            self.to_snake_case_stringified()
        )
    }
}

pub trait TrySelfSnakeCaseTokenStream {
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfSnakeCaseTokenStream for T
where
    T: TrySelfSnakeCaseStringified,
{
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseStringified {
    fn try_self_response_variants_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            response_variants_upper_camel_case_stringified()
        )
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseTokenStream {
    fn try_self_response_variants_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseTokenStream for T
where
    T: TrySelfResponseVariantsUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_response_variants_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfUpperCamelCaseStringified {
    fn try_self_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfUpperCamelCaseTokenStream {
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfUpperCamelCaseTokenStream for T
where
    T: TrySelfUpperCamelCaseStringified,
{
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            with_upper_camel_case_stringified(),
            serialize_upper_camel_case_stringified(),
            deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementUpperCamelCaseStringified {
    fn self_payload_element_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadElementUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementUpperCamelCaseTokenStream {
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementUpperCamelCaseStringified,
{
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            element_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_stringified(),
            payload_snake_case_stringified(),
            element_snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self
            .self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
            );
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T>
    SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfErrorNamedUpperCamelCaseTokenStream
{
    fn try_self_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T>
    TrySelfErrorNamedUpperCamelCaseTokenStream
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = format!(
            "{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
        );
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRequestErrorUpperCamelCaseStringified {
    fn try_self_request_error_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> TrySelfRequestErrorUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            request_upper_camel_case_stringified(),
            error_upper_camel_case_stringified()
        )
    }
}

pub trait TrySelfRequestErrorUpperCamelCaseTokenStream {
    fn try_self_request_error_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRequestErrorUpperCamelCaseTokenStream for T
where
    T: TrySelfRequestErrorUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_request_error_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified {
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeStringified,
{
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}",
            self.self_payload_try_from_self_payload_with_serialize_deserialize_stringified(),
            error_upper_camel_case_stringified(),
            named_upper_camel_case_stringified()
        )
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream {
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream for T
where
    T: PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified,
{
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TryOperationWithSerializeDeserializeUpperCamelCaseStringified {
    fn try_operation_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> TryOperationWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_operation_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            with_upper_camel_case_stringified(),
            serialize_upper_camel_case_stringified(),
            deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait TryOperationWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn try_operation_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TryOperationWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: TryOperationWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn try_operation_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.try_operation_with_serialize_deserialize_upper_camel_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsDesirableAttributeStringified {
    fn try_self_response_variants_desirable_attribute_stringified(
        &self,
        attribute: &crate::attribute::Attribute
    ) -> std::string::String;
}

impl<T> TrySelfResponseVariantsDesirableAttributeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_desirable_attribute_stringified(
        &self,
        attribute: &crate::attribute::Attribute
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            response_upper_camel_case_stringified(),
            variants_upper_camel_case_stringified(),
            proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(attribute),
        )
    }
}

pub trait TrySelfResponseVariantsDesirableAttributeTokenStream {
    fn try_self_response_variants_desirable_attribute_token_stream(
        &self,
        attribute: &crate::attribute::Attribute
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsDesirableAttributeTokenStream for T
where
    T: TrySelfResponseVariantsDesirableAttributeStringified,
{
    fn try_self_response_variants_desirable_attribute_token_stream(
        &self,
        attribute: &crate::attribute::Attribute
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_response_variants_desirable_attribute_stringified(attribute);
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TryOperationWithSerializeDeserializeStringified {
    fn try_operation_with_serialize_deserialize_stringified(&self) -> std::string::String;
}

impl<T> TryOperationWithSerializeDeserializeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_operation_with_serialize_deserialize_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            with_upper_camel_case_stringified(),
            serialize_upper_camel_case_stringified(),
            deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait TryOperationWithSerializeDeserializeTokenStream {
    fn try_operation_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TryOperationWithSerializeDeserializeTokenStream for T
where
    T: TryOperationWithSerializeDeserializeStringified,
{
    fn try_operation_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_operation_with_serialize_deserialize_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TvfrrExtractionLogicTrySelfSnakeCaseStringified {
    fn tvfrr_extraction_logic_try_self_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TvfrrExtractionLogicTrySelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn tvfrr_extraction_logic_try_self_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}_{}",
            tvfrr_extraction_logic_snake_case_stringified(),
            try_snake_case_stringified(),
            self.to_snake_case_stringified()
        )
    }
}

pub trait TvfrrExtractionLogicTrySelfSnakeCaseTokenStream {
    fn tvfrr_extraction_logic_try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TvfrrExtractionLogicTrySelfSnakeCaseTokenStream for T
where
    T: TvfrrExtractionLogicTrySelfSnakeCaseStringified,
{
    fn tvfrr_extraction_logic_try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.tvfrr_extraction_logic_try_self_snake_case_stringified();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}