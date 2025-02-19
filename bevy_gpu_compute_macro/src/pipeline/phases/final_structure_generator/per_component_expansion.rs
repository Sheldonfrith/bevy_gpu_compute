use std::collections::HashMap;

use bevy_gpu_compute_core::wgsl::{
    shader_custom_type_name::ShaderCustomTypeName,
    shader_sections::{
        WgslConstAssignment, WgslFunction, WgslImport, WgslInputArray, WgslOutputArray,
        WgslShaderModuleSectionCode, WgslType,
    },
};
use proc_macro2::TokenStream;
use quote::quote;

pub struct ToStructInitializer {}

impl ToStructInitializer {
    pub fn wgsl_shader_module_component(c: &WgslShaderModuleSectionCode) -> TokenStream {
        let w = &c.wgsl_code;
        quote!(
            WgslShaderModuleSectionCode {
                wgsl_code: (#w).to_string(),
            }
        )
    }

    pub fn wgsl_type(c: &WgslType) -> TokenStream {
        let n = ToStructInitializer::custom_type_name(&c.name);
        let c = ToStructInitializer::wgsl_shader_module_component(&c.code);
        quote!(
            WgslType {
                name: #n,
                code: #c,
            }
        )
    }

    pub fn custom_type_name(c: &ShaderCustomTypeName) -> TokenStream {
        let n = c.name();
        quote!(
            ShaderCustomTypeName::new(#n)
        )
    }

    pub fn wgsl_function(c: &WgslFunction) -> TokenStream {
        let n = &c.name;
        let c = ToStructInitializer::wgsl_shader_module_component(&c.code);
        quote!(
            WgslFunction {
                name: (#n).to_string(),
                code: #c
            }
        )
    }

    pub fn wgsl_const_assignment(c: &WgslConstAssignment) -> TokenStream {
        let c = ToStructInitializer::wgsl_shader_module_component(&c.code);
        quote!(
            WgslConstAssignment {
                code: #c,
            }
        )
    }

    pub fn wgsl_input_array(c: &WgslInputArray) -> TokenStream {
        let i = ToStructInitializer::wgsl_type(&c.item_type);
        quote!(
            WgslInputArray {
                item_type: #i,
            }
        )
    }

    pub fn wgsl_output_array(c: &WgslOutputArray) -> TokenStream {
        let i = ToStructInitializer::wgsl_type(&c.item_type);
        let ac: TokenStream = c
            .atomic_counter_name
            .as_ref()
            .map_or("None".to_string(), |counter| {
                format!("Some(\"{}\".to_string())", counter)
            })
            .to_string()
            .parse()
            .unwrap();
        quote!(
            WgslOutputArray {
                item_type: #i,
                atomic_counter_name: #ac

            }
        )
    }

    pub fn wgsl_import(c: &WgslImport) -> TokenStream {
        let i: TokenStream = c.path.parse().unwrap();
        quote!(#i)
    }

    pub fn hash_map(c: &HashMap<String, u32>) -> TokenStream {
        let entries: TokenStream = c
            .iter()
            .map(|(k, v)| {
                quote! {
                    (#k .to_string(), #v),
                }
            })
            .collect();
        quote!(
            HashMap::from([#entries])
        )
    }
}
