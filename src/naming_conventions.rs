pub trait ToUpperCamelCase<T> {
    fn to_upper_camel_case(&self) -> std::string::String;
}

impl<T> ToUpperCamelCase<T> for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_upper_camel_case(&self) -> String {
        convert_case::Casing::to_case(self, convert_case::Case::UpperCamel)
    }
}

pub trait ToLowerSnakeCase<T> {
    fn to_lower_snake_case(&self) -> std::string::String;
}

impl<T> ToLowerSnakeCase<T> for T
where
    String: PartialEq<T>,
    Self: AsRef<str>,
{
    fn to_lower_snake_case(&self) -> String {
        convert_case::Casing::to_case(self, convert_case::Case::Snake).to_lowercase()
    }
}
