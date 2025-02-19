#![feature(allocator_api)]

use pipeline::lib::CompilerPipeline;
use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, set_dummy};
use syn::parse_macro_input;
mod pipeline;
/**
## *Please read this documentation carefully, especially if you are getting errors that you don't understand!*

*...because it's currently impossible with rust proc-macros to override some of the error messages, so a lot of them don't actually indicate correctly what your issue is. So this documentation is how you are going to solve them!*

Here are some pointers:
- No let statements allowed except within functions. If you want to define a variable use "const" instead.
- If you see an error like `cannot transmute between types of different sizes...`, it is probably a memory padding/alignment issue. The f16 (aka PodF16) is a common culprit. If you have structs with multiple fields of different types, try adding various amounts of padding to the structs to see if that fixes the issue.
- When accessing special WGSL types like `Vec3`, `Mat3x4`, etc. you CANNOT use parenthesis when accessing the fields. For example:
### Valid:
```rust
# use bevy_gpu_compute_core::wgsl_helpers::*;
let my_vec3 = Vec2I32::new(1,2);
// valid:
let x = my_vec3.x;
let x = my_vec3[0];
```
### Invalid:
```compile_fail
// invalid:
let x = my_vec3.x();
```
- "rgba" field access on vectors is not supported right now, use index or xyzw instead.
- Every Input/Output you want to transfer between the CPU and GPU must have its type defined within the shader module. Here's how you do that:
    - Input Vec/Array/Matrices: Define the inner-type, and put `#[vec_input]` above the type definition. Example: If you want to work with an input equivalent to `Vec<{x:f32, y:f32}>` in your module, then write
    ```ignore
    #[vec_input]
    pub struct MyStruct { x: f32, y: f32 }
    ```

    We wrap the inner type in an array for you automatically, so that you don't have to worry about data length or alignments.
* You cannot use rust's struct-literal syntax for initializing structs from the helper types module. You must use the `new` method instead. For example:
    ## Wrong:
     ```compile_fail
         #use bevy_gpu_compute_corepu_compute_core::wgsl_in_rust_helpers::*;
          const MY_CONST: Vec3Bool = Vec3Bool {
        x: true,
        y: false,
        z: true,
    };```


    ## Correct:
    ```
    #use bevy_gpu_compute_corepu_compute_core::wgsl_in_rust_helpers::*;
    const MY_CONST: Vec3Bool = Vec3Bool::new(true, false, true);
    ```
* If you see the error `the trait bound `bool: Pod` is not satisfied...` make sure you are not trying to use a `bool` in any input data or output data. The `bool` type CAN be used but only ON the GPU, it cannot be passed between the CPU and GPU.
 */
#[proc_macro_attribute]
#[proc_macro_error]
pub fn wgsl_shader_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    set_dummy(item.clone().into());
    let module = parse_macro_input!(item as syn::ItemMod);
    let compiler_pipeline = CompilerPipeline::default();
    compiler_pipeline.compile(module, true).into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn wgsl_shader_module_library(_attr: TokenStream, item: TokenStream) -> TokenStream {
    set_dummy(item.clone().into());
    let module = parse_macro_input!(item as syn::ItemMod);
    let compiler_pipeline = CompilerPipeline::default();
    compiler_pipeline.compile(module, false).into()
}

/// used to help this library figure out what to do with user-defined types
#[proc_macro_attribute]
#[proc_macro_error]
pub fn wgsl_config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}
/// used to help this library figure out what to do with user-defined types
#[proc_macro_attribute]
#[proc_macro_error]
pub fn wgsl_input_array(_attr: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}
/// used to help this library figure out what to do with user-defined types
#[proc_macro_attribute]
#[proc_macro_error]
pub fn wgsl_output_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}
/// used to help this library figure out what to do with user-defined types
#[proc_macro_attribute]
#[proc_macro_error]
pub fn wgsl_output_array(_attr: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}
