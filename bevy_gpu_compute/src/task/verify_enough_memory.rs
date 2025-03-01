use bevy::log;

use crate::ram_limit::RamLimit;

use super::lib::BevyGpuComputeTask;

pub fn verify_have_enough_memory(tasks: &Vec<&BevyGpuComputeTask>, ram_limit: &RamLimit) {
    let total_bytes = tasks.iter().fold(0, |sum, task| {
        sum + task.runtime_state().max_output_bytes().get()
    });
    let available_memory = ram_limit.total_mem;
    if total_bytes as f32 > available_memory as f32 * 0.9 {
        log::error!(
            "Not enough memory to store all gpu compute task outputs. Available memory: {} GB, Max Output size: {} GB",
            available_memory as f32 / 1024.0 / 1024.0 / 1024.0,
            total_bytes as f32 / 1024.0 / 1024.0 / 1024.0
        );

        panic!("Not enough memory to store all gpu compute task outputs");
    }
}
