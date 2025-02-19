use std::collections::HashMap;

use array::ArrayToWgslTransformer;
use expr::ExprToWgslTransformer;
use implicit_to_explicit_return::ImplicitToExplicitReturnTransformer;
use local_var::replace_let_mut_with_var;
use proc_macro_error::abort;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use remove_attributes::remove_attributes;
use remove_pub_from_struct_def::PubRemover;
use remove_use_stmts::UseStmtRemover;
use syn::{File, parse, visit::Visit, visit_mut::VisitMut};
use r#type::TypeToWgslTransformer;
use type_def::TypeDefToWgslTransformer;
use wgsl_builtin_constructors::convert_wgsl_builtin_constructors;

use crate::pipeline::phases::custom_type_collector::custom_type::CustomType;

/**
 # Notes about conversions (all syntax not mentioned is either the same or not supported in wgsl)

- ForLoop(ExprForLoop):
  in wgsl, but with javascript style syntax: for (var i = 0; i< 10; i++){}

- Loop(ExprLoop):
  supported in wgsl, but with different syntax: `for (;;) {}`

- Reference(ExprReference):
  support pointer types, but this is something for a future version. Example of pointers in wgsl:
  ```ignore
  fn my_function(
      /* 'ptr<function,i32,read_write>' is the type of a pointer value that references
         memory for keeping an 'i32' value, using memory locations in the 'function'
         address space.  Here 'i32' is the store type.
         The implied access mode is 'read_write'.
         See "Address Space" section for defaults. */
      ptr_int: ptr<function,i32>,

      /* 'ptr<private,array<f32,50>,read_write>' is the type of a pointer value that
       refers to memory for keeping an array of 50 elements of type 'f32', using
       memory locations in the 'private' address space.
       Here the store type is 'array<f32,50>'.
       The implied access mode is 'read_write'.
       See the "Address space section for defaults.
      */
      ptr_array: ptr<private, array<f32, 50>>
    ) { }
  ```

- Struct(ExprStruct):
  supported, different syntax. in wgsl it becomes `Point(1,1)`, but we must warn the user that
  the order that they list the fields in when constructing their struct MUST be the same order
  that they are listed in when defining the struct type

- Array(ExprArray):
  supported, but with different syntax. in wgsl it becomes `array<f32, 3>`

- Types:
  - f32, f16, i32, u32, bool, vec2, vec3, vec4, mat2x2, mat3x3, mat4x4
  */
mod array;
mod expr;
mod implicit_to_explicit_return;
mod local_var;
pub mod remove_attributes;
mod remove_pub_from_struct_def;
mod remove_use_stmts;
mod r#type;
mod type_def;
mod wgsl_builtin_constructors;
/// called_from is for debug messages
pub fn convert_file_to_wgsl(
    input: TokenStream,
    custom_types: &Vec<CustomType>,
    called_from: String,
) -> String {
    let unprocessed_string = input.to_string();
    let input_no_attributes = remove_attributes(unprocessed_string);
    let processed_stream: TokenStream = input_no_attributes.parse().unwrap();
    let debug_str = processed_stream.clone().to_string();

    let mut file = if let Ok(f) = parse::<File>(processed_stream.into()) {
        f
    } else {
        let message = format!(
            "Failed to parse file, in convert_to_wgsl, from {}. File:{}",
            called_from, debug_str
        );
        abort!(Span::call_site(), message);
    };
    UseStmtRemover {}.visit_file_mut(&mut file);
    PubRemover {}.visit_file_mut(&mut file);
    TypeToWgslTransformer { custom_types }.visit_file_mut(&mut file);
    ArrayToWgslTransformer {}.visit_file_mut(&mut file);
    ExprToWgslTransformer {}.visit_file_mut(&mut file);
    ImplicitToExplicitReturnTransformer {}.visit_file_mut(&mut file);
    let mut type_def_transformer = TypeDefToWgslTransformer {
        replacements: HashMap::new(),
    };
    type_def_transformer.visit_file(&file);
    // expressions and type defs have to be transformed differently because they may change the token structure, so we have to transition to strings
    let mut string_version = file.to_token_stream().to_string();

    type_def_transformer.replacements.iter().for_each(|(k, v)| {
        string_version = string_version.replace(k, v);
    });
    // println!("Final string version: {}", string_version);
    // transform vec and matrix constructors
    string_version = convert_wgsl_builtin_constructors(string_version);
    string_version = replace_let_mut_with_var(&string_version);
    string_version
}
