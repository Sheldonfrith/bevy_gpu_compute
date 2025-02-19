use crate::pipeline::phases::user_import_collector::collect::collect_user_imports;
use crate::pipeline::{compilation_unit::CompilationUnit, phases::compiler_phase::CompilerPhase};

pub struct UserImportCollector;

impl CompilerPhase for UserImportCollector {
    fn execute(&self, input: &mut CompilationUnit) {
        let user_imports = collect_user_imports(input.original_rust_module());
        input.set_user_imports(user_imports);
    }
}
