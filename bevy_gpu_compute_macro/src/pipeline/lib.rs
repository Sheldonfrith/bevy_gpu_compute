use proc_macro2::TokenStream;

use super::phases::{
    compiler_phase::CompilerPhase, custom_type_collector::compiler_phase::CustomTypeCollector,
    final_structure_generator::compiler_phase::FinalStructureGenerator,
    gpu_resource_mngmnt_and_wgsl_generator::compiler_phase::GpuResourceMngmntAndWgslGenerator,
    module_for_rust_usage_cleaner::compiler_phase::ModuleForRustUsageCleaner,
    non_mutating_tree_validation::compiler_phase::NonMutatingTreeValidation,
    typesafe_buffer_builders_generator::compiler_phase::TypesafeBufferBuildersGenerator,
    user_import_collector::compiler_phase::UserImportCollector,
    wgsl_helper_transformer::compiler_phase::WgslHelperTransformer,
};
use crate::pipeline::compilation_unit::CompilationUnit;

pub struct CompilerPipeline {
    phases: Vec<Box<dyn CompilerPhase>>,
}

impl Default for CompilerPipeline {
    fn default() -> Self {
        Self {
            phases: vec![
                Box::new(NonMutatingTreeValidation {}),
                Box::new(UserImportCollector {}),
                Box::new(CustomTypeCollector {}),
                Box::new(TypesafeBufferBuildersGenerator {}),
                Box::new(WgslHelperTransformer {}),
                Box::new(GpuResourceMngmntAndWgslGenerator {}),
                Box::new(ModuleForRustUsageCleaner {}),
                Box::new(FinalStructureGenerator {}),
            ],
        }
    }
}
impl CompilerPipeline {
    pub fn compile(&self, module: syn::ItemMod, main_func_required: bool) -> TokenStream {
        let mut unit = CompilationUnit::new(module, main_func_required);
        for phase in &self.phases {
            phase.execute(&mut unit);
        }
        if let Some(compiled) = unit.compiled_tokens() {
            compiled.clone()
        } else {
            panic!("No compiled tokens found, missing a compile phase that produces them");
        }
    }
}
