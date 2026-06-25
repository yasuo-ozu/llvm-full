// Minimal `llvm-sys` consumer used by build.yml's `smoke` job to verify a freshly
// released LLVM archive is actually usable from Rust the same way llvm-pm consumes
// it. Compiling exercises llvm-sys's build script (it must find LLVM via
// LLVM_SYS_<key>_PREFIX) and linking against the archive's libraries; running
// exercises the runtime libraries (e.g. macOS @rpath libunwind, Linux libtinfo).
use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMDisposeModule, LLVMModuleCreateWithNameInContext,
};
use std::ffi::CString;

fn main() {
    unsafe {
        let ctx = LLVMContextCreate();
        assert!(!ctx.is_null(), "LLVMContextCreate returned null");
        let name = CString::new("smoke").unwrap();
        let module = LLVMModuleCreateWithNameInContext(name.as_ptr(), ctx);
        assert!(!module.is_null(), "module creation returned null");
        LLVMDisposeModule(module);
        LLVMContextDispose(ctx);
    }
    println!("llvm-sys consumer OK");
}
