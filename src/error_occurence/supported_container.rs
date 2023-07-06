#[derive(Debug)]
pub enum SupportedContainer {
    Vec {
        path: String,
        vec_element_type: crate::error_occurence::vec_element_type::VecElementType,
    },
    HashMap {
        path: String,
        hashmap_key_type: crate::error_occurence::hashmap_value_type::HashMapKeyType,
        hashmap_value_type: crate::error_occurence::hashmap_key_type::HashMapValueType,
    },
    Path {
        path: String,
        vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
    },
    Reference {
        reference_ident: proc_macro2::Ident,
        lifetime_ident: proc_macro2::Ident,
    },
}
