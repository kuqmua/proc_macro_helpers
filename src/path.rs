pub enum Path {
    TufaCommon,
    Crate,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Path::TufaCommon => write!(f, "tufa_common"),
            Path::Crate => write!(f, "crate"),
        }
    }
}
