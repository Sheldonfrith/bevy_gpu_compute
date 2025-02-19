use syn::{Field, Visibility, visit_mut::VisitMut};

pub struct PubRemover {}

impl VisitMut for PubRemover {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        syn::visit_mut::visit_item_fn_mut(self, i);
        i.vis = Visibility::Inherited;
    }

    fn visit_field_mut(&mut self, i: &mut Field) {
        syn::visit_mut::visit_field_mut(self, i);
        i.vis = Visibility::Inherited;
    }

    fn visit_item_type_mut(&mut self, i: &mut syn::ItemType) {
        syn::visit_mut::visit_item_type_mut(self, i);
        i.vis = Visibility::Inherited;
    }

    fn visit_item_struct_mut(&mut self, i: &mut syn::ItemStruct) {
        syn::visit_mut::visit_item_struct_mut(self, i);
        i.vis = Visibility::Inherited;
    }

    fn visit_item_const_mut(&mut self, i: &mut syn::ItemConst) {
        syn::visit_mut::visit_item_const_mut(self, i);
        i.vis = Visibility::Inherited;
    }
}
