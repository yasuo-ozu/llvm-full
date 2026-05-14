fn main() {
    // llvm-full (as a build-dependency) downloads and extracts LLVM.
    //
    // In CI, LLVM_SYS_XXX_PREFIX is already set by the yasuo-ozu/llvm-full action,
    // so llvm-sys's build script can find LLVM independently.
    //
    // For local development, set the appropriate LLVM_SYS_XXX_PREFIX manually
    // or ensure llvm-config is in PATH.
    if let Ok(root) = std::env::var("DEP_LLVM_FULL_ROOT") {
        eprintln!("llvm-full root: {root}");
    }
}
