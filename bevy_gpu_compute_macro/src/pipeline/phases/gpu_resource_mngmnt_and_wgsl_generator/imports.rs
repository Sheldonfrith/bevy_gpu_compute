use bevy_gpu_compute_core::wgsl::shader_sections::WgslImport;

use crate::pipeline::phases::user_import_collector::user_import::UserImport;

pub fn generate_user_imports_for_wgsl_module_def(
    user_imports: &Vec<UserImport>,
) -> Vec<WgslImport> {
    let mut out = vec![];
    for import in user_imports.iter() {
        let mut segments: Vec<String> = import.path.iter().map(|ident| ident.to_string()).collect();
        segments.push("parsed()".to_string());
        let mut path = segments.join("::");

        if import.has_leading_colon {
            path = format!("::{path}");
        }

        out.push(WgslImport { path });
    }
    out
}
