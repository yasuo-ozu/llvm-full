use pyo3::prelude::*;

/// Returns the size of LLVMContextRef (pointer-sized) to verify bindings were generated.
#[pyfunction]
fn llvm_context_ref_size() -> usize {
    std::mem::size_of::<llvm_sys::prelude::LLVMContextRef>()
}

/// Returns the integer value of LLVMIntEQ to verify enum variants exist.
#[pyfunction]
fn llvm_int_eq_value() -> u32 {
    llvm_sys::LLVMIntPredicate::LLVMIntEQ as u32
}

#[pymodule]
fn pyo3_llvm_sys(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(llvm_context_ref_size, m)?)?;
    m.add_function(wrap_pyfunction!(llvm_int_eq_value, m)?)?;
    Ok(())
}
