pub trait ToUpperCamelCase {
    fn to_upper_camel_case(&self) -> std::string::String;
}

impl<T> ToUpperCamelCase for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_upper_camel_case(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::UpperCamel)
    }
}

// pub trait ToUpperCamelCaseTokenStream {
//     fn to_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }

// impl<T> ToUpperCamelCaseTokenStream for T
//     where T: ToUpperCamelCase,
// {
//     fn to_upper_camel_case_token_stream(&self) -> std::string::String {
//         let value_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::ToUpperCamelCase::to_upper_camel_case(self);
//         value_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{value_upper_camel_case_stringified} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

pub trait ToSnakeCase {//todo rename as just snake case and all variable names
    fn to_snake_case(&self) -> std::string::String;
}

impl<T> ToSnakeCase for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_snake_case(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::Snake)
    }
}

//

pub trait ToScreamingSnakeCase {
    fn to_screaming_snake_case(&self) -> std::string::String;
}

impl<T> ToScreamingSnakeCase for T
where
    std::string::String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_screaming_snake_case(&self) -> std::string::String {
        convert_case::Casing::to_case(self, convert_case::Case::ScreamingSnake)
    }
}