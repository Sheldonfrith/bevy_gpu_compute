use crate::wgsl::shader_module::user_defined_portion::WgslShaderModuleUserPortion;

pub fn merge_libraries_into_wgsl_module(user_module: &mut WgslShaderModuleUserPortion, library_modules: &mut Vec<WgslShaderModuleUserPortion>) {
    for library in library_modules.iter_mut() {
        user_module.helper_functions.append(&mut library.helper_functions);
        user_module.static_consts.append(&mut library.static_consts);
        user_module.helper_types.append(&mut library.helper_types);
    }
}
