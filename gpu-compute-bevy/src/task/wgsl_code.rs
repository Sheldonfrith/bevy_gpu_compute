use std::hash::{Hash, Hasher};

use bevy::{prelude::Component, render::renderer::RenderDevice};
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub struct WgslCode {
    code: String,
    entry_point_function_name: String,
    shader_module: Option<ShaderModule>,
}
impl Default for WgslCode {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            entry_point_function_name: "".to_string(),
            shader_module: None,
        }
    }
}

impl WgslCode {
    pub fn from_string(
        label: &str,
        render_device: &RenderDevice,
        wgsl_code: String,
        entry_point_function_name: String,
    ) -> Self {
        Self {
            code: wgsl_code.clone(),
            entry_point_function_name,
            shader_module: Some(render_device.create_shader_module(ShaderModuleDescriptor {
                label: Some(label),
                source: ShaderSource::Wgsl(wgsl_code.into()),
            })),
        }
    }
    pub fn from_file(
        label: &str,
        render_device: &RenderDevice,
        file_path: &str,
        entry_point_function_name: String,
    ) -> Self {
        let code = std::fs::read_to_string(file_path).unwrap();
        Self::from_string(label, render_device, code, entry_point_function_name)
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn entry_point_function_name(&self) -> &str {
        &self.entry_point_function_name
    }
    pub fn shader_module(&self) -> &ShaderModule {
        assert!(
            self.shader_module.is_some(),
            "Trying to retrieve shader module that doesn't exist"
        );
        &self.shader_module.as_ref().unwrap()
    }
}
