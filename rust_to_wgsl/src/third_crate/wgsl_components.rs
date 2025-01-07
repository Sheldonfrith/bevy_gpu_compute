use proc_macro::TokenStream;

/// includes just the parts the user has input, with any relevant metadata necessary for the library to complete the module
pub struct WgslShaderModuleUserPortion {
    pub module_visibility: String,
    pub module_ident: String,
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
    pub input_arrays: Vec<WgslArray>,
    /// identified with a #[vec_output] attribute above them
    pub output_arrays: Vec<WgslOutputArray>,
    /// any function that appears besides the one called "main"
    pub helper_functions: Vec<WgslFunction>,
    /// the main function, identified by its name: "main"
    /// MUST contain a single parameter called "global_id" of type "WgslGlobalId"
    /// look for any attempt to ASSIGN to the value of "global_id.x", "global_id.y", or "global_id.z" or just "global_id" and throw an error
    pub main_function: Option<WgslFunction>,
}

pub struct WgslType {
    pub name: String,
    pub wgsl: String,
}
impl ToString for WgslType {
    fn to_string(&self) -> String {
        return format!("{}{}", self.wgsl.clone(), "\n");
    }
}
pub struct WgslFunction {
    pub name: String,
    pub wgsl_definition: String,
}
impl ToString for WgslFunction {
    fn to_string(&self) -> String {
        return format!("{}{}", self.wgsl_definition.clone(), "\n");
    }
}

/// assignments using let can happen within functions and we don't care about them, we don't need to change anything
pub struct WgslConstAssignment {
    pub assigner_keyword: String,
    pub var_name: String,
    pub var_type: WgslType,
    pub value: String,
}
impl ToString for WgslConstAssignment {
    fn to_string(&self) -> String {
        return format!(
            "{} {}: {} = {};\n",
            self.assigner_keyword, self.var_name, self.var_type.wgsl, self.value
        );
    }
}
pub struct WgslArray {
    pub type_name: String,
    pub item_type: WgslType,
    pub length: u32,
}
impl ToString for WgslArray {
    fn to_string(&self) -> String {
        return format!(
            "alias {} = array<{},{}>;\n",
            self.type_name, self.item_type.name, self.length
        );
    }
}
pub struct WgslOutputArray {
    pub arr: WgslArray,
    pub atomic_counter: bool,
}
impl ToString for WgslOutputArray {
    fn to_string(&self) -> String {
        let mut s = self.arr.to_string();
        if self.atomic_counter {
            s.push_str(&format!(
                "alias {}_counter : atomic<u32>;\n",
                self.arr.item_type.name
            ));
        }
        return s;
    }
}

pub enum WgpuShaderType {
    Compute,
    Vertex,
    Fragment,
}
impl ToString for WgpuShaderType {
    fn to_string(&self) -> String {
        match self {
            WgpuShaderType::Compute => "compute".to_string(),
            WgpuShaderType::Vertex => panic!("Vertex shaders not yet supported"),
            WgpuShaderType::Fragment => panic!("Fragment shaders not yet supported"),
        }
    }
}
pub struct WgslWorkgroupDeclaration {
    pub shader_type: WgpuShaderType,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl ToString for WgslWorkgroupDeclaration {
    fn to_string(&self) -> String {
        return format!(
            "@{} @workgroup_size({}, {}, {})\n",
            self.shader_type.to_string(),
            self.x,
            self.y,
            self.z
        );
    }
}
