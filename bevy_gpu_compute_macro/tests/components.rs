#![feature(f16)]
#![allow(clippy::all)]
use std::collections::HashMap;

use bevy_gpu_compute_core::{
    TypeErasedArrayOutputData, TypesSpec,
    wgsl::{
        shader_custom_type_name::ShaderCustomTypeName,
        shader_module::user_defined_portion::WgslShaderModuleUserPortion,
        shader_sections::{
            WgslConstAssignment, WgslFunction, WgslInputArray, WgslOutputArray,
            WgslShaderModuleSectionCode, WgslType,
        },
    },
    wgsl_helpers::WgslIterationPosition,
};
use bevy_gpu_compute_macro::wgsl_shader_module;
use pretty_assertions::assert_eq;

#[test]
fn test_simple_struct() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::WgslIterationPosition;

        struct TStruct {
            x: f32,
            y: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>) {}"
    );
}

#[test]
fn test_struct_creation() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::WgslIterationPosition;

        struct TStruct {
            x: f32,
            y: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            let obj = TStruct { x: 1.0, y: 2.0 };
        }
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>) { let obj = TStruct(1.0, 2.0); }"
    );
}

#[test]
fn test_struct_creation_with_nested_transforms() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;

        struct TStruct {
            x: f32,
            y: Vec3F32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            let obj = TStruct {
                x: 1.0,
                y: Vec3F32::new(2.0, 3.0, 4.0),
            };
        }
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());

    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)\n{ let obj = TStruct(1.0,vec3<f32>(2.0, 3.0, 4.0)); }"
    );
}
#[test]
fn test_type_alias() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        type MyType = i32;
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.helper_types.first().unwrap().code.wgsl_code,
        "alias MyType  = i32;"
    );
}
#[test]
fn test_consts() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::{WgslIterationPosition, *};
        const MY_CONST: i32 = 3;
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 1);
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.static_consts.first().unwrap().code.wgsl_code,
        "const MY_CONST : i32 = 3;"
    );
}

#[test]
fn test_output_arrays() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::wgsl_output_array;
        #[wgsl_output_array]
        struct CollisionResult {
            entity1: u32,
            entity2: u32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 1);
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.output_arrays.first().unwrap().item_type.code.wgsl_code,
        "struct CollisionResult { entity1 : u32, entity2 : u32, }"
    );

    assert!(
        t2.output_arrays
            .first()
            .unwrap()
            .atomic_counter_name
            .is_none()
    );
}

#[test]
fn test_helper_functions() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        fn calculate_distance_squared(p1: [f32; 2], p2: [f32; 2]) -> f32 {
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            dx * dx + dy * dy
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.len() == 1);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.helper_functions.first().unwrap().code.wgsl_code,
        "fn calculate_distance_squared(p1 : array < f32, 2 > , p2 : array < f32, 2 >)\n-> f32\n{\n    let dx = p1 [0] - p2 [0]; let dy = p1 [1] - p2 [1]; return dx * dx + dy *\n    dy;\n}"
    );
}

#[test]
// expect a panic
#[should_panic(expected = "not implemented")]
fn can_extract_types() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::wgsl_config;
        #[wgsl_config]
        struct MyConfig {
            value: PodF16,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    fn fun<T: TypesSpec>() -> T::ConfigInputTypes {
        unimplemented!();
    }
    let _t = fun::<test_module::Types>();
}

#[test]
fn test_simple_type_transforms() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::{WgslIterationPosition, *};
        struct TStruct {
            x: f32,
            y: Vec3F32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.helper_types.first().unwrap().code.wgsl_code,
        "struct TStruct { x : f32, y : vec3 < f32 > , }"
    );
}

#[test]
fn test_doc_comments() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::wgsl_config;
        #[wgsl_config]
        struct MyConfig {
            f16_val: PodF16,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.len() == 1);
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
}
#[test]
fn test_type_casting_and_implicit_returns() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        #[allow(clippy::unnecessary_cast)]
        fn helper() -> f32 {
            let x = 1 as f32;
            let y = 3.5_f32;
            let z = 54.4f32;
            if true { y + z } else { x }
        }
        #[allow(unused_variables)]
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.helper_functions.first().unwrap().code.wgsl_code,
        "fn helper() -> f32\n{\n    let x = f32(1); let y = f32(3.5); let z = f32(54.4); if true\n    { return y + z; } else { return x; }\n}"
    );
}
#[test]
fn test_mutable_variables() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        #[allow(unused_assignments)]
        fn main(iter_pos: WgslIterationPosition) {
            let mut x = 1;
            let x1 = x;
            x = 2;
        }
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)\n{ var x = 1; let x1 = x; x = 2; }"
    );
}

#[test]
fn test_input_arrays() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::wgsl_input_array;
        #[wgsl_input_array]
        type Position = [f32; 2];
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.len() == 1);
    assert!(t2.uniforms.is_empty());
    // type Position = array<f32, 2>;
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());

    assert_eq!(
        t2.input_arrays.first().unwrap().item_type.code.wgsl_code,
        "alias Position  = array < f32, 2 > ;"
    )
}
#[test]

fn test_max_output_lengths_helper() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::*;
        #[wgsl_output_vec]
        type MyPosition = [f32; 2];
        #[wgsl_output_vec]
        struct Debug {
            v1: f32,
            v2: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let max_outputs = test_module::MaxOutputLengthsBuilder::new()
        .set_my_position(10)
        .set_debug(20)
        .finish();
    assert_eq!(
        max_outputs.get_by_name(&ShaderCustomTypeName::new("MyPosition")),
        10
    );
    assert_eq!(
        max_outputs.get_by_name(&ShaderCustomTypeName::new("Debug")),
        20
    );
}

#[test]

fn test_configs_builder() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::*;
        #[wgsl_config]
        type MyPosition = [f32; 2];
        #[wgsl_config]
        struct MyConfig {
            pub v1: f32,
            pub v2: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    #[allow(unused_variables)]
    let config = test_module::ConfigInputDataBuilder::new()
        .set_my_position([3.0, 4.0])
        .set_my_config(test_module::MyConfig { v1: 10.0, v2: 20.0 })
        .finish();
}

#[test]
fn test_output_vec() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::wgsl_output_vec;
        #[wgsl_output_vec]
        struct CollisionResult {
            entity1: u32,
            entity2: u32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 1);
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.is_empty());
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.output_arrays.first().unwrap().item_type.code.wgsl_code,
        "struct CollisionResult { entity1 : u32, entity2 : u32, }"
    );

    assert!(
        t2.output_arrays
            .first()
            .unwrap()
            .atomic_counter_name
            .is_some()
    );
    assert_eq!(
        t2.output_arrays
            .first()
            .unwrap()
            .atomic_counter_name
            .as_ref()
            .unwrap(),
        &"collisionresult_counter".to_string()
    )
}

#[test]
fn test_entire_collision_shader() {
    #[wgsl_shader_module]
    pub mod collision_shader {

        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::*;
        const EXAMPLE_MODULE_CONST: u32 = 42;
        #[wgsl_config]
        struct Uniforms {
            time: f32,
            resolution: Vec2F32,
        }
        #[wgsl_input_array]
        type Position = [f32; 2];
        #[wgsl_input_array]
        type Radius = f32;
        //* user output vectors
        #[wgsl_output_vec]
        struct CollisionResult {
            entity1: u32,
            entity2: u32,
        }
        fn calculate_distance_squared(p1: [f32; 2], p2: [f32; 2]) -> f32 {
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            dx * dx + dy * dy
        }
        fn main(iter_pos: WgslIterationPosition) {
            //* USER GENERATED LOGIC
            let current_entity = iter_pos.x;
            let other_entity = iter_pos.y;
            // Early exit if invalid entity or zero radius
            if current_entity >= WgslVecInput::vec_len::<Position>()
                || other_entity >= WgslVecInput::vec_len::<Position>()
                || current_entity == other_entity
                || current_entity >= other_entity
            {
                return;
            }
            let current_radius = WgslVecInput::vec_val::<Radius>(current_entity);
            let other_radius = WgslVecInput::vec_val::<Radius>(other_entity);
            if current_radius <= 0.0 || other_radius <= 0.0 {
                return;
            }
            let current_pos = WgslVecInput::vec_val::<Position>(current_entity);
            let other_pos = WgslVecInput::vec_val::<Position>(other_entity);
            let dist_squared = calculate_distance_squared(current_pos, other_pos);
            let radius_sum = current_radius + other_radius;
            // Compare squared distances to avoid sqrt
            if dist_squared < radius_sum * radius_sum {
                WgslOutput::push::<CollisionResult>(CollisionResult {
                    entity1: current_entity,
                    entity2: other_entity,
                });
            }
        }
    }
    let t2 = collision_shader::parsed();

    let user_portion = WgslShaderModuleUserPortion { static_consts: vec![WgslConstAssignment { code: WgslShaderModuleSectionCode {  wgsl_code: "const EXAMPLE_MODULE_CONST : u32 = 42;".to_string() } }], helper_types: vec![], uniforms: vec![WgslType { name: ShaderCustomTypeName::new("Uniforms"), code: WgslShaderModuleSectionCode { wgsl_code: "struct Uniforms { time : f32, resolution : vec2 < f32 > , }".to_string() } }], input_arrays: vec![WgslInputArray { item_type: WgslType { name: ShaderCustomTypeName::new("Position"), code: WgslShaderModuleSectionCode {  wgsl_code: "alias Position  = array < f32, 2 > ;".to_string() } } }, WgslInputArray { item_type: WgslType { name: ShaderCustomTypeName::new("Radius") , code: WgslShaderModuleSectionCode {  wgsl_code: "alias Radius  = f32;".to_string() } } }], output_arrays: vec![WgslOutputArray { item_type: WgslType { name: ShaderCustomTypeName::new("CollisionResult"), code: WgslShaderModuleSectionCode { wgsl_code: "struct CollisionResult { entity1 : u32, entity2 : u32, }".to_string() } }, atomic_counter_name: Some("collisionresult_counter".to_string()) }], helper_functions: vec![WgslFunction { name: "calculate_distance_squared".to_string(), code: WgslShaderModuleSectionCode {  wgsl_code: "fn calculate_distance_squared(p1 : array < f32, 2 > , p2 : array < f32, 2 >)\n-> f32\n{\n    let dx = p1 [0] - p2 [0]; let dy = p1 [1] - p2 [1]; return dx * dx + dy *\n    dy;\n}".to_string() } }], use_statements: vec![], main_function: Some(WgslFunction { name: "main".to_owned(), code: WgslShaderModuleSectionCode {  wgsl_code: "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)\n{\n    let current_entity = iter_pos.x; let other_entity = iter_pos.y; if\n    current_entity >= POSITION_INPUT_ARRAY_LENGTH || other_entity >=\n    POSITION_INPUT_ARRAY_LENGTH || current_entity == other_entity ||\n    current_entity >= other_entity { return; } let current_radius =\n    radius_input_array [current_entity]; let other_radius = radius_input_array\n    [other_entity]; if current_radius <= 0.0 || other_radius <= 0.0\n    { return; } let current_pos = position_input_array [current_entity]; let\n    other_pos = position_input_array [other_entity]; let dist_squared =\n    calculate_distance_squared(current_pos, other_pos); let radius_sum =\n    current_radius + other_radius; if dist_squared < radius_sum * radius_sum\n    {\n        {\n            let collisionresult_output_array_index =\n            atomicAdd(& collisionresult_counter, 1u); if\n            collisionresult_output_array_index <\n            COLLISIONRESULT_OUTPUT_ARRAY_LENGTH\n            {\n                collisionresult_output_array\n                [collisionresult_output_array_index] =
                CollisionResult(current_entity, other_entity);\n            }\n        };\n    }\n}".to_owned() } }),
        binding_numbers_by_variable_name: Some(HashMap::from([
            ("uniforms".to_string(), 1),
            ("position_input_array".to_string(), 2),
            ("radius_input_array".to_string(), 3),
            ("collisionresult_output_array".to_string(), 4),
            ("collisionresult_counter".to_string(), 5),
        ])), };
    assert_eq!(t2, user_portion);
}

#[test]

pub fn test_that_byte_conversions_work() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::{wgsl_input_array, *};
        #[wgsl_input_array]
        struct MyPosition {
            pub x: f32,
            pub y: f32,
        }
        #[wgsl_output_array]
        struct MyPositionOut {
            pub x: f32,
            pub y: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }

    // this is the desired inputdata api:
    let input_data = test_module::InputDataBuilder::new()
        .set_my_position(vec![
            test_module::MyPosition { x: 1.0, y: 2.0 },
            test_module::MyPosition { x: 3.0, y: 4.0 },
        ])
        .finish();

    assert_eq!(input_data.get_length("MyPosition").unwrap(), 2);
    let mut map = input_data.get_map().clone();
    // rename map key of "MyPosition" to "MyPositionOut"
    let my_position = map.remove("MyPosition").unwrap();
    map.insert("MyPositionOut".to_string(), my_position);
    let type_erased_output_data = TypeErasedArrayOutputData::new(map);
    let output_data = test_module::OutputDataBuilder::from(type_erased_output_data);
    let r = output_data.my_position_out.unwrap();
    assert_eq!(r.len(), 2);
    assert_eq!(r[0].x, 1.0);
}

#[test]
fn test_uniforms() {
    #[wgsl_shader_module]
    pub mod test_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::wgsl_config;
        #[wgsl_config]
        struct Uniforms {
            time: f32,
            resolution: Vec2F32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            let time = WgslConfigInput::get::<Uniforms>().time;
        }
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.is_empty());
    assert!(t2.input_arrays.is_empty());
    assert!(t2.uniforms.len() == 1);
    assert!(t2.helper_functions.is_empty());
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.is_empty());
    assert!(t2.helper_types.is_empty());
    assert_eq!(
        t2.uniforms.first().unwrap().code.wgsl_code,
        "struct Uniforms { time : f32, resolution : vec2 < f32 > , }"
    );
}

#[test]

fn test_functions_exposed_for_rust() {
    #[wgsl_shader_module]
    pub mod example_shader_module {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::*;
        const EXAMPLE_MODULE_CONST: u32 = 42;
        #[wgsl_config]
        struct Config {
            pub example_value: f32,
        }
        #[wgsl_input_array]
        type Position = [f32; 2];
        #[wgsl_input_array]
        type Radius = f32;
        //* user output vectors
        #[wgsl_output_vec]
        #[derive(PartialEq)]
        struct CollisionResult {
            pub entity1: u32,
            pub entity2: u32,
        }
        pub fn calculate_distance_squared(p1: [f32; 2], p2: [f32; 2]) -> f32 {
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            dx * dx + dy * dy
        }
        pub fn main(iter_pos: WgslIterationPosition) {
            //* USER GENERATED LOGIC
            let current_entity = iter_pos.x;
            let other_entity = iter_pos.y;
            // Early exit if invalid entity or zero radius
            if current_entity >= WgslVecInput::vec_len::<Position>()
                || other_entity >= WgslVecInput::vec_len::<Position>()
                || current_entity == other_entity
                || current_entity >= other_entity
            {
                return;
            }
            let current_radius =
                WgslVecInput::vec_val::<Radius>(current_entity) + EXAMPLE_MODULE_CONST as f32;
            let other_radius = WgslVecInput::vec_val::<Radius>(other_entity)
                + WgslConfigInput::get::<Config>().example_value;
            if current_radius <= 0.0 || other_radius <= 0.0 {
                return;
            }
            let current_pos = WgslVecInput::vec_val::<Position>(current_entity);
            let other_pos = WgslVecInput::vec_val::<Position>(other_entity);
            let dist_squared = calculate_distance_squared(current_pos, other_pos);
            let radius_sum = current_radius + other_radius;
            // Compare squared distances to avoid sqrt
            if dist_squared < radius_sum * radius_sum {
                WgslOutput::push::<CollisionResult>(CollisionResult {
                    entity1: current_entity,
                    entity2: other_entity,
                });
            }
        }
    }

    #[allow(dead_code, unused_variables, unused_imports)]
    pub mod example_shader_module2 {
        use bevy_gpu_compute_core::wgsl_helpers::*;
        use bevy_gpu_compute_macro::*;
        const EXAMPLE_MODULE_CONST: u32 = 42;
        struct Config {
            pub example_value: f32,
        }
        type Position = [f32; 2];
        type Radius = f32;
        //* user output vectors
        #[derive(PartialEq)]
        struct CollisionResult {
            pub entity1: u32,
            pub entity2: u32,
        }
        pub fn calculate_distance_squared(p1: [f32; 2], p2: [f32; 2]) -> f32 {
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            dx * dx + dy * dy
        }
        pub fn main(iter_pos: WgslIterationPosition) {
            //* USER GENERATED LOGIC
            let current_entity = iter_pos.x;
            let other_entity = iter_pos.y;
            // Early exit if invalid entity or zero radius
            if current_entity >= WgslVecInput::vec_len::<Position>()
                || other_entity >= WgslVecInput::vec_len::<Position>()
                || current_entity == other_entity
                || current_entity >= other_entity
            {
                return;
            }
            let current_radius =
                WgslVecInput::vec_val::<Radius>(current_entity) + EXAMPLE_MODULE_CONST as f32;
            let other_radius = WgslVecInput::vec_val::<Radius>(other_entity)
                + WgslConfigInput::get::<Config>().example_value;
            if current_radius <= 0.0 || other_radius <= 0.0 {
                return;
            }
            let current_pos = WgslVecInput::vec_val::<Position>(current_entity);
            let other_pos = WgslVecInput::vec_val::<Position>(other_entity);
            let dist_squared = calculate_distance_squared(current_pos, other_pos);
            let radius_sum = current_radius + other_radius;
            // Compare squared distances to avoid sqrt
            if dist_squared < radius_sum * radius_sum {
                WgslOutput::push::<CollisionResult>(CollisionResult {
                    entity1: current_entity,
                    entity2: other_entity,
                });
            }
        }
    }

    // you can now use any helper functions in the module like this:
    let f1_test = example_shader_module::calculate_distance_squared([1.0, 2.0], [3.0, 4.0]);
    assert_eq!(f1_test, 8.0);

    //* The main function is designed to mutate GPU buffers in WGSL, so we have to replicate this for a cpu version by requiring those same inputs to be passed as parameters to the main function */
    // setup inputs and outputs for main function:
    let config = example_shader_module::Config { example_value: 3. };
    let input_positions: Vec<example_shader_module::Position> = vec![[1.0, 2.0], [3.0, 4.0]];
    let input_radii: Vec<example_shader_module::Radius> = vec![10.0, 10.0];
    let mut output_collisions: Vec<example_shader_module::CollisionResult> = vec![];
    // run the main function, you can see that its signature has been changed to include all the stuff that would normally be in GPU buffers as direct function parameters
    example_shader_module::main(
        WgslIterationPosition { x: 0, y: 1, z: 1 },
        config,
        input_positions,
        input_radii,
        &mut output_collisions,
    );
    // results are in the mutated "output_collisions"...
    assert_eq!(
        output_collisions,
        vec![example_shader_module::CollisionResult {
            entity1: 0,
            entity2: 1
        }]
    );
}
