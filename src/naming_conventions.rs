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
    where T: ToUpperCamelCaseString,
{
    fn to_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_upper_camel_case_stringified = ToUpperCamelCaseString::to_upper_camel_case_string(self);
        value_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_upper_camel_case_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait ToSnakeCaseString {//todo rename as just snake case and all variable names
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
    where T: ToSnakeCaseString,
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
    where T: ToScreamingSnakeCaseString,
{
    fn to_screaming_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_screaming_snake_case_stringified = ToScreamingSnakeCaseString::to_screaming_snake_case_string(self);
        value_screaming_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_screaming_snake_case_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

////
//todo maybe use struct like struct Parameters<'a>(&'a str) and impl ToUpperCamelCaseString for it ?
fn parameters_stringified() -> &'static str {
    "parameters"
}
fn parameters_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&parameters_stringified())
}
// fn parameters_snake_case_stringified() -> std::string::String {
//     ToSnakeCaseString::to_snake_case_string(&parameters_stringified())
// }
fn payload_stringified() -> &'static str {
    "payload"
}
fn payload_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&payload_stringified())
}
fn payload_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&payload_stringified())
}
fn element_stringified() -> &'static str {
    "element"
}
fn element_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&element_stringified())
}
fn element_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&element_stringified())
}
fn with_serialize_deserialize_stringified() -> &'static str {
    "with_serialize_deserialize"
}
fn with_serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&with_serialize_deserialize_stringified())
}
fn with_serialize_deserialize_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&with_serialize_deserialize_stringified())
}
fn try_stringified() -> &'static str {
    "try"
}
fn try_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&try_stringified())
}
fn try_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&try_stringified())
}
fn from_stringified() -> &'static str {
    "From"
}
fn from_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&from_stringified())
}
fn from_snake_case_stringified() -> std::string::String {
    ToSnakeCaseString::to_snake_case_string(&from_stringified())
}
fn response_variants_stringified() -> &'static str {
    "response_variants"
}
fn response_variants_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&response_variants_stringified())
}
// fn response_variants_snake_case_stringified() -> std::string::String {
//     ToSnakeCaseString::to_snake_case_string(&response_variants_stringified())
// }
fn error_stringified() -> &'static str {
    "error"
}
fn error_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&error_stringified())
}
// fn error_snake_case_stringified() -> std::string::String {
//     ToSnakeCaseString::to_snake_case_string(&error_stringified())
// }
fn named_stringified() -> &'static str {
    "named"
}
fn named_upper_camel_case_stringified() -> std::string::String {
    ToUpperCamelCaseString::to_upper_camel_case_string(&named_stringified())
}
// fn named_snake_case_stringified() -> std::string::String {
//     ToSnakeCaseString::to_snake_case_string(&named_stringified())
// }
///

pub trait ParametersUpperCamelCaseTokenStream {
    fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> ParametersUpperCamelCaseTokenStream for T 
    where T: ToUpperCamelCaseString
{
    fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_string(),
            parameters_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadUpperCamelCaseTokenStream {
    fn payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadUpperCamelCaseTokenStream for T 
    where T: ToUpperCamelCaseString
{
    fn payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn payload_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T 
    where T: ToUpperCamelCaseString
{
    fn payload_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
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

pub trait PayloadTryFromPayloadWithSerializeDeserializeString {
    fn payload_try_from_payload_with_serialize_deserialize_string(&self) -> std::string::String;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeString for T 
    where T: ToUpperCamelCaseString
{
    fn payload_try_from_payload_with_serialize_deserialize_string(&self) -> std::string::String {
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

pub trait PayloadTryFromPayloadWithSerializeDeserializeTokenStream {
    fn payload_try_from_payload_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeTokenStream for T 
    where T: PayloadTryFromPayloadWithSerializeDeserializeString
{
    fn payload_try_from_payload_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_try_from_payload_with_serialize_deserialize_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeSnakeCaseString {
    fn payload_try_from_payload_with_serialize_deserialize_snake_case_string(&self) -> std::string::String;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeSnakeCaseString for T 
    where T: ToSnakeCaseString
{
    fn payload_try_from_payload_with_serialize_deserialize_snake_case_string(&self) -> std::string::String {
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

pub trait PayloadTryFromPayloadWithSerializeDeserializeSnakeCaseTokenStream {
    fn payload_try_from_payload_with_serialize_deserialize_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeSnakeCaseTokenStream for T 
    where T: PayloadTryFromPayloadWithSerializeDeserializeSnakeCaseString
{
    fn payload_try_from_payload_with_serialize_deserialize_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_try_from_payload_with_serialize_deserialize_snake_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TryOperationSnakeCaseString {
    fn try_operation_snake_case_string(&self) -> std::string::String;
}

impl<T> TryOperationSnakeCaseString for T 
    where T: ToSnakeCaseString
{
    fn try_operation_snake_case_string(&self) -> std::string::String {
        format!(
            "{}_{}",
            try_snake_case_stringified(),
            self.to_snake_case_string()
        )
    }
}

pub trait TryOperationSnakeCaseTokenStream {
    fn try_operation_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TryOperationSnakeCaseTokenStream for T 
    where T: TryOperationSnakeCaseString
{
    fn try_operation_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_operation_snake_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TryOperationResponseVariantsUpperCamelCaseString {
    fn try_operation_response_variants_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> TryOperationResponseVariantsUpperCamelCaseString for T 
    where T: ToUpperCamelCaseString
{
    fn try_operation_response_variants_upper_camel_case_string(&self) -> std::string::String {
        format!(
            "{}{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
            response_variants_upper_camel_case_stringified()
        )
    }
}

pub trait TryOperationResponseVariantsUpperCamelCaseTokenStream {
    fn try_operation_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TryOperationResponseVariantsUpperCamelCaseTokenStream for T 
    where T: TryOperationResponseVariantsUpperCamelCaseString
{
    fn try_operation_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_operation_response_variants_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TryOperationUpperCamelCaseString {
    fn try_operation_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> TryOperationUpperCamelCaseString for T 
    where T: ToUpperCamelCaseString
{
    fn try_operation_upper_camel_case_string(&self) -> std::string::String {
        format!(
            "{}{}",
            try_upper_camel_case_stringified(),
            self.to_upper_camel_case_string(),
        )
    }
}

pub trait TryOperationUpperCamelCaseTokenStream {
    fn try_operation_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TryOperationUpperCamelCaseTokenStream for T 
    where T: TryOperationUpperCamelCaseString
{
    fn try_operation_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.try_operation_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadElementUpperCamelCaseString {
    fn payload_element_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> PayloadElementUpperCamelCaseString for T 
    where T: ToUpperCamelCaseString
{
    fn payload_element_upper_camel_case_string(&self) -> std::string::String {
        format!(
            "{}{}{}",
            self.to_upper_camel_case_string(),
            payload_upper_camel_case_stringified(),
            element_upper_camel_case_stringified(),
        )
    }
}

pub trait PayloadElementUpperCamelCaseTokenStream {
    fn payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadElementUpperCamelCaseTokenStream for T 
    where T: PayloadElementUpperCamelCaseString
{
    fn payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_element_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadElementTryFromPayloadElementWithSerializeDeserializeUpperCamelCaseString {
    fn payload_element_try_from_payload_element_with_serialize_deserialize_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> PayloadElementTryFromPayloadElementWithSerializeDeserializeUpperCamelCaseString for T 
    where T: ToUpperCamelCaseString
{
    fn payload_element_try_from_payload_element_with_serialize_deserialize_upper_camel_case_string(&self) -> std::string::String {
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

pub trait PayloadElementTryFromPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn payload_element_try_from_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadElementTryFromPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T 
    where T: PayloadElementTryFromPayloadElementWithSerializeDeserializeUpperCamelCaseString
{
    fn payload_element_try_from_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_element_try_from_payload_element_with_serialize_deserialize_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadElementTryFromPayloadElementWithSerializeDeserializeSnakeCaseString {
    fn payload_element_try_from_payload_element_with_serialize_deserialize_snake_sase_string(&self) -> std::string::String;
}

impl<T> PayloadElementTryFromPayloadElementWithSerializeDeserializeSnakeCaseString for T 
    where T: ToSnakeCaseString
{
    fn payload_element_try_from_payload_element_with_serialize_deserialize_snake_sase_string(&self) -> std::string::String {
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

pub trait PayloadElementTryFromPayloadElementWithSerializeDeserializeSnakeCaseTokenStream {
    fn payload_element_try_from_payload_element_with_serialize_deserialize_snake_sase_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadElementTryFromPayloadElementWithSerializeDeserializeSnakeCaseTokenStream for T 
    where T: PayloadElementTryFromPayloadElementWithSerializeDeserializeSnakeCaseString
{
    fn payload_element_try_from_payload_element_with_serialize_deserialize_snake_sase_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_element_try_from_payload_element_with_serialize_deserialize_snake_sase_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadElementTryFromPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseString {
    fn payload_element_try_from_payload_element_with_serialize_deserialize_error_named_upper_camel_case_string(&self) -> std::string::String;
}

impl<T> PayloadElementTryFromPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseString for T 
    where T: ToUpperCamelCaseString
{
    fn payload_element_try_from_payload_element_with_serialize_deserialize_error_named_upper_camel_case_string(&self) -> std::string::String {
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

pub trait PayloadElementTryFromPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream {
    fn payload_element_try_from_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> PayloadElementTryFromPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream for T 
    where T: PayloadElementTryFromPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseString
{
    fn payload_element_try_from_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = self.payload_element_try_from_payload_element_with_serialize_deserialize_error_named_upper_camel_case_string();
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}