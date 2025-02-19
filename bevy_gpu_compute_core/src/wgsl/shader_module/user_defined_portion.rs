use std::collections::HashMap;

use super::super::shader_sections::*;

#[derive(Clone, Debug, PartialEq)]
pub struct WgslShaderModuleUserPortion {
    /// defined with the "const" keyword
    /// single line
    /// value remains static
    /// type must be wgsl type or created somewhere else in the module
    /// value could be a type instantiation, a scalar, or a function
    pub static_consts: Vec<WgslConstAssignment>,
    /// defined with either struct keyword, or a type alias
    /// These are not associated with any buffers and exist only on the GPU
    pub helper_types: Vec<WgslType>,
    /// identified with a #[config_input] attribute above them
    pub uniforms: Vec<WgslType>,
    /// identified with a #[vec_input] attribute above them
    pub input_arrays: Vec<WgslInputArray>,
    /// identified with a #[vec_output] attribute above them
    pub output_arrays: Vec<WgslOutputArray>,
    /// any function that appears besides the one called "main"
    pub helper_functions: Vec<WgslFunction>,
    /// the main function, identified by its name: "main"
    /// MUST contain a single parameter called "global_id" of type "WgslGlobalId"
    /// look for any attempt to ASSIGN to the value of "global_id.x", "global_id.y", or "global_id.z" or just "global_id" and throw an error
    pub main_function: Option<WgslFunction>,
    pub binding_numbers_by_variable_name: Option<HashMap<String, u32>>,
    pub use_statements: Vec<WgslImport>,
}
impl WgslShaderModuleUserPortion {
    pub fn empty() -> Self {
        Self {
            static_consts: vec![],
            helper_types: vec![],
            uniforms: vec![],
            input_arrays: vec![],
            output_arrays: vec![],
            helper_functions: vec![],
            main_function: None,
            binding_numbers_by_variable_name: None,
            use_statements: vec![],
        }
    }
}
