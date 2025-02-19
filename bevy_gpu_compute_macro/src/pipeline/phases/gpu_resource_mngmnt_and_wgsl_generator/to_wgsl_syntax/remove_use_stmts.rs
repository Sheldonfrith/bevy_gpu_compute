pub struct UseStmtRemover {}

use quote::quote;
use syn::{Item, visit_mut::VisitMut};

impl VisitMut for UseStmtRemover {
    fn visit_item_mut(&mut self, i: &mut Item) {
        syn::visit_mut::visit_item_mut(self, i);
        if let Item::Use(use_stmt) = i {
            // remove the use statement
            *i = Item::Verbatim(quote! {})
        }
    }
}
