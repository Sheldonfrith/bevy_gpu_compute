use crate::pipeline::{compilation_unit::CompilationUnit, phases::compiler_phase::CompilerPhase};

use super::{
    alter_main_function_for_cpu_usage::mutate_main_function_for_cpu_usage,
    make_types_pod::make_types_pod, make_types_public::make_types_public,
    remove_internal_attributes::remove_internal_attributes,
};

/// alter the original rust code slightly to ensure it can be safely used by the user without interferring with the GPU side of the library
pub struct ModuleForRustUsageCleaner;

impl CompilerPhase for ModuleForRustUsageCleaner {
    fn execute(&self, input: &mut CompilationUnit) {
        let mut m = input.rust_module_for_cpu().clone();
        if input.main_func_required() {
            mutate_main_function_for_cpu_usage(input.wgsl_module_user_portion(), &mut m);
        }
        remove_internal_attributes(&mut m);
        make_types_pod(&mut m);
        make_types_public(&mut m);
        input.set_rust_module_for_cpu(m.clone());
    }
}
