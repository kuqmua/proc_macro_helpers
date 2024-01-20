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
    ToUpperCamelCaseString::to_upper_camel_case_string(&NAMED)
}
pub fn named_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&NAMED)
}
const ERROR: &str = "error";
pub fn error_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&ERROR)
}
pub fn error_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&ERROR)
}
const OCCURENCE: &str = "occurence";
pub fn occurence_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&OCCURENCE)
}
pub fn occurence_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&OCCURENCE)
}
const STRING: &str = "string";
pub fn string_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&STRING)
}
pub fn string_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&STRING)
}
const PARAMETERS: &str = "parameters";
pub fn parameters_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&PARAMETERS)
}
// pub fn parameters_snake_case_stringified() -> std::string::String {
//     ToSnakeCaseString::to_snake_case_string(&PARAMETERS)
// }
const PAYLOAD: &str = "payload";
pub fn payload_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&PAYLOAD)
}
pub fn payload_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&PAYLOAD)
}
const ELEMENT: &str = "element";
pub fn element_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&ELEMENT)
}
pub fn element_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&ELEMENT)
}
const TRY: &str = "try";
pub fn try_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&TRY)
}
pub fn try_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&TRY)
}
const FROM: &str = "from";
pub fn from_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&FROM)
}
pub fn from_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&FROM)
}
const RESPONSE_VARIANTS: &str = "response_variants";
pub fn response_variants_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&RESPONSE_VARIANTS)
}
// pub fn response_variants_snake_case_stringified() -> std::string::String {
//     ToSnakeCaseString::to_snake_case_string(&RESPONSE_VARIANTS)
// }
const PATH: &str = "path";
pub fn path_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&PATH)
}
pub fn path_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&PATH)
}
const KEY: &str = "key";
pub fn key_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&KEY)
}
pub fn key_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&KEY)
}
const VALUE: &str = "value";
pub fn value_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&VALUE)
}
pub fn value_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&VALUE)
}
const VEC: &str = "vec";
pub fn vec_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&VEC)
}
pub fn vec_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&VEC)
}
const HASHMAP: &str = "HashMap";
pub fn hashmap_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&HASHMAP)
}
pub fn hashmap_snake_case_stringified() -> std::string::String {
    //naming exception - 
    std::string::String::from("hashmap")
}
const REFERENCE: &str = "reference";
pub fn reference_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&REFERENCE)
}
pub fn reference_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&REFERENCE)
}
const WITH: &str = "with";
pub fn with_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&WITH)
}
pub fn with_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&WITH)
}
const SERIALIZE: &str = "serialize";
pub fn serialize_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&SERIALIZE)
}
pub fn serialize_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&SERIALIZE)
}
const DESERIALIZE: &str = "deserialize";
pub fn deserialize_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&DESERIALIZE)
}
pub fn deserialize_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&DESERIALIZE)
}




const WITH_SERIALIZE_DESERIALIZE: &str = "with_serialize_deserialize";
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


pub const SERIALIZE_DESERIALIZE_UPPER_CAMEL_CASE: &str = "SerializeDeserialize";

// pub fn with_serialize_deserialize_snake_case() -> std::string::String {
//     format!(
//         "{}_{}_{}",
//         with_snake_case_stringified(),
//         serialize_snake_case_stringified(),
//         deserialize_snake_case_stringified()
//     )
// }



pub fn unnamed_upper_camel_case() -> std::string::String {
    format!("Un{}", named_snake_case_stringified())
}
pub fn error_occurence_upper_camel_case() -> std::string::String {
    format!("{}{}", error_upper_camel_case_stringified(), occurence_upper_camel_case_stringified())
}
pub fn error_occurence_snake_case() -> std::string::String {
    crate::naming_conventions::ToSnakeCaseString::to_snake_case_string(&error_occurence_upper_camel_case())
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

//todo maybe add another generic - trait casing. and ToUpperCamelCaseString and others would implement it like .to_case::<UpperCamel>()

pub trait ToUpperCamelCaseString {
    fn to_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> ToUpperCamelCaseString for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_upper_camel_case_string(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::UpperCamel)
    }
}

pub trait ToUpperCamelCaseTokenStream {
    fn to_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> ToUpperCamelCaseTokenStream for T
where
    T: ToUpperCamelCaseString,
{
    fn to_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_upper_camel_case_stringified =
            ToUpperCamelCaseString::to_upper_camel_case_string(self);
        value_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_upper_camel_case_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait ToSnakeCaseString {
    //todo rename as just snake case and all variable names
    fn to_snake_case_string(&self) -> std::string::String;
}

impl<T> ToSnakeCaseString for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_snake_case_string(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::Snake)
    }
}

pub trait ToSnakeCaseTokenStream {
    fn to_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> ToSnakeCaseTokenStream for T
where
    T: ToSnakeCaseString,
{
    fn to_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_snake_case_stringified = ToSnakeCaseString::to_snake_case_string(self);
        value_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_snake_case_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait ToScreamingSnakeCaseString {
    fn to_screaming_snake_case_string(&self) -> std::string::String;
}

impl<T> ToScreamingSnakeCaseString for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_screaming_snake_case_string(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::ScreamingSnake)
    }
}

pub trait ToScreamingSnakeCaseTokenStream {
    fn to_screaming_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> ToScreamingSnakeCaseTokenStream for T
where
    T: ToScreamingSnakeCaseString,
{
    fn to_screaming_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_screaming_snake_case_stringified =
            ToScreamingSnakeCaseString::to_screaming_snake_case_string(self);
        value_screaming_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_screaming_snake_case_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}


pub trait SelfParametersUpperCamelCaseTokenStream {
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfParametersUpperCamelCaseTokenStream for T
where
    T: ToUpperCamelCaseString,
{
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_string(),
            parameters_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadUpperCamelCaseTokenStream {
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadUpperCamelCaseTokenStream for T
where
    T: ToUpperCamelCaseString,
{
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: ToUpperCamelCaseString,
{
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeString {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_string(&self) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeString for T
where
    T: ToUpperCamelCaseString,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_string(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
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
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeString,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_try_from_self_payload_with_serialize_deserialize_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseString {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_string(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseString for T
where
    T: ToSnakeCaseString,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_string(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_string(),
            payload_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_string(),
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
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseString,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified =
            self.self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfSnakeCaseString {
    fn try_self_snake_case_string(&self) -> std::string::String;
}

impl<T> TrySelfSnakeCaseString for T
where
    T: ToSnakeCaseString,
{
    fn try_self_snake_case_string(&self) -> std::string::String {
        format!(
            "{}_{}",
            try_snake_case_stringified(),
            self.to_snake_case_string()
        )
    }
}

pub trait TrySelfSnakeCaseTokenStream {
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfSnakeCaseTokenStream for T
where
    T: TrySelfSnakeCaseString,
{
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_snake_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseString {
    fn try_self_response_variants_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseString for T
where
    T: ToUpperCamelCaseString,
{
    fn try_self_response_variants_upper_camel_case_string(&self) -> std::string::String {
        format!(
            "{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
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
    T: TrySelfResponseVariantsUpperCamelCaseString,
{
    fn try_self_response_variants_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_response_variants_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfUpperCamelCaseString {
    fn try_self_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> TrySelfUpperCamelCaseString for T
where
    T: ToUpperCamelCaseString,
{
    fn try_self_upper_camel_case_string(&self) -> std::string::String {
        format!(
            "{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
        )
    }
}

pub trait TrySelfUpperCamelCaseTokenStream {
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfUpperCamelCaseTokenStream for T
where
    T: TrySelfUpperCamelCaseString,
{
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_self_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementUpperCamelCaseString {
    fn self_payload_element_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> SelfPayloadElementUpperCamelCaseString for T
where
    T: ToUpperCamelCaseString,
{
    fn self_payload_element_upper_camel_case_string(&self) -> std::string::String {
        format!(
            "{}{}{}",
            self.to_upper_camel_case_string(),
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
    T: SelfPayloadElementUpperCamelCaseString,
{
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseString {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_string(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseString for T
where
    T: ToUpperCamelCaseString,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_string(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
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
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseString,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseString {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_string(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseString for T
where
    T: ToSnakeCaseString,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_string(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_string(),
            payload_snake_case_stringified(),
            element_snake_case_stringified(),
            try_snake_case_stringified(),
            from_snake_case_stringified(),
            self.to_snake_case_string(),
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
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseString,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self
            .self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_string(
            );
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseString
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_string(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseString
    for T
where
    T: ToUpperCamelCaseString,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_string(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
            try_upper_camel_case_stringified(),
            from_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
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
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseString,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value_stringified = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
