use bevy_gpu_compute_core::wgsl::shader_module::user_defined_portion::WgslShaderModuleUserPortion;
use proc_macro2::TokenStream;
use quote::quote;

use crate::pipeline::phases::final_structure_generator::per_component_expansion::ToStructInitializer;

pub fn generate_shader_module_object(
    wgsl_shader_module: &WgslShaderModuleUserPortion,
) -> TokenStream {
    let static_consts: TokenStream = wgsl_shader_module
        .static_consts
        .iter()
        .map(|const_assignment| {
            let ts = ToStructInitializer::wgsl_const_assignment(const_assignment);
            quote!(#ts,)
        })
        .collect();

    let helper_types: TokenStream = wgsl_shader_module
        .helper_types
        .iter()
        .map(|type_def| {
            let ts = ToStructInitializer::wgsl_type(type_def);
            quote!(#ts,)
        })
        .collect();

    let uniforms2: TokenStream = wgsl_shader_module
        .uniforms
        .iter()
        .map(|uniform| {
            let ts = ToStructInitializer::wgsl_type(uniform);
            quote!(#ts,)
        })
        .collect();

    let input_arrays: TokenStream = wgsl_shader_module
        .input_arrays
        .iter()
        .map(|array| {
            let ts = ToStructInitializer::wgsl_input_array(array);
            quote!(#ts,)
        })
        .collect();

    let output_arrays: TokenStream = wgsl_shader_module
        .output_arrays
        .iter()
        .map(|output_array| {
            let ts = ToStructInitializer::wgsl_output_array(output_array);
            quote!(#ts,)
        })
        .collect();

    let helper_functions: TokenStream = wgsl_shader_module
        .helper_functions
        .iter()
        .map(|func| {
            let ts = ToStructInitializer::wgsl_function(func);
            quote!(#ts,)
        })
        .collect();

    let main_function: TokenStream =
        wgsl_shader_module
            .main_function
            .as_ref()
            .map_or(quote!(None), |func| {
                let ts = ToStructInitializer::wgsl_function(func);
                quote!(Some(#ts))
            });
    let bindings_map: TokenStream = ToStructInitializer::hash_map(
        wgsl_shader_module
            .binding_numbers_by_variable_name
            .as_ref()
            .unwrap(),
    );

    let library_modules: TokenStream = wgsl_shader_module
        .use_statements
        .iter()
        .map(|use_statement| {
            let ts = ToStructInitializer::wgsl_import(use_statement);
            quote!(#ts,)
        })
        .collect();

    quote!(
        pub fn parsed() -> WgslShaderModuleUserPortion {
            let mut user_portion = WgslShaderModuleUserPortion {
                static_consts: [
                    #static_consts
                    ]
                .into(),
                helper_types: [
                    #helper_types
                    ]
                .into(),
                uniforms: Vec::from([
                   #uniforms2
                    ]),
                input_arrays: [
                    #input_arrays
                    ]
                .into(),
                output_arrays: [
                    #output_arrays
                    ]
                .into(),
                helper_functions: [
                    #helper_functions
                    ]
                .into(),
                main_function: #main_function,
                binding_numbers_by_variable_name: Some(#bindings_map),
                use_statements: [].into(),
            };
            merge_libraries_into_wgsl_module(&mut user_portion, &mut [
                #library_modules
            ].into());
            user_portion
        }
    )
}

#[cfg(test)]
mod test {
    use proc_macro2::{Span, TokenStream};
    use proc_macro_error::abort;

    #[test]
    pub fn test_parse_str() {
        let uniforms_str2 = "";
        let _uniforms2: TokenStream = if let Ok(ts) = uniforms_str2.parse() {
            ts
        } else {
            abort!(
                Span::call_site(),
                "Failed to parse uniforms into TokenStream"
            );
        };
    }
}
