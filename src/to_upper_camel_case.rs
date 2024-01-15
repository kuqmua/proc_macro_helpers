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
