use bevy::{
    log,
    render::{render_resource::Buffer, renderer::RenderDevice},
};

use crate::task::lib::BevyGpuComputeTask;

/**
Binding the buffers to the corresponding wgsl code.

For example, this might be the wgsl code:
```wgsl

@group(0) @binding(0) var<storage, read> positions: Positions;
@group(0) @binding(1) var<storage, read> radii: Radii;
@group(0) @binding(2) var<storage, read_write> results: CollisionResults;
```

The numbers in the `@binding` are the bind group entry numbers. The `@group` is the bind group number. We are only using a single bind group in the current library version.
 */
pub fn create_bind_group(task: &mut BevyGpuComputeTask, render_device: &RenderDevice) {
    log::trace!("Creating bind group for task {}", task.name());
    let mut bindings = Vec::new();
    for (i, s) in task.configuration().inputs().configs().iter().enumerate() {
        if let Some(conf_in_buff) = task.buffers().config.get(i) {
            bindings.push(wgpu::BindGroupEntry {
                binding: s.binding_number,
                resource: conf_in_buff.as_entire_binding(),
            });
        } else {
            panic!("Config input has not been set for task {}", task.name());
        }
    }
    for (i, s) in task.configuration().inputs().arrays().iter().enumerate() {
        if let Some(buffer) = task.buffers().input.get(i) {
            bindings.push(wgpu::BindGroupEntry {
                binding: s.binding_number,
                resource: buffer.as_entire_binding(),
            });
        } else {
            panic!(
                "Input has not been set for task {}, with index: {}. Input buffers: {:?}",
                task.name(),
                i,
                task.buffers().input
            );
        }
    }
    for (i, s) in task.configuration().outputs().arrays().iter().enumerate() {
        let output_buffer: &Buffer = task.buffers().output.main.get(i).unwrap();
        bindings.push(wgpu::BindGroupEntry {
            binding: s.binding_number,
            resource: output_buffer.as_entire_binding(),
        });
        if s.include_count {
            let count_buffer: &Buffer = task.buffers().output.count.get(i).unwrap();
            bindings.push(wgpu::BindGroupEntry {
                binding: s.count_binding_number.unwrap(),
                resource: count_buffer.as_entire_binding(),
            });
        }
    }
    let layout = task.runtime_state().bind_group_layout();
    *task.runtime_state_mut().bind_group_mut() =
        Some(render_device.create_bind_group(task.name(), layout, &bindings));
}
