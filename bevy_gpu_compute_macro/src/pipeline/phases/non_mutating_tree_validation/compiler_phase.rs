use crate::pipeline::{compilation_unit::CompilationUnit, phases::compiler_phase::CompilerPhase};

use super::validate_no_doc_comments::validate_no_doc_comments;
use super::validate_no_iter_pos_assignments::validate_no_iter_pos_assignments;

/// any sort of input validation that can be done on the original tree that doesn't require mutation
pub struct NonMutatingTreeValidation;

impl CompilerPhase for NonMutatingTreeValidation {
    fn execute(&self, input: &mut CompilationUnit) {
        validate_no_doc_comments(input.original_rust_module());
        validate_no_iter_pos_assignments(input.original_rust_module());
    }
}
