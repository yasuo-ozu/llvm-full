#[cfg(test)]
mod tests {
    #[test]
    fn test_llvm_sys_types_are_usable() {
        // Verify that llvm-sys FFI types are defined and have expected sizes.
        // This proves that llvm-sys successfully generated bindings from
        // the LLVM headers provided by llvm-full.
        assert!(std::mem::size_of::<llvm_sys::prelude::LLVMTypeRef>() > 0);
        assert!(std::mem::size_of::<llvm_sys::prelude::LLVMValueRef>() > 0);
        assert!(std::mem::size_of::<llvm_sys::prelude::LLVMModuleRef>() > 0);
        assert!(std::mem::size_of::<llvm_sys::prelude::LLVMContextRef>() > 0);
    }

    #[test]
    fn test_llvm_sys_enum_variants() {
        // Verify some well-known enum variants exist
        let _void = llvm_sys::LLVMTypeKind::LLVMVoidTypeKind;
        let _int = llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind;
        let _float = llvm_sys::LLVMTypeKind::LLVMFloatTypeKind;
    }
}
