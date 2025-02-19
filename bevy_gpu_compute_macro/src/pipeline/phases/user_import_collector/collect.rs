// find all user declared imports, and make a list of them

use syn::visit::Visit;

use super::user_import::UserImport;

#[derive(Default)]
struct UserImportCollector {
    use_statements: Vec<UserImport>,
}

const BUILT_IN_USE_STATEMENT_PATHS: [&str; 3] =
    ["wgsl_helpers", "bevy_gpu_compute", "bevy_gpu_compute_macro"];

impl<'ast> Visit<'ast> for UserImportCollector {
    fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
        syn::visit::visit_item_use(self, i);
        let mut built_in_handler = BuiltInUseStatementHandler::default();
        built_in_handler.visit_item_use(i);

        if built_in_handler.found {
            return;
        }

        let leading_colon = i.leading_colon.is_some();
        let path = traverse_use_tree(&i.tree);
        self.use_statements
            .push(UserImport::new(leading_colon, path));
    }
}

#[derive(Default)]
struct BuiltInUseStatementHandler {
    found: bool,
}

impl Visit<'_> for BuiltInUseStatementHandler {
    fn visit_use_path(&mut self, i: &syn::UsePath) {
        syn::visit::visit_use_path(self, i);
        if BUILT_IN_USE_STATEMENT_PATHS.contains(&i.ident.to_string().as_str()) {
            self.found = true;
        }
    }
    fn visit_use_name(&mut self, i: &syn::UseName) {
        syn::visit::visit_use_name(self, i);
        if BUILT_IN_USE_STATEMENT_PATHS.contains(&i.ident.to_string().as_str()) {
            self.found = true;
        }
    }
}

fn traverse_use_tree(use_tree: &syn::UseTree) -> Vec<syn::Ident> {
    let mut handler = UseTreeHandler::default();
    handler.visit_use_tree(use_tree);

    if !handler.is_glob {
        panic!("Only use globs are allowed (e.g. `use foo::bar::*;`)");
    }

    handler.path
}

#[derive(Default)]
struct UseTreeHandler {
    is_glob: bool,
    path: Vec<syn::Ident>,
}

impl<'ast> Visit<'ast> for UseTreeHandler {
    fn visit_use_tree(&mut self, i: &'ast syn::UseTree) {
        match i {
            syn::UseTree::Path(path) => self.path.push(path.ident.clone()),
            syn::UseTree::Name(_) => panic!("Only use globs are allowed (e.g. `use foo::bar::*;`)"),
            syn::UseTree::Rename(_) => {
                panic!("Use renames are unsupported (e.g. `use foo as bar;`)")
            }
            syn::UseTree::Glob(_) => self.is_glob = true,
            syn::UseTree::Group(_) => {
                panic!("Use groups are unsupported (e.g. `use foo::{{bar, baz}};`)")
            }
        };
        syn::visit::visit_use_tree(self, i);
    }
}

pub fn collect_user_imports(original_rust_module: &syn::ItemMod) -> Vec<UserImport> {
    let mut import_collector = UserImportCollector::default();
    import_collector.visit_item_mod(original_rust_module);
    import_collector.use_statements
}
