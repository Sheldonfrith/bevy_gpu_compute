use super::super::shader_sections::*;
use super::user_defined_portion::WgslShaderModuleUserPortion;
pub struct WgslShaderModuleDerivedPortion {
    // generate these based on inputs and outputs
    pub pipeline_consts: Vec<WgslConstAssignment>,
    /// currently unused
    pub uniforms: Vec<WgslType>,
    /// currently unused
    pub helper_functions: Vec<WgslFunction>,
    /// static, generate automatically from the user portion
    pub bindings: Vec<WgslWgpuBinding>,
    /// static, workgroup sizes changed via pipeline consts
    pub workgroups_declaration: WgslWorkgroupDeclaration,
}

impl From<&WgslShaderModuleUserPortion> for WgslShaderModuleDerivedPortion {
    fn from(user_portion: &WgslShaderModuleUserPortion) -> Self {
        let mut pipeline_consts = vec![];
        let bindings_map = user_portion
            .binding_numbers_by_variable_name
            .as_ref()
            .unwrap();
        let mut bindings = Vec::new();
        user_portion.uniforms.iter().for_each(|u| {
            bindings.push(WgslWgpuBinding::uniform(
                0,
                *bindings_map.get(&u.name.uniform()).unwrap(),
                u.name.uniform(),
                u.name.name(),
            ));
        });
        user_portion.input_arrays.iter().for_each(|a| {
            pipeline_consts.push(WgslConstAssignment::no_default(
                &a.item_type.name.input_array_length(),
                "u32",
            ));
            bindings.push(WgslWgpuBinding::input_array(
                0,
                *bindings_map.get(&a.item_type.name.input_array()).unwrap(),
                a.item_type.name.input_array(),
                format!("array < {} >", a.item_type.name.name(),),
            ));
        });
        user_portion.output_arrays.iter().for_each(|a| {
            pipeline_consts.push(WgslConstAssignment::no_default(
                &a.item_type.name.output_array_length(),
                "u32",
            ));
            let output_array = WgslWgpuBinding::output_array(
                0,
                *bindings_map.get(&a.item_type.name.output_array()).unwrap(),
                a.item_type.name.output_array(),
                format!("array < {} >", a.item_type.name.name(),),
            );
            bindings.push(output_array.clone());

            if let Some(counter_name) = &a.atomic_counter_name {
                bindings.push(WgslWgpuBinding::counter(
                    *bindings_map.get(counter_name).unwrap(),
                    a,
                    &output_array,
                ));
            }
        });
        WgslShaderModuleDerivedPortion {
            pipeline_consts,
            uniforms: Vec::new(),
            helper_functions: Vec::new(),
            bindings,
            workgroups_declaration: WgslWorkgroupDeclaration {
                shader_type: WgpuShaderType::Compute,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::{
        IterSpaceDimmension,
        wgsl::{
            shader_custom_type_name::ShaderCustomTypeName,
            shader_module::complete_shader_module::WgslShaderModule,
            shader_sections::{WgslInputArray, WgslOutputArray, WgslShaderModuleSectionCode},
        },
    };

    use super::*;

    #[test]
    fn test_wgsl_shader_module_library_portion_from_user_portion() {
        let user_portion = WgslShaderModuleUserPortion { static_consts: vec![WgslConstAssignment { code: WgslShaderModuleSectionCode { wgsl_code: "const example_module_const : u32 = 42;".to_string() } }], helper_types: vec![], uniforms: vec![WgslType { name: ShaderCustomTypeName::new("Uniforms"), code: WgslShaderModuleSectionCode { wgsl_code: "struct Uniforms { time : f32, resolution : vec2 < f32 > , }".to_string() } }], input_arrays: vec![WgslInputArray { item_type: WgslType { name: ShaderCustomTypeName::new("Position"), code: WgslShaderModuleSectionCode { wgsl_code: "alias Position  = array < f32, 2 > ;".to_string() } } }, WgslInputArray { item_type: WgslType { name: ShaderCustomTypeName::new("Radius") , code: WgslShaderModuleSectionCode { wgsl_code: "alias Radius  = f32;".to_string() } }}], output_arrays: vec![WgslOutputArray { item_type: WgslType { name: ShaderCustomTypeName::new("CollisionResult"), code: WgslShaderModuleSectionCode { wgsl_code: "struct CollisionResult { entity1 : u32, entity2 : u32, }".to_string() } }, atomic_counter_name: Some("collisionresult_counter".to_string()) }], helper_functions: vec![WgslFunction { name: "calculate_distance_squared".to_string(), code: WgslShaderModuleSectionCode { wgsl_code: "fn calculate_distance_squared(p1 : array < f32, 2 > , p2 : array < f32, 2 >)\n-> f32\n{\n    let dx = p1 [0] - p2 [0]; let dy = p1 [1] - p2 [1]; return dx * dx + dy *\n    dy;\n}".to_string() } }], main_function: Some(WgslFunction { name: "main".to_owned(), code: WgslShaderModuleSectionCode { wgsl_code: "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)\n{\n    let current_entity = iter_pos.x; let other_entity = iter_pos.y; if\n    current_entity >= POSITION_INPUT_ARRAY_LENGTH || other_entity >=\n    POSITION_INPUT_ARRAY_LENGTH || current_entity == other_entity ||\n    current_entity >= other_entity { return; } let current_radius =\n    radius_input_array [current_entity]; let other_radius = radius_input_array\n    [other_entity]; if current_radius <= 0.0 || other_radius <= 0.0\n    { return; } let current_pos = position_input_array [current_entity]; let\n    other_pos = position_input_array [other_entity]; let dist_squared =\n    calculate_distance_squared(current_pos, other_pos); let radius_sum =\n    current_radius + other_radius; if dist_squared < radius_sum * radius_sum\n    {\n        {\n            let collisionresult_output_array_index =\n            atomicAdd(& collisionresult_counter, 1u); if\n            collisionresult_output_array_index <\n            COLLISIONRESULT_OUTPUT_ARRAY_LENGTH\n            {\n                collisionresult_output_array\n                [collisionresult_output_array_index] = CollisionResult\n                { entity1 : current_entity, entity2 : other_entity, };\n            }\n        };\n    }\n}".to_owned() } }), binding_numbers_by_variable_name: Some(HashMap::from([(String::from("uniforms"), 0), (String::from("position_input_array"), 1), (String::from("radius_input_array"), 2), (String::from("collisionresult_output_array"), 3), (String::from("collisionresult_counter"), 4)])), use_statements: vec![],
     };

        let expected_wgsl_code = "const example_module_const : u32 = 42;
override POSITION_INPUT_ARRAY_LENGTH: u32;
override RADIUS_INPUT_ARRAY_LENGTH: u32;
override COLLISIONRESULT_OUTPUT_ARRAY_LENGTH: u32;
struct Uniforms { time : f32, resolution : vec2 < f32 > , }
alias Position  = array < f32, 2 > ;
alias Radius  = f32;
struct CollisionResult { entity1 : u32, entity2 : u32, }
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(0) @binding(1) var<storage, read> position_input_array: array < Position >;

@group(0) @binding(2) var<storage, read> radius_input_array: array < Radius >;

@group(0) @binding(3) var<storage, read_write> collisionresult_output_array: array < CollisionResult >;

@group(0) @binding(4) var<storage, read_write> collisionresult_counter: atomic<u32>;

fn calculate_distance_squared(p1 : array < f32, 2 > , p2 : array < f32, 2 >)
-> f32
{
    let dx = p1 [0] - p2 [0]; let dy = p1 [1] - p2 [1]; return dx * dx + dy *
    dy;
}
@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)
{
    let current_entity = iter_pos.x; let other_entity = iter_pos.y; if
    current_entity >= POSITION_INPUT_ARRAY_LENGTH || other_entity >=
    POSITION_INPUT_ARRAY_LENGTH || current_entity == other_entity ||
    current_entity >= other_entity { return; } let current_radius =
    radius_input_array [current_entity]; let other_radius = radius_input_array
    [other_entity]; if current_radius <= 0.0 || other_radius <= 0.0
    { return; } let current_pos = position_input_array [current_entity]; let
    other_pos = position_input_array [other_entity]; let dist_squared =
    calculate_distance_squared(current_pos, other_pos); let radius_sum =
    current_radius + other_radius; if dist_squared < radius_sum * radius_sum
    {
        {
            let collisionresult_output_array_index =
            atomicAdd(& collisionresult_counter, 1u); if
            collisionresult_output_array_index <
            COLLISIONRESULT_OUTPUT_ARRAY_LENGTH
            {
                collisionresult_output_array
                [collisionresult_output_array_index] = CollisionResult
                { entity1 : current_entity, entity2 : other_entity, };
            }
        };
    }
}
";
        let module = WgslShaderModule::new(user_portion);
        assert_eq!(
            module.wgsl_code(IterSpaceDimmension::OneD),
            expected_wgsl_code
        );
    }
}
