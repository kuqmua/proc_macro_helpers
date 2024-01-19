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
        let value_upper_camel_case_stringified = crate::naming_conventions::ToUpperCamelCaseString::to_upper_camel_case_string(self);
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
        let value_snake_case_stringified = crate::naming_conventions::ToSnakeCaseString::to_snake_case_string(self);
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
        let value_screaming_snake_case_stringified = crate::naming_conventions::ToScreamingSnakeCaseString::to_screaming_snake_case_string(self);
        value_screaming_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_screaming_snake_case_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

////

pub trait ParametersUpperCamelCaseTokenStream {
    fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<Generic> ParametersUpperCamelCaseTokenStream for Generic 
    where Generic: crate::naming_conventions::ToUpperCamelCaseString
{
    fn parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}Parameters",
            self.to_upper_camel_case_string()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadUpperCamelCaseTokenStream {
    fn payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<Generic> PayloadUpperCamelCaseTokenStream for Generic 
    where Generic: crate::naming_conventions::ToUpperCamelCaseString
{
    fn payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}Payload",
            self.to_upper_camel_case_string()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn payload_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<Generic> PayloadWithSerializeDeserializeUpperCamelCaseTokenStream for Generic 
    where Generic: crate::naming_conventions::ToUpperCamelCaseString
{
    fn payload_with_serialize_deserialize_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}PayloadWithSerializeDeserialize",
            self.to_upper_camel_case_string()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}