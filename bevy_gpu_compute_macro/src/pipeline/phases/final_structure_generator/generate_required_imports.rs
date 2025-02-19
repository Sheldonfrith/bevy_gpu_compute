use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_required_imports() -> TokenStream {
    quote! {
        use super::*;
        use bevy_gpu_compute_core::wgsl::shader_sections::*; //todo, make this less brittle, how?
        use bevy_gpu_compute_core::wgsl::shader_custom_type_name::*;
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_core::wgsl::shader_module::user_defined_portion::WgslShaderModuleUserPortion;
        use bevy_gpu_compute_core::*;
        use std::collections::HashMap;
    }
}
