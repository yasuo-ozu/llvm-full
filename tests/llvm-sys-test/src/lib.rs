#[cfg(feature = "llvm10-0")]
extern crate llvm_sys_100 as llvm_sys;
#[cfg(feature = "llvm11-0")]
extern crate llvm_sys_110 as llvm_sys;
#[cfg(feature = "llvm12-0")]
extern crate llvm_sys_120 as llvm_sys;
#[cfg(feature = "llvm13-0")]
extern crate llvm_sys_130 as llvm_sys;
#[cfg(feature = "llvm14-0")]
extern crate llvm_sys_140 as llvm_sys;
#[cfg(feature = "llvm15-0")]
extern crate llvm_sys_150 as llvm_sys;
#[cfg(feature = "llvm16-0")]
extern crate llvm_sys_160 as llvm_sys;
#[cfg(feature = "llvm17-0")]
extern crate llvm_sys_170 as llvm_sys;
#[cfg(feature = "llvm18-1")]
extern crate llvm_sys_181 as llvm_sys;
#[cfg(feature = "llvm19-1")]
extern crate llvm_sys_191 as llvm_sys;
#[cfg(feature = "llvm20-1")]
extern crate llvm_sys_201 as llvm_sys;
#[cfg(feature = "llvm21-1")]
extern crate llvm_sys_211 as llvm_sys;
#[cfg(feature = "llvm22-1")]
extern crate llvm_sys_221 as llvm_sys;

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
