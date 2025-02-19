pub struct UserImport {
    pub has_leading_colon: bool,
    pub path: Vec<syn::Ident>,
}

impl UserImport {
    pub fn new(has_leading_colon: bool, path: Vec<syn::Ident>) -> Self {
        Self {
            has_leading_colon,
            path,
        }
    }
}
