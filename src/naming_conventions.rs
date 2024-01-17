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